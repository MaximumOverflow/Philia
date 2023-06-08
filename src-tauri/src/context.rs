use rayon::iter::{ParallelIterator, IntoParallelRefIterator};
use crate::images::{Image, PreviewCache};
use philia::source::ScriptableSource;
use philia::prelude::{Client, Post};
use fxhash::{FxHashMap, FxHashSet};
use std::path::{Path, PathBuf};
use crate::settings::Settings;
use crate::datasets::Dataset;
use std::time::SystemTime;
use itertools::Itertools;
use std::sync::Mutex;
use std::fs::File;
use png::Decoder;

pub struct Context {
	pub settings: Settings,
	pub datasets: Vec<Dataset>,
	pub images: FxHashMap<PathBuf, Image>,
	pub sources: FxHashMap<String, Client>,
	pub source_tags: FxHashMap<String, Option<FxHashSet<String>>>,
	preview_cache: PreviewCache,
}

pub type GlobalContext = Mutex<Context>;

impl Context {
	pub fn load_or_default(preview_cache: PreviewCache) -> Self {
		let mut ctx = Self {
			settings: Default::default(),
			datasets: vec![],
			images: Default::default(),
			sources: Default::default(),
			source_tags: Default::default(),
			preview_cache,
		};

		ctx.refresh_settings();
		ctx.refresh_datasets();
		ctx.refresh_sources();
		ctx.refresh_images();
		ctx
	}

	pub fn refresh_sources(&mut self) {
		let _ = std::fs::create_dir_all("./cache");
		let _ = std::fs::create_dir_all("./sources");
		let Ok(entries) = std::fs::read_dir("./sources") else {
			eprintln!("Could not read 'sources' directory.");
			return;
		};

		self.sources.clear();
		self.source_tags.clear();
		for entry in entries.filter_map(Result::ok) {
			let path = entry.path();
			if path.extension().map(|s| s.to_str()) != Some(Some("rhai")) {
				eprintln!("Could not read source {path:?}");
				continue;
			}

			let Ok(code) = std::fs::read_to_string(&path) else {
				eprintln!("Could not read source {path:?}");
				continue;
			};

			let Some(name) = path.file_stem().map(|s| s.to_string_lossy().to_string()) else {
				continue;
			};

			let source = match ScriptableSource::new(&name, &code) {
				Ok(source) => source,
				Err(err) => {
					eprintln!("Could not compile source {path:?}: {err:?}");
					continue;
				},
			};

			let tags = Path::new("./cache").join(format!("{}_tags.json", name));
			let tags = match std::fs::read(tags) {
				Err(_) => FxHashSet::default(),
				Ok(file) => match serde_json::from_slice(&file) {
					Err(_) => continue,
					Ok(tags) => tags,
				},
			};

			self.sources.insert(name.clone(), Client::new(source));
			self.source_tags.insert(name, Some(tags));
		}
	}

	pub fn refresh_images(&mut self) {
		let start = SystemTime::now();
		println!("Refreshing images...");
		let Ok(read_dir) = std::fs::read_dir(&self.settings.download_folder) else {
			return;
		};

		let read_dir = read_dir.filter_map(Result::ok).collect_vec();

		self.images = read_dir
			.par_iter()
			.filter_map(|entry| {
				let path = entry.path();
				let file = File::open(&path).ok()?;
				let decoder = Decoder::new(&file);
				let reader = decoder.read_info().ok()?;

				let metadata = reader
					.info()
					.utf8_text
					.iter()
					.find(|chunk| chunk.keyword == "post_metadata")?;

				let json = metadata.get_text().ok()?;
				let post = serde_json::from_str::<Post>(&json).ok()?;

				let file_path = PathBuf::from(path.to_string_lossy().replace('\\', "/"));
				let preview = self
					.preview_cache
					.get_or_generate_image_preview(path, 128)
					.unwrap_or_default();

				let image = Image {
					info: post,
					file_path: file_path.clone(),
					preview_data: preview,
				};

				Some((file_path, image))
			})
			.collect();

		let preview_bytes: usize =
			self.images.values().map(|i| i.preview_data.as_bytes().len()).sum();

		println! {
			"Loaded {} images in {:?}. Total preview data size: {}MB.",
			self.images.len(),
			start.elapsed().unwrap(),
			preview_bytes as f32 / 1_000_000f32,
		};
	}

	pub fn refresh_datasets(&mut self) {
		let Ok(json) = std::fs::read("./datasets.json") else {
			eprintln!("Could not read 'datasets.json'.");
			return;
		};

		let Ok(datasets) = serde_json::from_slice(&json) else {
			eprintln!("Could not deserialize 'datasets.json'.");
			return;
		};

		self.datasets = datasets;
	}

	pub fn refresh_settings(&mut self) {
		let Ok(json) = std::fs::read("./settings.json") else {
			eprintln!("Could not read 'settings.json'.");
			return;
		};

		let Ok(settings) = serde_json::from_slice(&json) else {
			eprintln!("Could not deserialize 'settings.json'.");
			return;
		};

		self.settings = settings;
	}
}
