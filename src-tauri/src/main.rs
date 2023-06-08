#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use crate::context::{Context, GlobalContext};
use crate::images::PreviewCache;

mod sources;
mod download;
mod settings;
mod datasets;
mod images;
mod context;

fn main() {
	if let Ok(value) = std::env::var("PHILIA_WORK_DIR") {
		std::env::set_current_dir(value).expect("Invalid work directory.");
	}

	let preview_cache = PreviewCache::default();

	tauri::Builder::default()
		.manage(preview_cache.clone())
		.manage(GlobalContext::new(Context::load_or_default(preview_cache)))
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
			images::generate_image_preview,
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
