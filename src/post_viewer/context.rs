use crate::download::{apply_letterboxing, get_tag_string};
use crate::search::{PostPreview, SearchResult};
use crate::application::{Message, Philia};
use iced_native::image::Handle;
use native_dialog::FileDialog;
use iced_native::image::Data;
use std::time::SystemTime;
use iced_native::Command;
use std::path::Path;

pub enum PostViewerContext {
	None,
	Some {
		post: SearchResult,
		image: PostImage,
		timestamp: SystemTime,
	},
}

#[derive(Debug, Clone)]
pub enum PostViewerMessage {
	Save,
	Closed,
	CopyTags,
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
					post: post.clone(),
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

			PostViewerMessage::CopyTags => {
				let PostViewerContext::Some { post, .. } = &context.preview else {
					return Command::none()
				};

				let tags = get_tag_string(
					&post.info,
					context.settings.remove_tag_underscores,
					context.settings.escape_tag_parentheses,
				);

				iced::clipboard::write(tags)
			}

			PostViewerMessage::Save => {
				macro_rules! let_or_ret {
					($expr: pat = $value: expr) => {
						let $expr = $value else { return Command::none() };
					};
				}

				let_or_ret!(PostViewerContext::Some { post, image, .. } = &context.preview);
				let_or_ret!(PostImage::Loaded(handle) = image);
				let_or_ret!(Data::Bytes(bytes) = handle.data());
				let_or_ret!(Some(resource_url) = &post.info.resource_url);
				let_or_ret!(Some(Some(filename)) = Path::new(resource_url).file_name().map(|e| e.to_str()));
				let_or_ret!(Some(Some(extension)) = Path::new(resource_url).extension().map(|e| e.to_str()));

				let path = match FileDialog::new()
					.add_filter("Image format", &[extension])
					.set_filename(filename)
					.show_save_single_file()
				{
					Ok(Some(path)) => path,
					Err(err) => panic!("{}", err),
					_ => return Command::none(),
				};

				let mut bytes = bytes.to_vec();

				if context.settings.apply_letterboxing {
					apply_letterboxing(&mut bytes)
				}

				if context.settings.save_tags {
					if let Some(Some(name)) = path.file_stem().map(|e| e.to_str()) {
						let path = path.parent().unwrap_or(Path::new("")).join(format!("{}.txt", name));
						let tags = get_tag_string(
							&post.info,
							context.settings.remove_tag_underscores,
							context.settings.escape_tag_parentheses,
						);
						let _ = std::fs::write(path, tags);
					}
				}

				let _ = std::fs::write(path, bytes);
				Command::none()
			}
		}
	}
}
