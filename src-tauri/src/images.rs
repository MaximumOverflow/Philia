use crate::settings::SettingsState;
use tauri::{AppHandle, Manager, State};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::ops::Deref;
use philia::prelude::Post;
use std::sync::Mutex;
use itertools::Itertools;
use philia::data::Tags;
use png::Decoder;

pub type ImagesState = Mutex<HashMap<String, Post>>;

pub fn get_images_state(handle: &AppHandle) -> State<'_, ImagesState> {
	match handle.try_state() {
		Some(datasets) => datasets,
		None => {
			handle.manage(ImagesState::new(HashMap::default()));

			let images = handle.state();
			let settings = handle.state();
			refresh_images_impl(&images, &settings);

			images
		},
	}
}

pub fn refresh_images_impl(
	state: &State<'_, ImagesState>, download_settings: &State<'_, SettingsState>,
) {
	let images = 'block: {
		let download_settings = download_settings.lock().unwrap();

		let Ok(read_dir) = std::fs::read_dir(&download_settings.download_folder) else {
			break 'block HashMap::default();
		};

		let images = read_dir
			.filter_map(|entry| match entry {
				Err(_) => None,
				Ok(entry) => {
					let path = entry.path();
					let file = File::open(&path).ok()?;
					let decoder = Decoder::new(file);
					let reader = decoder.read_info().ok()?;

					let metadata = reader
						.info()
						.utf8_text
						.iter()
						.find(|chunk| chunk.keyword == "post_metadata")?;

					let json = metadata.get_text().ok()?;
					let post = serde_json::from_str::<Post>(&json).ok()?;

					let key = path.to_string_lossy().replace('\\', "/");
					Some((key, post))
				},
			})
			.collect();

		images
	};

	let mut state = state.lock().unwrap();
	*state = images;
}

#[tauri::command]
pub async fn get_images(handle: AppHandle) -> Vec<(String, Post)> {
	let images = get_images_state(&handle);
	let images = images.lock().unwrap();
	images.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
}

#[tauri::command]
pub async fn refresh_images(handle: AppHandle) -> Vec<(String, Post)> {
	let images = handle.state();
	let settings = handle.state();
	refresh_images_impl(&images, &settings);
	get_images(handle).await
}

#[tauri::command]
pub async fn get_image_tags(
	image_paths: Vec<String>, ignored_categories: Option<HashSet<String>>, handle: AppHandle,
) -> Vec<String> {
	let images = get_images_state(&handle);
	let images = images.lock().unwrap();
	let images = images.deref();

	let tags: HashSet<String> = image_paths
		.into_iter()
		.filter_map(|image| {
			let post = images.get(&image)?;

			if let Some(ignored_categories) = &ignored_categories {
				match &post.tags {
					Tags::All(a) => Some(a.clone()),
					Tags::Categorized(c) => {
						let mut tags = vec![];
						for (key, value) in c.iter() {
							if !ignored_categories.contains(key) {
								tags.extend_from_slice(&value);
							}
						}

						Some(tags)
					},
				}
			} else {
				let tags = post.tags.iter().map(str::to_string).collect_vec();
				Some(tags)
			}
		})
		.flatten()
		.collect();

	let mut tags = Vec::from_iter(tags);
	tags.sort();
	tags
}

#[tauri::command]
pub async fn get_image_categories(image_paths: Vec<String>, handle: AppHandle) -> Vec<String> {
	let images = get_images_state(&handle);
	let images = images.lock().unwrap();
	let images = images.deref();

	let categories: HashSet<String> = image_paths
		.into_iter()
		.filter_map(|image| {
			let post = images.get(&image)?;
			match &post.tags {
				Tags::All(_) => None,
				Tags::Categorized(c) => Some(c.keys().cloned()),
			}
		})
		.flatten()
		.collect();

	let mut categories = Vec::from_iter(categories);
	categories.sort();
	categories
}
