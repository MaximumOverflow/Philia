use std::collections::HashSet;
use std::fs::File;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use crate::sources::SOURCES;
use philia::prelude::{Post, Tags};
use std::io::{BufWriter, Cursor};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use png::{BitDepth, ColorType, Compression, Encoder};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, State};
use crate::settings::{DownloadSettingsState, TagSettings};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Options {
	dataset: Option<String>,
	collection: Option<String>,
}

#[tauri::command]
pub async fn download_posts(
	source: String, posts: Vec<Post>, options: Options, handle: AppHandle,
	download_settings: State<'_, DownloadSettingsState>,
) -> Result<(), String> {
	let Some(source) = SOURCES.get(&source) else {
		eprintln!("Source {source} not found");
		return Err("Source not found".into());
	};

	let (download_folder, letterboxing) = {
		let settings = download_settings.lock().unwrap();
		(
			settings.download_folder.clone(),
			settings.apply_letterboxing,
		)
	};

	let count = posts.len() as f32;
	let mut progress = AtomicUsize::new(0);
	let progress = Arc::new(&mut progress);

	posts
		.into_par_iter()
		.for_each_with((progress, handle), |(progress, handle), post| {
			macro_rules! inc_ret {
				() => {{
					let progress = progress.fetch_add(1, Ordering::Relaxed) as f32;
					let _ =
						handle.emit_all("download_progress", ((progress / count) * 100.0).trunc());
					return;
				}};
			}

			let filename = format!("{}.png", post.id);
			let filepath = download_folder.join(filename);
			if filepath.exists() {
				inc_ret!();
			}

			match post.resource_url.as_ref().map(String::as_str) {
				Some("gif") | Some("mp4") | Some("ogg") | Some("flv") | Some("webm") => inc_ret!(),
				_ => {},
			}

			println!("Downloading {:?}", post.resource_url);

			let mut data = match source.download(&post) {
				Ok(data) => data,
				Err(err) => {
					eprintln!("{:?}", err);
					inc_ret!();
				},
			};

			if let Err(err) = match letterboxing {
				true => apply_letterboxing(&mut data),
				false => convert_to_png(&mut data),
			} {
				eprintln!("{:?}", err);
				inc_ret!();
			}

			let image = image::load_from_memory(&data).unwrap();

			let file = match File::create(filepath) {
				Ok(file) => file,
				Err(err) => {
					eprintln!("{:?}", err);
					inc_ret!();
				},
			};

			let buf_writer = BufWriter::new(file);
			let mut encoder = Encoder::new(buf_writer, image.width(), image.height());
			encoder.set_depth(BitDepth::Eight);
			encoder.set_color(ColorType::Rgba);
			encoder.set_compression(Compression::Best);

			let post_metadata = serde_json::to_string(&post).unwrap();
			if let Err(err) = encoder.add_text_chunk("post_metadata".into(), post_metadata) {
				eprintln!("Could not write tags {:?}", err);
				inc_ret!();
			}

			let mut writer = match encoder.write_header() {
				Ok(writer) => writer,
				Err(err) => {
					eprintln!("{:?}", err);
					inc_ret!();
				},
			};

			let pixel_bytes = image.as_bytes();
			if let Err(err) = writer.write_image_data(&pixel_bytes) {
				eprintln!("{:?}", err);
				inc_ret!();
			}

			inc_ret!();
		});
	Ok(())
}

pub fn convert_to_png(buffer: &mut Vec<u8>) -> Result<(), String> {
	let mut new_buffer = vec![];
	let image = image::load_from_memory(buffer)
		.map_err(|e| e.to_string())?
		.to_rgba8();
	
	image
		.write_to(&mut Cursor::new(&mut new_buffer), ImageFormat::Png)
		.unwrap();
	*buffer = new_buffer;
	Ok(())
}

pub fn apply_letterboxing(buffer: &mut Vec<u8>) -> Result<(), String> {
	let image = image::load_from_memory(buffer)
		.map_err(|e| e.to_string())?;
	
	let dimensions = image.width().max(image.height());
	let mut output = ImageBuffer::from_pixel(dimensions, dimensions, [0, 0, 0, 255].into());

	let x_offset = (dimensions - image.width()) / 2;
	let y_offset = (dimensions - image.height()) / 2;
	for (x, y, p) in image.pixels() {
		unsafe {
			output.unsafe_put_pixel(x + x_offset, y + y_offset, p);
		}
	}

	buffer.clear();
	output.write_to(&mut Cursor::new(buffer), ImageFormat::Png).unwrap();
	Ok(())
}

pub fn get_tag_string(post: &Post, settings: &TagSettings) -> String {
	let categories = settings
		.ignore_categories
		.split(',')
		.map(|str| str.trim())
		.collect::<HashSet<_>>();

	let tags = match &post.tags {
		Tags::All(tags) => tags.iter().join(", "),
		Tags::Categorized(cats) => cats
			.iter()
			.filter(|(category, _)| !categories.contains(category.to_lowercase().as_str()))
			.flat_map(|(_, tags)| tags)
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
