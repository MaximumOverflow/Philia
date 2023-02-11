use crate::application::{Message, Philia};
use philia::prelude::DownloadAsync;
use philia::prelude::GenericPost;
use iced_native::image::Handle;
use std::time::SystemTime;
use iced_native::Command;

pub enum PostPreviewContext {
	None,
	Some {
		info: GenericPost,
		timestamp: SystemTime,
		image: Result<Handle, Handle>,
	},
}

#[derive(Debug, Clone)]
pub enum PostPreviewMessage {
	PostPreviewClosed,
	PostPreviewOpened(usize),
	PostPreviewLoaded(SystemTime, Result<Handle, Handle>),
}

impl From<PostPreviewMessage> for Message {
	fn from(value: PostPreviewMessage) -> Self {
		Message::PostPreviewMessage(value)
	}
}

impl PostPreviewMessage {
	pub fn handle(self, context: &mut Philia) -> Command<Message> {
		match self {
			PostPreviewMessage::PostPreviewClosed => {
				context.preview = PostPreviewContext::None;
				Command::none()
			}

			PostPreviewMessage::PostPreviewLoaded(time, handle) => {
				if let PostPreviewContext::Some { image, timestamp, .. } = &mut context.preview {
					if *timestamp == time {
						*image = handle
					}
				}

				Command::none()
			}

			PostPreviewMessage::PostPreviewOpened(index) => {
				let timestamp = SystemTime::now();
				let post = context.search.results[index].clone();

				let handle = match &post.preview {
					None => return Command::none(),
					Some(handle) => handle.clone(),
				};

				context.preview = PostPreviewContext::Some {
					info: post.info.clone(),
					timestamp,
					image: Err(handle),
				};

				Command::perform(
					async move {
						match post.info.download_async().await {
							Ok(bytes) => {
								let handle = Handle::from_memory(bytes);
								PostPreviewMessage::PostPreviewLoaded(timestamp, Ok(handle)).into()
							}

							Err(err) => {
								println!("Could not download post {}.\nError:{:?}", post.info.id, err);
								PostPreviewMessage::PostPreviewLoaded(timestamp, Err(post.preview.unwrap())).into()
							}
						}
					},
					|msg| msg,
				)
			}
		}
	}
}
