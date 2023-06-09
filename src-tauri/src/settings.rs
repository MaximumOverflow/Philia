use serde::{Deserialize, Serialize};
use tauri::api::path::download_dir;
use crate::context::GlobalContext;
use tauri::{AppHandle, Manager};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	pub dark_mode: bool,
	pub accent: String,

	pub tag_search_result_limit: u32,
	pub search_image_list_columns: u32,
	pub full_resolution_preview: bool,

	pub download_folder: PathBuf,
}

impl Settings {
	pub fn save(&self) -> Result<(), std::io::Error> {
		let json = serde_json::to_string_pretty(self).unwrap();
		std::fs::write("./settings.json", json)
	}
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			dark_mode: true,
			accent: "#ffb446".to_string(),

			tag_search_result_limit: 10,
			search_image_list_columns: 6,
			full_resolution_preview: false,

			download_folder: {
				fn get_local_download_dir() -> Result<PathBuf, std::io::Error> {
					std::fs::create_dir_all("./downloads")?;
					dunce::canonicalize("./downloads")
				}

				match get_local_download_dir() {
					Ok(path) => path,
					Err(_) => {
						let path = download_dir().expect("Missing download dir").join("Philia");
						std::fs::create_dir_all(&path).expect("Could not create download dir");
						path
					},
				}
			},
		}
	}
}

#[tauri::command]
pub async fn get_settings(handle: AppHandle) -> Settings {
	let state = handle.state::<GlobalContext>();
	let state = state.lock().unwrap();
	state.settings.clone()
}

#[tauri::command]
pub async fn set_settings(settings: Settings, handle: AppHandle) {
	let state = handle.state::<GlobalContext>();
	let mut state = state.lock().unwrap();
	let _ = settings.save();
	state.settings = settings;
}
