use philia::prelude::{Order, Post, Source, Client};
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::path::Path;

lazy_static! {
	pub static ref SOURCES: HashMap<String, Client> = 'block: {
		let Ok(entries) = std::fs::read_dir("./Sources") else {
			break 'block Default::default();
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
	};
}

#[tauri::command]
pub async fn get_available_sources() -> Vec<String> {
	let mut sources: Vec<_> = SOURCES.keys().cloned().collect();
	sources.sort();
	sources
}

#[tauri::command]
pub async fn get_source_tags(source: String) -> Option<Vec<String>> {
	let file = std::fs::read(Path::new("./cache").join(format!("{source}_tags.json"))).ok()?;
	serde_json::from_slice(&file).ok()
}

#[tauri::command]
pub async fn search(
	source: String, page: usize, limit: usize, order: Order, tags: Vec<String>,
) -> Result<Vec<Post>, String> {
	let Some(source) = SOURCES.get(&source) else {
		return Err("Source not found".into());
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
