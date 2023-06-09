use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, ImageFormat};
use serde::{Deserialize, Serialize};
use crate::context::GlobalContext;
use philia::prelude::{Post, Tags};
use image::imageops::FilterType;
use tauri::{AppHandle, Manager};
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use itertools::Itertools;

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
	#[serde(default = "Default::default")]
	pub target_format: TargetImageFormat,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum TargetImageFormat {
	#[default]
	Png, 
	Jpg, 
	Bmp,
	Gif, 
	Qoi,
	WebP,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSettings {
	#[serde(default = "Default::default")]
	pub keyword: String,
	#[serde(default = "Default::default")]
	pub repetitions: u32,
}

#[tauri::command]
pub async fn get_datasets(handle: AppHandle) -> Vec<Dataset> {
	let context = handle.state::<GlobalContext>();
	let context = context.lock().unwrap();
	context.datasets.clone()
}

#[tauri::command]
pub async fn new_dataset(handle: AppHandle) -> Vec<Dataset> {
	let context = handle.state::<GlobalContext>();
	let mut context = context.lock().unwrap();
	context.datasets.push(Dataset::new("New Dataset".into()));
	save_datasets(&context.datasets);
	context.datasets.clone()
}

#[tauri::command]
pub async fn del_dataset(index: usize, handle: AppHandle) -> Vec<Dataset> {
	let context = handle.state::<GlobalContext>();
	let mut context = context.lock().unwrap();
	let datasets = &mut context.datasets;
	if index < datasets.len() {
		datasets.remove(index);
	}
	save_datasets(&datasets);
	datasets.clone()
}

#[tauri::command]
pub async fn set_dataset(index: usize, dataset: Dataset, handle: AppHandle) -> Vec<Dataset> {
	let context = handle.state::<GlobalContext>();
	let mut context = context.lock().unwrap();
	let datasets = &mut context.datasets;
	datasets[index] = dataset;
	save_datasets(&datasets);
	datasets.clone()
}

#[tauri::command]
pub async fn export_dataset(index: usize, path: PathBuf, handle: AppHandle) -> Result<(), String> {
	if !path.exists() {
		return Err("Path does not exist".into());
	}

	let context = handle.state::<GlobalContext>();
	let context = context.lock().unwrap();

	let Some(dataset) = context.datasets.get(index) else {
		return Err("Invalid dataset index".into());
	};

	let path = path.join(format! {
		"{}_{}",
		dataset.settings.training.repetitions,
		dataset.settings.training.keyword,
	});

	std::fs::create_dir_all(&path).map_err(|e| e.to_string())?;
	
	dataset.images.par_iter().filter_map(|i| context.images.get(Path::new(i))).for_each(|post| {
		let Some(file_stem) = post.file_path.file_stem() else { return };
		let mut image = match image::open(&post.file_path) {
			Ok(image) => image,
			Err(err) => {
				eprintln!("{:?}", err);
				return;
			}
		};

		if dataset.settings.image.apply_letterboxing {
			image = apply_letterboxing(&image);
		}

		match dataset.settings.image.resize {
			(0, 0) => {},
			(width, 0) => image = image.resize(width, image.height(), FilterType::Lanczos3),
			(0, height) => image = image.resize(image.width(), height, FilterType::Lanczos3),
			(width, height) => image = image.resize_exact(width, height, FilterType::Lanczos3),
		}

		let (target_format, extension) = match dataset.settings.image.target_format {
			TargetImageFormat::Png =>  (ImageFormat::Png, "png"),
			TargetImageFormat::Bmp =>  (ImageFormat::Bmp, "bmp"),
			TargetImageFormat::Gif =>  (ImageFormat::Gif, "gif"),
			TargetImageFormat::Qoi =>  (ImageFormat::Qoi, "qoi"),
			TargetImageFormat::Jpg =>  (ImageFormat::Jpeg, "jpeg"),
			TargetImageFormat::WebP => (ImageFormat::WebP, "webp"),
		};

		let mut image_destination = path.as_path().join(file_stem);
		image_destination.set_extension(extension);

		if let Err(err) = image.save_with_format(image_destination, target_format) {
			eprintln!("{:?}", err);
			return;
		}

		let tags = get_tag_string(&post.info, &dataset.settings.tags);
		let mut tags_destination = path.as_path().join(file_stem);
		tags_destination.set_extension("json");

		if let Err(err) = std::fs::write(tags_destination, tags) {
			eprintln!("{:?}", err);
			return;
		}
	});

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
