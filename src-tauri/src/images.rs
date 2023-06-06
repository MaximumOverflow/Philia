use crate::settings::SettingsState;
use tauri::{AppHandle, Manager, State};
use std::collections::HashMap;
use std::fs::File;
use philia::prelude::Post;
use std::path::PathBuf;
use std::sync::Mutex;
use png::Decoder;

pub type ImagesState = Mutex<HashMap<PathBuf, Post>>;

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
					let mut decoder = Decoder::new(file);
					let reader = decoder.read_info().ok()?;

					let metadata = reader
						.info()
						.utf8_text
						.iter()
						.find(|chunk| chunk.keyword == "post_metadata")?;

					let json = metadata.get_text().ok()?;
					let post = serde_json::from_str::<Post>(&json).ok()?;

					Some((path, post))
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
	images
		.iter()
		.map(|(k, v)| (k.to_str().unwrap().replace("\\", "/"), v.clone()))
		.collect()
}

#[tauri::command]
pub async fn refresh_images(handle: AppHandle) -> Vec<(String, Post)> {
	let images = handle.state();
	let settings = handle.state();
	refresh_images_impl(&images, &settings);
	get_images(handle).await
}
