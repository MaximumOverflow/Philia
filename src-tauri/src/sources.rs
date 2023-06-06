use philia::prelude::{Order, Post, Source, Client};
use tauri::{AppHandle, Manager, State};
use std::collections::HashMap;
use std::sync::Mutex;
use std::path::Path;

type SourcesState = Mutex<HashMap<String, Client>>;

#[tauri::command]
pub async fn get_available_sources(handle: AppHandle) -> Vec<Source> {
	let (state, no_fetch) = get_sources_state(&handle).await;
	
	if !no_fetch {
		let sources = fetch_sources().await;
		let mut state = state.lock().unwrap();
		*state = sources;
	}

	let state = state.lock().unwrap();
	let mut sources: Vec<_> = state.values().map(|v| v.source().clone()).collect();
	sources.sort_by(|a, b| a.name.cmp(&b.name));
	sources
}

#[tauri::command]
pub async fn get_source_tags(source: String) -> Option<Vec<String>> {
	let file = std::fs::read(Path::new("./Cache").join(format!("{source}_tags.json"))).ok()?;
	serde_json::from_slice(&file).ok()
}

#[tauri::command]
pub async fn fetch_source_tags(source: String, handle: AppHandle) -> Result<Vec<String>, String> {
	let source = {
		let (state, _) = get_sources_state(&handle).await;
		let state = state.lock().unwrap();

		let Some(source) = state.get(&source) else {
			return Err("Source not found".into());
		};

		source.clone()
	};
	
	let mut all_tags = vec![];
	for i in 1..25 {
		let tags = source.tags_async(i, 1000).await.map_err(|e| e.to_string())?;
		if tags.is_empty() { break; }
		all_tags.extend(tags.into_iter().map(|tag| tag.name));
		let _ = handle.emit_all("fetch_source_tags_count", all_tags.len());
	}
	
	Ok(all_tags)
}

#[tauri::command]
pub async fn search(source: String, page: usize, limit: usize, order: Order, tags: Vec<String>, handle: AppHandle) -> Result<Vec<Post>, String> {
	let source = {
		let (state, _) = get_sources_state(&handle).await;
		let state = state.lock().unwrap();

		let Some(source) = state.get(&source) else {
			return Err("Source not found".into());
		};
		
		source.clone()
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

	source
		.search_async(page, limit, order, include.into_iter(), exclude.into_iter())
		.await
}

pub async fn get_sources_state(handle: &AppHandle) -> (State<'_, SourcesState>, bool) {
	match handle.try_state::<SourcesState>() {
		Some(state) => (state, false),
		None => {
			let sources = fetch_sources().await;
			handle.manage(SourcesState::new(sources));
			(handle.state(), true)
		}
	}
}

async fn fetch_sources() -> HashMap<String, Client> {
	println!("Fetching sources...");
	
	let _ = std::fs::create_dir_all("./Cache");
	let _ = std::fs::create_dir_all("./Sources");
	let Ok(entries) = std::fs::read_dir("./Sources") else {
		return Default::default();
	};

	HashMap::from_iter(entries.filter_map(|entry| match entry {
		Err(_) => None,
		Ok(entry) => {
			let path = entry.path();
			let Ok(json) = std::fs::read(&path) else {
				eprintln!("Could not read source {path:?}");
				return None;
			};

			let Ok(source) = serde_json::from_slice::<Source>(&json) else {
				eprintln!("Could not deserialize source {path:?}");
				return None;
			};

			Some((source.name.clone(), Client::new(source)))
		},
	}))
}