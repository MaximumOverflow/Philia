use std::fs::File;
use image::ImageFormat;
use crate::sources::SOURCES;
use philia::prelude::Post;
use std::io::{BufWriter, Cursor};
use std::sync::{Arc, Mutex};
use png::{BitDepth, ColorType, Compression, Encoder};
use tauri::{AppHandle, Manager, State};
use crate::settings::{SettingsState};

#[tauri::command]
pub async fn download_posts(
	source: String, posts: Vec<Post>, handle: AppHandle,
	download_settings: State<'_, SettingsState>,
) -> Result<Vec<String>, String> {
	let Some(source) = SOURCES.get(&source) else {
		eprintln!("Source {source} not found");
		return Err("Source not found".into());
	};

	let download_folder = {
		let settings = download_settings.lock().unwrap();
		settings.download_folder.clone()
	};

	let count = posts.len() as f32;
	let progress = Arc::new(Mutex::new(0f32));

	let promises: Vec<_> = posts
		.into_iter()
		.map(move |post| {
			let handle = handle.clone();
			let progress = progress.clone();
			let download_folder = download_folder.clone();
			tauri::async_runtime::spawn(async move {
				macro_rules! inc_ret {
					($val: expr) => {{
						let mut progress = progress.lock().unwrap();
						let _ = handle
							.emit_all("download_progress", ((*progress / count) * 100.0).trunc());
						*progress += 1.0;
						return $val;
					}};
				}

				let filename = format!("{}_{}.png", post.source, post.id);
				let filepath = download_folder.join(filename);
				if filepath.exists() {
					inc_ret!(Err("File exists"));
				}

				match post.resource_url.as_ref().map(String::as_str) {
					Some("gif") | Some("mp4") | Some("ogg") | Some("flv") | Some("webm") => {
						inc_ret!(Err("Unsupported file type"))
					},
					_ => {},
				}

				let mut data = match source.download_async(&post).await {
					Ok(data) => data,
					Err(err) => {
						eprintln!("{:?}", err);
						inc_ret!(Err("Download failed"));
					},
				};

				if let Err(err) = convert_to_png(&mut data) {
					eprintln!("{:?}", err);
					inc_ret!(Err("Png conversion failed"));
				}

				let image = image::load_from_memory(&data).unwrap();

				let file = match File::create(&filepath) {
					Ok(file) => file,
					Err(err) => {
						eprintln!("{:?}", err);
						inc_ret!(Err("File creation failed"));
					},
				};

				let buf_writer = BufWriter::new(file);
				let mut encoder = Encoder::new(buf_writer, image.width(), image.height());
				encoder.set_depth(BitDepth::Eight);
				encoder.set_color(ColorType::Rgba);
				encoder.set_compression(Compression::Best);

				let post_metadata = serde_json::to_string(&post).unwrap();
				if let Err(err) = encoder.add_itxt_chunk("post_metadata".into(), post_metadata) {
					eprintln!("Could not write tags {:?}", err);
					inc_ret!(Err("Metadata encoding failed"));
				}

				let mut writer = match encoder.write_header() {
					Ok(writer) => writer,
					Err(err) => {
						eprintln!("{:?}", err);
						inc_ret!(Err("Header creation failed"));
					},
				};

				let pixel_bytes = image.as_bytes();
				if let Err(err) = writer.write_image_data(&pixel_bytes) {
					eprintln!("{:?}", err);
					inc_ret!(Err("Save operation failed"));
				}

				inc_ret!(Ok(filepath));
			})
		})
		.collect();

	let mut paths = Vec::with_capacity(promises.len());
	for handle in promises {
		if let Ok(Ok(path)) = handle.await {
			paths.push(path.to_string_lossy().replace('\\', "/"));
		}
	}

	Ok(paths)
}

pub fn convert_to_png(buffer: &mut Vec<u8>) -> Result<(), String> {
	let mut new_buffer = vec![];
	let image = image::load_from_memory(buffer).map_err(|e| e.to_string())?.to_rgba8();

	image
		.write_to(&mut Cursor::new(&mut new_buffer), ImageFormat::Png)
		.unwrap();
	*buffer = new_buffer;
	Ok(())
}