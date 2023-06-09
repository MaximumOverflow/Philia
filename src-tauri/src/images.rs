use serde::{Deserialize, Serialize};
use crate::context::GlobalContext;
use cached::{Cached, SizedCache};
use image::imageops::FilterType;
use tauri::{AppHandle, Manager};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use philia::prelude::Post;
use itertools::Itertools;
use philia::data::Tags;
use image::ImageFormat;
use std::io::Cursor;
use base64::Engine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
	pub info: Post,
	pub file_path: PathBuf,
	pub preview_data: String,
}

#[tauri::command]
pub async fn get_images(handle: AppHandle) -> Vec<Image> {
	let context = handle.state::<GlobalContext>();
	let context = context.lock().unwrap();
	let mut images = context.images.values().cloned().collect_vec();
	images.sort_by(|a, b| a.file_path.cmp(&b.file_path));
	images
}

#[tauri::command]
pub async fn refresh_images(handle: AppHandle) -> Vec<Image> {
	let context = handle.state::<GlobalContext>();
	let mut context = context.lock().unwrap();
	context.refresh_images();

	let mut images = context.images.values().cloned().collect_vec();
	images.sort_by(|a, b| a.file_path.cmp(&b.file_path));
	images
}

#[tauri::command]
pub async fn get_image_tags(
	image_paths: Vec<PathBuf>, ignored_categories: Option<HashSet<String>>, handle: AppHandle,
) -> Vec<String> {
	let context = handle.state::<GlobalContext>();
	let context = context.lock().unwrap();
	let images = &context.images;

	let tags: HashSet<String> = image_paths
		.into_iter()
		.filter_map(|image| {
			let post = images.get(&image)?;

			if let Some(ignored_categories) = &ignored_categories {
				match &post.info.tags {
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
				let tags = post.info.tags.iter().map(str::to_string).collect_vec();
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
	let context = handle.state::<GlobalContext>();
	let context = context.lock().unwrap();
	let images = &context.images;

	let categories: HashSet<String> = image_paths
		.into_iter()
		.filter_map(|image| {
			let post = images.get(Path::new(&image))?;
			match &post.info.tags {
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

#[tauri::command]
pub async fn generate_image_preview(
	path: PathBuf, size: u32, handle: AppHandle,
) -> Result<String, String> {
	let cache = handle.state::<PreviewCache>();
	cache.get_or_generate_image_preview(path, size)
}

#[derive(Clone)]
pub struct PreviewCache {
	cache: Arc<Mutex<SizedCache<(PathBuf, u32), String>>>,
}

impl Default for PreviewCache {
	fn default() -> Self {
		Self {
			cache: Arc::new(Mutex::new(SizedCache::with_size(4096))),
		}
	}
}

impl PreviewCache {
	pub fn get_or_generate_image_preview(
		&self, path: PathBuf, size: u32,
	) -> Result<String, String> {
		let key = (path, size);

		{
			let mut cache = self.cache.lock().unwrap();
			if let Some(preview) = cache.cache_get(&key) {
				return Ok(preview.clone());
			}
		}

		// println!("Generating preview of size {} for {:?}...", key.1, key.0);
		let mut image = image::open(&key.0).map_err(|e| e.to_string())?;

		let mut buffer = vec![];
		image = image.resize(size, size, FilterType::Gaussian);
		image
			.write_to(&mut Cursor::new(&mut buffer), ImageFormat::Png)
			.map_err(|e| e.to_string())?;

		let mut data = String::from("data:image/png;base64,");
		base64::engine::general_purpose::STANDARD.encode_string(&buffer, &mut data);

		{
			let mut cache = self.cache.lock().unwrap();
			cache.cache_set(key, data.clone());
		}

		Ok(data)
	}
}
