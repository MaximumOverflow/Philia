#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::Manager;
use crate::settings::{Settings, SettingsState};

mod sources;
mod download;
mod settings;
mod datasets;
mod images;

fn main() {
	#[cfg(debug_assertions)]
	if std::env::var("TAURI_DEBUG").is_ok() {
		std::env::set_current_dir("../work_dir/").unwrap();
	}

	tauri::Builder::default()
		.manage(SettingsState::new(Settings::load().unwrap_or_default()))
		.invoke_handler(tauri::generate_handler![
			sources::get_available_sources,
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
