use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat};
use philia::prelude::{Post, DownloadAsync};
use crate::application::{Message, Philia};
use std::time::{Duration, SystemTime};
use crate::search::SearchResult;
use native_dialog::FileDialog;
use std::sync::{Arc, Mutex};
use iced_native::Command;
use std::io::Cursor;

#[derive(Default)]
pub enum DownloadContext {
	#[default]
	Complete,
	Downloading {
		total: usize,
		downloaded: usize,
		timestamp: Arc<Mutex<SystemTime>>
	},
}

#[derive(Debug, Clone)]
pub enum DownloadMessage {
	DownloadCanceled,
	ImageDownloaded(bool),
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
			
			DownloadMessage::DownloadRequested(posts) => {
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
					let current_timestamp = timestamp.clone();
					let save_tags = context.settings.save_tags;
					let add_letterboxing = context.settings.apply_letterboxing;

					Command::perform(
						async move {
							let img = format!("{}.{}", post.info.id, post.info.file_ext().unwrap());
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
									
									match post.info.download_async().await {
										Ok(mut bytes) => {
											if *current_timestamp.lock().unwrap() != initial_timestamp {
												println!("Download of post {} canceled. Aborting...", post.info.id);
												return DownloadMessage::ImageDownloaded(false).into();
											}
											
											if add_letterboxing {
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

							if save_tags {
								let txt_path = dir.join(format!("{}.txt", post.info.id));
								if save_tags && !txt_path.exists() {
									let tags = post
										.info
										.tags
										.iter()
										.map(|t| t.replace(|c| c == '_', " "))
										.collect::<Vec<_>>();

									let tags = tags
										.join(", ")
										.replace(|c| c == '(', "\\(")
										.replace(|c| c == ')', "\\)");

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

fn apply_letterboxing(buffer: &mut Vec<u8>) {
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
