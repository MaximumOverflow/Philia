use std::collections::HashMap;
use crate::gui::{post_image_list, post_preview, PostPreview, tob_bar};
use crate::search::{SearchProgress, Source, SearchParameters};
use philia::prelude::{DownloadAsync, GenericPost};
use iced::{Application, Command, Element, Theme};
use crate::download::DownloadProgress;
use iced::widget::{column, Row};
use iced::widget::image::Handle;
use notify_rust::Notification;

#[derive(Default)]
pub struct Philia {
	search_progress: SearchProgress,
	search_parameters: SearchParameters,
	download_progress: DownloadProgress,

	show: PostPreview,
	posts: HashMap<usize, (GenericPost, Handle)>,
}

#[derive(Debug, Clone)]
pub enum Message {
	#[allow(unused)]
	None,

	SearchRequested,
	SearchQueryChanged(String),
	SearchSourceChanged(Source),
	SearchCountChanged(Option<usize>),
	SearchReturned(Vec<GenericPost>),
	SearchProgressUp,

	DownloadPosts,
	DownloadProgressUp,
	DownloadPreview(Handle),
	PushPost((GenericPost, Handle)),

	HidePost,
	ShowPostWithId(usize),
	ShowPost(GenericPost, Handle),

	CopyTags(String),
}

impl Application for Philia {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = ();

	fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
		(
			Self {
				search_parameters: SearchParameters {
					count: 16,
					tags: String::new(),
					source: Source::Danbooru,
				},
				..Default::default()
			},
			Command::none(),
		)
	}

	fn title(&self) -> String {
		"Philia".into()
	}

	fn update(&mut self, message: Self::Message) -> Command<Message> {
		use Message::*;
		match message {
			None => Command::none(),

			SearchQueryChanged(query) => {
				self.search_parameters.tags = query;
				Command::none()
			}

			SearchSourceChanged(source) => {
				self.search_parameters.source = source;
				Command::none()
			}

			SearchCountChanged(count) => {
				self.search_parameters.count = count.unwrap_or_default();
				Command::none()
			}

			SearchRequested => {
				crate::search::perform_search(&mut self.posts, &mut self.search_progress, &self.search_parameters)
			}

			SearchReturned(posts) => crate::search::load_posts(posts, &mut self.search_progress),

			SearchProgressUp => {
				crate::search::search_progress_up(&mut self.search_progress);
				Command::none()
			}

			PushPost((post, handle)) => {
				self.posts.insert(post.id, (post, handle));
				crate::search::search_progress_up(&mut self.search_progress);
				Command::none()
			}

			DownloadPosts => crate::download::download_posts(&self.posts, &mut self.download_progress),

			DownloadProgressUp => crate::download::download_progress_up(&mut self.download_progress),

			DownloadPreview(handle) => crate::download::save_preview(handle),

			HidePost => {
				self.show = PostPreview::None;
				Command::none()
			}

			ShowPost(post, handle) => {
				self.show = PostPreview::Loaded { post, handle };
				Command::none()
			}

			ShowPostWithId(id) => {
				if let Some((post, _)) = self.posts.get(&id) {
					self.show = PostPreview::Loading;
					let post = post.clone();

					Command::perform(
						async move {
							match post.download_async().await {
								Err(_) => HidePost,
								Ok(bytes) => ShowPost(post, Handle::from_memory(bytes)),
							}
						},
						|m| m,
					)
				} else {
					Command::none()
				}
			}

			CopyTags(tags) => {
				let _ = Notification::new()
					.summary("Tags copied")
					.appname("Philia")
					.icon("copy")
					.show();

				iced::clipboard::write(tags)
			}
		}
	}

	fn view(&self) -> Element<'_, Self::Message> {
		let search = tob_bar(&self.search_parameters, &self.search_progress, &self.download_progress);
		let images = post_image_list(self.posts.values(), 6);

		let preview = post_preview(&self.show);
		let view: Element<'_, Message> = match preview {
			None => images.into(),
			Some(preview) => Row::with_children(vec![images.into(), preview]).into(),
		};

		column![search, view].into()
	}
}
