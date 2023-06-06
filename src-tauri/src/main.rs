#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use crate::settings::{Settings, SettingsState};

mod sources;
mod download;
mod settings;
mod datasets;
mod images;

fn main() {
	if let Ok(value) = std::env::var("PHILIA_WORK_DIR") {
		std::env::set_current_dir(value).expect("Invalid work directory.");
	}

	tauri::Builder::default()
		.manage(SettingsState::new(Settings::load().unwrap_or_default()))
		.invoke_handler(tauri::generate_handler![
			sources::get_available_sources,
			sources::fetch_source_tags,
			sources::get_source_tags,
			sources::search,
			download::download_posts,
			datasets::get_datasets,
			datasets::set_dataset,
			datasets::new_dataset,
			datasets::del_dataset,
			datasets::export_dataset,
			images::get_images,
			images::refresh_images,
			images::get_image_tags,
			images::get_image_categories,
			settings::get_settings,
			settings::set_settings,
		])
		.setup(|_handle| {
			#[cfg(debug_assertions)]
			{
				use tauri::Manager;
				let window = _handle.get_window("main").unwrap();
				window.open_devtools();
			}

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
