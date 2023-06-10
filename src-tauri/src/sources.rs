use philia::prelude::{SearchOrder, Post, Client, TagOrder};
use philia::source::{FeatureFlags, ScriptableSource};
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::context::GlobalContext;
use tauri::{AppHandle, Manager};
use std::cmp::Ordering;
use std::path::Path;

#[derive(Clone, Serialize, Deserialize)]
pub struct SourceInfo {
	name: String,
	search: bool,
	tag_list: bool,
}

#[tauri::command]
pub async fn get_available_sources(handle: AppHandle) -> Vec<SourceInfo> {
	let context = handle.state::<GlobalContext>();
	let context = context.lock().unwrap();
	let mut sources: Vec<_> = context
		.sources
		.iter()
		.map(|(name, client)| {
			let flags = client.source().feature_flags();
			SourceInfo {
				name: name.clone(),
				search: (flags & FeatureFlags::SEARCH) != FeatureFlags::NONE,
				tag_list: (flags & FeatureFlags::TAG_LIST) != FeatureFlags::NONE,
			}
		})
		.collect();

	sources.sort_by(|a, b| a.name.cmp(&b.name));
	sources
}

#[tauri::command]
pub async fn get_source_tags(source: String, handle: AppHandle) -> Option<Vec<String>> {
	let context = handle.state::<GlobalContext>();
	let context = context.lock().unwrap();
	let tags = context.source_tags.get(&source)?;

	match tags {
		None => None,
		Some(tags) => {
			let mut vec = Vec::from_iter(tags.iter().cloned());
			vec.sort_by(sort_tags);
			Some(vec)
		},
	}
}

#[tauri::command]
pub async fn fetch_source_tags(source: String, handle: AppHandle) -> Result<Vec<String>, String> {
	let client = {
		let context = handle.state::<GlobalContext>();
		let context = context.lock().unwrap();
		let Some(client) = context.sources.get(&source) else {
			return Err("Source not found".into());
		};

		client.clone()
	};

	let mut all_tags = vec![];
	for i in 1..25 {
		let tags = client
			.get_tags_async(i, 1000, TagOrder::Count)
			.await
			.map_err(|e| e.to_string())?;
		if tags.is_empty() {
			break;
		}
		all_tags.extend(tags.into_iter().map(|tag| tag.name));
		let _ = handle.emit_all("fetch_source_tags_count", all_tags.len());
	}

	let context = handle.state::<GlobalContext>();
	let mut context = context.lock().unwrap();
	let tags = context.source_tags.get_mut(&source).unwrap();

	*tags = Some(HashSet::from_iter(all_tags.iter().cloned()));

	all_tags.sort_by(sort_tags);
	Ok(all_tags)
}

#[tauri::command]
pub async fn search(
	source: String, page: u32, limit: u32, order: SearchOrder, tags: Vec<String>, handle: AppHandle,
) -> Result<(Vec<Post>, Vec<String>), String> {
	let client = {
		let context = handle.state::<GlobalContext>();
		let context = context.lock().unwrap();
		let Some(client) = context.sources.get(&source) else {
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

	let posts = client
		.search_async(page, limit, order, include.into_iter(), exclude.into_iter())
		.await
		.map_err(|e| e.to_string())?;

	let context = handle.state::<GlobalContext>();
	let mut context = context.lock().unwrap();
	let Some(tags) = context.source_tags.get_mut(&source) else {
		return Ok((posts, vec![]));
	};

	let tags = match tags {
		Some(tags) => tags,
		None => tags.insert(HashSet::default()),
	};

	tags.extend(posts.iter().map(|p| p.tags.iter().map(str::to_string)).flatten());
	let mut tags = Vec::from_iter(tags.iter().cloned());
	tags.sort_by(sort_tags);
	Ok((posts, tags))
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
