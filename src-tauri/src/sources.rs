use philia::prelude::{SearchOrder, Post, Client, TagOrder};
use std::collections::{HashMap, HashSet};
use tauri::{AppHandle, Manager, State};
use philia::source::{FeatureFlags, ScriptableSource};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::sync::Mutex;
use std::path::Path;

type SourcesState = Mutex<HashMap<String, (Client, Option<HashSet<String>>)>>;

#[derive(Clone, Serialize, Deserialize)]
pub struct SourceInfo {
	name: String,
	search: bool,
	tag_list: bool,
}

#[tauri::command]
pub async fn get_available_sources(handle: AppHandle) -> Vec<SourceInfo> {
	let (state, no_fetch) = get_sources_state(&handle);
	
	if !no_fetch {
		let sources = fetch_sources();
		let mut state = state.lock().unwrap();
		*state = sources;
	}

	let state = state.lock().unwrap();
	let mut sources: Vec<_> = state.iter().map(|(name, (client, _))| {
		let flags = client.source().feature_flags();
		SourceInfo {
			name: name.clone(),
			search: (flags & FeatureFlags::SEARCH) != FeatureFlags::NONE,
			tag_list: (flags & FeatureFlags::TAG_LIST) != FeatureFlags::NONE,
		}
	}).collect();
	
	sources.sort_by(|a, b| a.name.cmp(&b.name));
	sources
}

#[tauri::command]
pub async fn get_source_tags(source: String, handle: AppHandle) -> Option<Vec<String>> {
	let (state, _) = get_sources_state(&handle);
	let state = state.lock().unwrap();
	let (_, tags) = state.get(&source)?;
	
	match tags {
		None => None,
		Some(tags) => {
			let mut vec = Vec::from_iter(tags.iter().cloned());
			vec.sort_by(sort_tags);
			Some(vec)
		}
	}
}

#[tauri::command]
pub async fn fetch_source_tags(source: String, handle: AppHandle) -> Result<Vec<String>, String> {
	let client = {
		let (state, _) = get_sources_state(&handle);
		let state = state.lock().unwrap();

		let Some((client, _)) = state.get(&source) else {
			return Err("Source not found".into());
		};
		
		client.clone()
	};
	
	let mut all_tags = vec![];
	for i in 1..25 {
		let tags = client.get_tags_async(i, 1000, TagOrder::Count).await.map_err(|e| e.to_string())?;
		if tags.is_empty() { break; }
		all_tags.extend(tags.into_iter().map(|tag| tag.name));
		let _ = handle.emit_all("fetch_source_tags_count", all_tags.len());
	}

	let (state, _) = get_sources_state(&handle);
	let mut state = state.lock().unwrap();
	let (_, tags) = state.get_mut(&source).unwrap();
	
	*tags = Some(HashSet::from_iter(all_tags.iter().cloned()));
	
	all_tags.sort_by(sort_tags);
	Ok(all_tags)
}

#[tauri::command]
pub async fn search(source: String, page: u32, limit: u32, order: SearchOrder, tags: Vec<String>, handle: AppHandle) -> Result<Vec<Post>, String> {
	let client = {
		let (state, _) = get_sources_state(&handle);
		let state = state.lock().unwrap();

		let Some((client, _)) = state.get(&source) else {
			return Err("Source not found".into());
		};

		client.clone()
	};

	let mut include = vec![];
	let mut exclude = vec![];
	for tag in tags.into_iter() {
		if tag.starts_with('-') {
			exclude.push((&tag[1..]).to_string());
		} else {
			include.push(tag);
		}
	}

	client.search_async(page, limit, order, include.into_iter(), exclude.into_iter()).await.map_err(|e| e.to_string())
}

pub fn get_sources_state(handle: &AppHandle) -> (State<'_, SourcesState>, bool) {
	match handle.try_state::<SourcesState>() {
		Some(state) => (state, false),
		None => {
			let sources = fetch_sources();
			handle.manage(SourcesState::new(sources));
			(handle.state(), true)
		}
	}
}

fn fetch_sources() -> HashMap<String, (Client, Option<HashSet<String>>)> {
	println!("Fetching sources...");
	
	let _ = std::fs::create_dir_all("./cache");
	let _ = std::fs::create_dir_all("./sources");
	let Ok(entries) = std::fs::read_dir("./sources") else {
		return Default::default();
	};

	HashMap::from_iter(entries.filter_map(|entry| match entry {
		Err(_) => None,
		Ok(entry) => {
			let path = entry.path();
			
			let Some(extension) = path.extension() else {
				return None;
			};
			
			if extension != "rhai" {
				return None;
			}
			
			let name = path.file_stem().unwrap().to_string_lossy().to_string();
			let Ok(code) = std::fs::read_to_string(&path) else {
				eprintln!("Could not read source {path:?}");
				return None;
			};
			
			let Ok(source) = ScriptableSource::new(&name, &code) else {
				eprintln!("Could not compile source {path:?}");
				return None;
			};
			
			let tags = 'tags: {
				let Ok(file) = std::fs::read(Path::new("./cache").join(format!("{}_tags.json", name))) else {
					break 'tags Default::default();
				};
				
				serde_json::from_slice(&file).unwrap_or_default()
			};

			Some((name.clone(), (Client::new(source), tags)))
		},
	}))
}

fn sort_tags(a: &String, b: &String) -> Ordering {
	let a = match a.chars().next().unwrap_or_default().is_alphabetic() {
		true => a.as_str(),
		false => "z",
	};

	let b = match b.chars().next().unwrap_or_default().is_alphabetic() {
		true => b.as_str(),
		false => "z",
	};

	a.cmp(b)
}