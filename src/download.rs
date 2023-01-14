use image::{GenericImage, GenericImageView, ImageBuffer, ImageFormat};
use philia::prelude::{DownloadAsync, GenericPost, Post};
use native_dialog::{FileDialog, MessageDialog};
use crate::application::Message;
use iced_native::image::Handle;
use notify_rust::Notification;
use iced_native::Command;
use std::io::Cursor;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum DownloadProgress {
	#[default]
	Complete,
	DownloadingPosts {
		downloaded: usize,
		total: usize,
	},
}

pub fn download_posts(
	posts: &Vec<(usize, GenericPost, Handle)>,
	download_progress: &mut DownloadProgress,
) -> Command<Message> {
	let path = FileDialog::new().show_open_single_dir().unwrap();

	let add_letterboxing = MessageDialog::new()
		.set_title("Add letterboxing?")
		.set_text("Would you like to add letterboxing to the images?")
		.show_confirm()
		.unwrap();

	let save_tags = MessageDialog::new()
		.set_title("Save tags?")
		.set_text("Would you like to save the image's tags?")
		.show_confirm()
		.unwrap();

	match path {
		None => Command::none(),
		Some(dir) => {
			*download_progress = DownloadProgress::DownloadingPosts {
				downloaded: 0,
				total: posts.len(),
			};
			Command::batch(posts.iter().map(|(_, p, _)| {
				let post = p.clone();
				let dir = dir.clone();
				Command::perform(
					async move {
						if !dir.exists() {
							std::fs::create_dir(&dir).unwrap();
						}

						let img = format!("{}.{}", post.id, post.file_ext().unwrap());
						let img_path = dir.join(img);

						if !img_path.exists() {
							let mut retry = 0;
							const RETRY_COUNT: usize = 8;

							loop {
								match post.download_async().await {
									Ok(mut bytes) => {
										if add_letterboxing {
											apply_letterboxing(&mut bytes);
										}

										std::fs::write(img_path, bytes).unwrap();

										if save_tags {
											let tags = post
												.tags
												.iter()
												.map(|t| t.replace(|c| c == '_', " "))
												.collect::<Vec<_>>();

											let tags = tags
												.join(", ")
												.replace(|c| c == '(', "\\(")
												.replace(|c| c == ')', "\\)");

											let txt = format!("{}.txt", post.id);
											std::fs::write(dir.join(txt), &tags).unwrap();
										}

										break;
									}

									Err(err) if retry == RETRY_COUNT => {
										let _ = MessageDialog::new()
											.set_title(&format!(
												"Could not download post {}",
												post.id
											))
											.set_text(&format!("{:?}", err))
											.show_alert();
									}

									Err(_) => {
										println!(
											"Failed downloading post {}. Retry {} of {}.",
											post.id, retry, RETRY_COUNT,
										);

										retry += 1;
									}
								}
							}
						}
					},
					|_| Message::DownloadProgressUp,
				)
			}))
		}
	}
}

pub fn download_progress_up(progress: &mut DownloadProgress) -> Command<Message> {
	if let DownloadProgress::DownloadingPosts {
		downloaded: loaded,
		total,
	} = progress
	{
		*loaded += 1;

		if *loaded == *total {
			*progress = DownloadProgress::Complete;

			let _ = Notification::new()
				.summary("Download complete")
				.body("All images have been downloaded")
				.appname("Philia")
				.icon("download")
				.show();
		}
	}

	Command::none()
}

pub fn save_preview(_: Handle) -> Command<Message> {
	let _ = Notification::new()
		.summary("Post not saved")
		.body("Function not implemented")
		.appname("Philia")
		.icon("download")
		.show();

	Command::none()
}

fn apply_letterboxing(buffer: &mut Vec<u8>) {
	let image = image::load_from_memory(&buffer).unwrap();
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
	output
		.write_to(&mut Cursor::new(buffer), ImageFormat::Png)
		.unwrap();
}
