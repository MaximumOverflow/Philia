use tauri::{AppHandle, Manager, State};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageFormat};
use image::imageops::FilterType;
use itertools::Itertools;
use philia::prelude::{Post, Tags};
use crate::images::get_images_state;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dataset {
	name: String,
	#[serde(default = "Default::default")]
	images: HashSet<String>,
	#[serde(default = "Default::default")]
	thumbnail: Option<PathBuf>,
	#[serde(default = "Default::default")]
	settings: Settings,
}

impl Dataset {
	pub fn new(name: String) -> Self {
		Self {
			name,
			images: Default::default(),
			settings: Default::default(),
			thumbnail: Default::default(),
		}
	}
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	#[serde(default = "Default::default")]
	tags: TagSettings,
	#[serde(default = "Default::default")]
	image: ImageSettings,
	#[serde(default = "Default::default")]
	training: TrainingSettings,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TagSettings {
	#[serde(default = "Default::default")]
	pub remove_underscores: bool,
	#[serde(default = "Default::default")]
	pub escape_parentheses: bool,
	#[serde(default = "Default::default")]
	pub ignore_categories: HashSet<String>,
	#[serde(default = "Default::default")]
	pub ignore_tags: HashSet<String>,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ImageSettings {
	#[serde(default = "Default::default")]
	pub apply_letterboxing: bool,
	#[serde(default = "Default::default")]
	pub resize: (u32, u32),
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSettings {
	#[serde(default = "Default::default")]
	pub keyword: String,
	#[serde(default = "Default::default")]
	pub repetitions: u32,
}

pub type DatasetsState = Mutex<Vec<Dataset>>;

fn get_dataset_state(handle: &AppHandle) -> State<'_, DatasetsState> {
	match handle.try_state() {
		Some(datasets) => datasets,
		None => {
			let datasets = 'block: {
				let Ok(json) = std::fs::read("datasets.json") else {
					break 'block DatasetsState::default();
				};

				let Ok(datasets) = serde_json::from_slice(&json) else {
					break 'block DatasetsState::default();
				};

				DatasetsState::new(datasets)
			};

			handle.manage(datasets);
			handle.state::<DatasetsState>()
		},
	}
}

#[tauri::command]
pub async fn get_datasets(handle: AppHandle) -> Vec<Dataset> {
	let datasets = get_dataset_state(&handle);
	let datasets = datasets.lock().unwrap();
	datasets.clone()
}

#[tauri::command]
pub async fn new_dataset(handle: AppHandle) -> Vec<Dataset> {
	let datasets = get_dataset_state(&handle);
	let mut datasets = datasets.lock().unwrap();
	datasets.push(Dataset::new("New Dataset".into()));
	save_datasets(&datasets);
	datasets.clone()
}

#[tauri::command]
pub async fn del_dataset(index: usize, handle: AppHandle) -> Vec<Dataset> {
	let datasets = get_dataset_state(&handle);
	let mut datasets = datasets.lock().unwrap();
	if index < datasets.len() {
		datasets.remove(index);
	}
	save_datasets(&datasets);
	datasets.clone()
}

#[tauri::command]
pub async fn set_dataset(index: usize, dataset: Dataset, handle: AppHandle) -> Vec<Dataset> {
	let datasets = get_dataset_state(&handle);
	let mut datasets = datasets.lock().unwrap();
	datasets[index] = dataset;
	save_datasets(&datasets);
	datasets.clone()
}

#[tauri::command]
pub async fn export_dataset(index: usize, path: PathBuf, handle: AppHandle) -> Result<(), String> {
	if !path.exists() {
		return Err("Path does not exist".into());
	}

	let datasets = get_dataset_state(&handle);
	let datasets = datasets.lock().unwrap();
	let Some(dataset) = datasets.get(index) else {
		return Err("Invalid dataset index".into());
	};

	let path = path.join(format! {
		"{}_{}",
		dataset.settings.training.repetitions,
		dataset.settings.training.keyword,
	});

	std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;

	let images = get_images_state(&handle);
	let images = images.lock().unwrap();
	let images = &*images;

	for (image_path, post) in dataset.images.iter().map(|i| (Path::new(i), images.get(i))) {
		let Some(post) = post else { continue };
		let Some(file_stem) = image_path.file_stem() else { continue; };
		let Some(file_name) = image_path.file_name() else { continue; };

		let image_destination = path.as_path().join(file_name);
		let mut image = image::open(image_path).map_err(|e| e.to_string())?;

		if dataset.settings.image.apply_letterboxing {
			image = apply_letterboxing(&image);
		}

		match dataset.settings.image.resize {
			(0, 0) => {},
			(width, 0) => image = image.resize(width, image.height(), FilterType::Lanczos3),
			(0, height) => image = image.resize(image.width(), height, FilterType::Lanczos3),
			(width, height) => image = image.resize_exact(width, height, FilterType::Lanczos3),
		}

		image
			.save_with_format(image_destination, ImageFormat::Png)
			.map_err(|e| e.to_string())?;

		let tags = get_tag_string(post, &dataset.settings.tags);
		let mut tags_destination = path.as_path().join(file_stem);
		tags_destination.set_extension("json");
		std::fs::write(tags_destination, tags).map_err(|e| e.to_string())?;
	}

	Ok(())
}

fn save_datasets(datasets: &[Dataset]) {
	let json = serde_json::to_string_pretty(datasets).unwrap();
	std::fs::write("datasets.json", json).unwrap();
}

pub fn apply_letterboxing(image: &DynamicImage) -> DynamicImage {
	let dimensions = image.width().max(image.height());
	let mut output = ImageBuffer::from_pixel(dimensions, dimensions, [0, 0, 0, 255].into());

	let x_offset = (dimensions - image.width()) / 2;
	let y_offset = (dimensions - image.height()) / 2;
	for (x, y, p) in image.pixels() {
		unsafe {
			output.unsafe_put_pixel(x + x_offset, y + y_offset, p);
		}
	}

	DynamicImage::from(output)
}

pub fn get_tag_string(post: &Post, settings: &TagSettings) -> String {
	let tags = match &post.tags {
		Tags::All(tags) => tags.iter().join(", "),
		Tags::Categorized(cats) => cats
			.iter()
			.filter(|(category, _)| {
				!settings.ignore_categories.contains(category.to_lowercase().as_str())
			})
			.flat_map(|(_, tags)| tags)
			.filter(|tag| !settings.ignore_tags.contains(*tag))
			.join(", "),
	};

	let tags = match settings.remove_underscores {
		false => tags,
		true => tags.replace('_', " "),
	};

	match settings.escape_parentheses {
		false => tags,
		true => tags.replace('(', "\\(").replace(')', "\\)"),
	}
}
