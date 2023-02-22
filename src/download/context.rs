use std::collections::HashSet;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat};
use crate::application::{Message, Philia};
use std::time::{Duration, SystemTime};
use crate::settings::TagSettings;
use crate::search::SearchResult;
use philia::data::{Post, Tags};
use native_dialog::FileDialog;
use std::sync::{Arc, Mutex};
use iced_native::Command;
use itertools::Itertools;
use std::io::Cursor;
use std::path::Path;

#[derive(Default)]
pub enum DownloadContext {
	#[default]
	Complete,
	Downloading {
		total: usize,
		downloaded: usize,
		timestamp: Arc<Mutex<SystemTime>>,
	},
}

#[derive(Debug, Clone)]
pub enum DownloadMessage {
	DownloadCanceled,
	ImageDownloaded(bool),
	FilteredDownloadRequested,
	DownloadRequested(Arc<Vec<SearchResult>>),
}

impl From<DownloadMessage> for Message {
	fn from(value: DownloadMessage) -> Self {
		Self::DownloadMessage(value)
	}
}

impl DownloadMessage {
	pub fn handle(self, context: &mut Philia) -> Command<Message> {
		match self {
			DownloadMessage::DownloadCanceled => {
				if let DownloadContext::Downloading { timestamp, .. } = &mut context.download {
					println!("Download canceled");
					*timestamp.lock().unwrap() = SystemTime::now();
					context.download = DownloadContext::Complete;
				}

				Command::none()
			}
			
			DownloadMessage::FilteredDownloadRequested => {
				let posts = context.search.selected.iter().map(|i| {
					context.search.results[*i].clone()
				}).collect();
				
				DownloadMessage::DownloadRequested(Arc::new(posts)).handle(context)
			}

			DownloadMessage::DownloadRequested(posts) => {
				let Some(client) = context.client.upgrade() else {
					return Command::none();
				};

				let path = match FileDialog::new().show_open_single_dir() {
					Ok(Some(path)) => path,
					Err(err) => panic!("{}", err),
					_ => return Command::none(),
				};

				let initial_timestamp = SystemTime::now();
				let timestamp = Arc::new(Mutex::new(initial_timestamp));

				context.download = DownloadContext::Downloading {
					downloaded: 0,
					total: posts.len(),
					timestamp: timestamp.clone(),
				};

				if !path.exists() {
					if let Err(err) = std::fs::create_dir(&path) {
						panic!("{:?}", err)
					}
				}

				Command::batch(posts.iter().map(|post| {
					let dir = path.clone();
					let post = post.clone();
					let client = client.clone();
					let settings = context.settings.clone();
					let current_timestamp = timestamp.clone();

					Command::perform(
						async move {
							let Some(resource_url) = &post.info.resource_url else {
								return DownloadMessage::ImageDownloaded(false).into();
							};

							let img = format! {
								"{}.{}",
								post.info.id,
								Path::new(resource_url).extension().unwrap().to_str().unwrap()
							};

							let img_path = dir.join(img);

							let image_downloaded = if !img_path.exists() {
								let mut retry = 0;
								const RETRY_COUNT: usize = 8;

								loop {
									async_std::task::sleep(Duration::from_millis(100)).await;

									if *current_timestamp.lock().unwrap() != initial_timestamp {
										println!("Download of post {} canceled. Aborting...", post.info.id);
										return DownloadMessage::ImageDownloaded(false).into();
									}

									match client.download_async(&post.info).await {
										Ok(mut bytes) => {
											if *current_timestamp.lock().unwrap() != initial_timestamp {
												println!("Download of post {} canceled. Aborting...", post.info.id);
												return DownloadMessage::ImageDownloaded(false).into();
											}

											if settings.image_settings.apply_letterboxing {
												apply_letterboxing(&mut bytes);
											}

											std::fs::write(img_path, bytes).unwrap();
											break true;
										}

										Err(err) if retry == RETRY_COUNT => {
											println!("Could not download post {}.\nError:{:?}", post.info.id, err);
											break false;
										}

										Err(_) => {
											println!(
												"Failed downloading post {}. Retry {} of {}",
												post.info.id, retry, RETRY_COUNT,
											);
											retry += 1;
										}
									}
								}
							} else {
								false
							};

							if settings.tag_settings.save_tags {
								let txt_path = dir.join(format!("{}.txt", post.info.id));
								if !txt_path.exists() {
									let tags = get_tag_string(&post.info, &settings.tag_settings);
									std::fs::write(txt_path, tags).unwrap();
								}
							}

							DownloadMessage::ImageDownloaded(image_downloaded).into()
						},
						|message| message,
					)
				}))
			}

			DownloadMessage::ImageDownloaded(success) => {
				if let DownloadContext::Downloading { total, downloaded, .. } = &mut context.download {
					if success {
						*downloaded += 1;
					} else {
						*total -= 1;
					}

					if *downloaded == *total {
						println!("Downloaded {} images", downloaded);
						context.download = DownloadContext::Complete;
					}
				}

				Command::none()
			}
		}
	}
}

pub fn apply_letterboxing(buffer: &mut Vec<u8>) {
	let image = image::load_from_memory(buffer).unwrap();
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
}

pub fn get_tag_string(post: &Post, settings: &TagSettings) -> String {
	let categories = settings.ignore_categories.split(',')
		.map(|str| str.trim())
		.collect::<HashSet<_>>();
	
	let tags = match &post.tags {
		Tags::All(tags) => tags.iter().join(", "),
		Tags::Categorized(cats) => cats.iter()
			.filter(|(category, _)| !categories.contains(category.to_lowercase().as_str()))
			.flat_map(|(_, tags)| tags).join(", ")
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
