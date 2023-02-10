use std::io::Cursor;
use std::sync::Arc;
use std::time::Duration;
use iced_native::Command;
use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat};
use native_dialog::FileDialog;
use crate::application::{Message, Philia};
use crate::search::SearchResult;
use philia::prelude::{Post, DownloadAsync};

#[derive(Default, Eq, PartialEq)]
pub enum DownloadContext {
	#[default]
	Complete,
	Downloading {
		total: usize,
		downloaded: usize,
	},
}

#[derive(Debug, Clone)]
pub enum DownloadMessage {
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
			DownloadMessage::DownloadRequested(posts) => {
				let path = match FileDialog::new().show_open_single_dir() {
					Ok(Some(path)) => path,
					Err(err) => panic!("{}", err),
					_ => return Command::none(),
				};

				context.download = DownloadContext::Downloading {
					downloaded: 0,
					total: posts.len(),
				};

				if !path.exists() {
					if let Err(err) = std::fs::create_dir(&path) {
						panic!("{:?}", err)
					}
				}

				Command::batch(posts.iter().map(|post| {
					let dir = path.clone();
					let post = post.clone();
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
									match post.info.download_async().await {
										Ok(mut bytes) => {
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
												"Failed downloading post {}. Retry {} of {}.",
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
				if let DownloadContext::Downloading { total, downloaded } = &mut context.download {
					if success {
						*downloaded += 1;
					} else {
						*total -= 1;
					}

					if *downloaded == *total {
						println!("Downloaded {} images.", downloaded);
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
