use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::api::path::download_dir;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TagSettings {
	pub remove_underscores: bool,
	pub escape_parentheses: bool,
	pub ignore_categories: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadSettings {
	pub apply_letterboxing: bool,
	pub download_folder: PathBuf,
}

impl Default for DownloadSettings {
	fn default() -> Self {
		Self {
			apply_letterboxing: false,
			download_folder: 'block: {
				let mut path = PathBuf::from("./Downloads");
				// if let Ok(_) = std::fs::create_dir_all(&path) {
				// 	break 'block path;
				// }

				path = download_dir().expect("Missing download dir").join("Philia");
				std::fs::create_dir_all(&path).expect("Could not create download dir");

				path
			},
		}
	}
}

pub type TagSettingsState = Mutex<TagSettings>;
pub type DownloadSettingsState = Mutex<DownloadSettings>;
