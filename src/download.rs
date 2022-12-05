use philia::prelude::{DownloadAsync, GenericPost, Post};
use crate::application::Message;
use iced_native::image::Handle;
use native_dialog::FileDialog;
use notify_rust::Notification;
use iced_native::Command;

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

						match post.download_async().await {
							Ok(bytes) => {
								let txt = format!("{}.txt", post.id);
								let img = format!("{}.{}", post.id, post.file_ext().unwrap());

								let tags = post
									.tags
									.join(", ")
									.replace(|c| c == '(', "\\(")
									.replace(|c| c == ')', "\\)");

								std::fs::write(dir.join(img), bytes).unwrap();
								std::fs::write(dir.join(txt), &tags).unwrap();
							},
							
							Err(err) => {
								let _ = native_dialog::MessageDialog::new()
									.set_title(&format!("Could not download post {}", post.id))
									.set_text(&format!("{:?}", err))
									.show_alert();
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