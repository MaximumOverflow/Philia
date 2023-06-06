use serde::{Deserialize, Serialize};
use tauri::api::path::download_dir;
use tauri::{AppHandle, Manager};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	pub download_folder: PathBuf,
}

impl Settings {
	pub fn load() -> Option<Self> {
		let json = std::fs::read("settings.json").ok()?;
		serde_json::from_slice(&json).ok()
	}

	pub fn save(&self) -> Result<(), std::io::Error> {
		let json = serde_json::to_string_pretty(self).unwrap();
		std::fs::write("settings.json", json)
	}
}

impl Default for Settings {
	fn default() -> Self {
		Self {
			download_folder: 'block: {
				let mut path = PathBuf::from("./Downloads");
				if let Ok(_) = std::fs::create_dir_all(&path) {
					break 'block path;
				}

				path = download_dir().expect("Missing download dir").join("Philia");
				std::fs::create_dir_all(&path).expect("Could not create download dir");

				path
			},
		}
	}
}

#[tauri::command]
pub async fn get_download_folder(handle: AppHandle) -> String {
	let state = handle.state::<SettingsState>();
	let state = state.lock().unwrap();
	state.download_folder.to_string_lossy().to_string()
}

#[tauri::command]
pub async fn set_download_folder(folder: PathBuf, handle: AppHandle) {
	let state = handle.state::<SettingsState>();
	let mut state = state.lock().unwrap();
	state.download_folder = folder;
	let _ = state.save();
}

pub type SettingsState = Mutex<Settings>;
