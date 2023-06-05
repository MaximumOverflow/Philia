#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::Manager;
use crate::settings::{DownloadSettingsState, TagSettingsState};

mod sources;
mod download;
mod settings;

fn main() {
	tauri::Builder::default()
		.manage(TagSettingsState::default())
		.manage(DownloadSettingsState::default())
		.invoke_handler(tauri::generate_handler![
			sources::get_available_sources,
			sources::get_source_tags,
			sources::search,
			download::download_posts,
		])
		.setup(|handle| {
			#[cfg(debug_assertions)]
			{
				let window = handle.get_window("main").unwrap();
				window.open_devtools();
			}

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
