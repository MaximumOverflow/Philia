#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::time::{Duration, SystemTime};
use tauri::{AppHandle, command, Manager, WindowBuilder, WindowUrl};
use crate::context::{Context, GlobalContext};
use crate::images::PreviewCache;

mod sources;
mod download;
mod settings;
mod datasets;
mod images;
mod context;

#[command]
async fn initialize(app: AppHandle) {
	let now = SystemTime::now();
	let preview_cache = PreviewCache::default();
	app.manage(preview_cache.clone());
	app.manage(GlobalContext::new(Context::load_or_default(preview_cache)));
	
	if let Some(sleep) = Duration::from_secs(1).checked_sub(now.elapsed().unwrap()) {
		tokio::time::sleep(sleep).await;
	}
	
	let _window = WindowBuilder::new(&app, "main", WindowUrl::App("index.html".into()))
		.inner_size(1280.0, 720.0)
		.min_inner_size(900.0, 600.0)
		.resizable(true)
		.center()
		.title("Philia")
		.visible(true)
		.build()
		.unwrap();

	#[cfg(debug_assertions)]
	_window.open_devtools();
	app.get_window("splashscreen").unwrap().close().unwrap();
	println!("Initialization complete")
}

fn main() {
	if let Ok(value) = std::env::var("PHILIA_WORK_DIR") {
		std::env::set_current_dir(value).expect("Invalid work directory.");
	}
	
	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![
			initialize,
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
				let window = _handle.get_window("splashscreen").unwrap();
				window.open_devtools();
			}

			Ok(())
		})
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
