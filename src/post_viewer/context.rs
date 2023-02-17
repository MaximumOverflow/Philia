use crate::application::{Message, Philia};
use iced_native::image::Handle;
use crate::search::PostPreview;
use philia::prelude::Post;
use std::time::SystemTime;
use iced_native::Command;

pub enum PostViewerContext {
	None,
	Some {
		info: Post,
		image: PostImage,
		timestamp: SystemTime,
	},
}

#[derive(Debug, Clone)]
pub enum PostViewerMessage {
	Closed,
	Opened(usize),
	Loaded(SystemTime, PostImage),
}

#[derive(Debug, Clone)]
pub enum PostImage {
	Pending,
	Missing,
	Loaded(Handle),
	PreviewOnly(Handle),
}

impl From<PostViewerMessage> for Message {
	fn from(value: PostViewerMessage) -> Self {
		Message::PostPreviewMessage(value)
	}
}

impl PostViewerMessage {
	pub fn handle(self, context: &mut Philia) -> Command<Message> {
		match self {
			PostViewerMessage::Closed => {
				context.preview = PostViewerContext::None;
				Command::none()
			}

			PostViewerMessage::Loaded(time, handle) => {
				if let PostViewerContext::Some { image, timestamp, .. } = &mut context.preview {
					if *timestamp == time {
						*image = handle
					}
				}

				Command::none()
			}

			PostViewerMessage::Opened(index) => {
				let Some(client) = context.client.upgrade() else {
					return Command::none();
				};

				let timestamp = SystemTime::now();
				let post = context.search.results[index].clone();

				context.preview = PostViewerContext::Some {
					timestamp,
					info: post.info.clone(),
					image: match post.preview.clone() {
						PostPreview::Failed => PostImage::Pending,
						PostPreview::Pending => PostImage::Pending,
						PostPreview::Loaded(handle) => PostImage::PreviewOnly(handle),
					},
				};

				Command::perform(
					async move {
						match client.download_async(&post.info).await {
							Ok(bytes) => {
								let handle = Handle::from_memory(bytes);
								PostViewerMessage::Loaded(timestamp, PostImage::Loaded(handle)).into()
							}

							Err(err) => {
								println!("Could not download post {}.\nError:{:?}", post.info.id, err);
								PostViewerMessage::Loaded(
									timestamp,
									match post.preview {
										PostPreview::Loaded(handle) => PostImage::PreviewOnly(handle),
										PostPreview::Failed | PostPreview::Pending => PostImage::Missing,
									},
								)
								.into()
							}
						}
					},
					|msg| msg,
				)
			}
		}
	}
}
