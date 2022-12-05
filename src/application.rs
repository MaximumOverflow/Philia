use crate::search::{SearchProgress, Source, SearchParameters};
use iced::{Application, Command, Element, Theme};
use crate::gui::{post_image_list, tob_bar};
use crate::download::DownloadProgress;
use philia::prelude::GenericPost;
use iced::widget::image::Handle;
use iced::widget::column;

#[derive(Default)]
pub struct Philia {
	pub search_parameters: SearchParameters,
	pub search_progress: SearchProgress,
	pub download_progress: DownloadProgress,
	pub posts: Vec<(usize, GenericPost, Handle)>,
}

#[derive(Debug, Clone)]
pub enum Message {
	SearchRequested,
	SearchQueryChanged(String),
	SearchSourceChanged(Source),
	SearchCountChanged(Option<usize>),
	SearchReturned(Vec<GenericPost>),
	SearchProgressUp,

	DownloadPosts,
	DownloadProgressUp,
	PushPost((usize, GenericPost, Handle)),
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

			SearchRequested => crate::search::perform_search(
				&mut self.posts,
				&mut self.search_progress,
				&self.search_parameters,
			),

			SearchReturned(posts) => crate::search::load_posts(posts, &mut self.search_progress),

			SearchProgressUp => {
				crate::search::search_progress_up(&mut self.search_progress);
				Command::none()
			}

			PushPost((i, post, handle)) => {
				let index = self.posts.partition_point(|(idx, _, _)| *idx < i);
				self.posts.insert(index, (i, post, handle));
				crate::search::search_progress_up(&mut self.search_progress);
				Command::none()
			}

			DownloadPosts => {
				crate::download::download_posts(&self.posts, &mut self.download_progress)
			}

			DownloadProgressUp => {
				crate::download::download_progress_up(&mut self.download_progress)
			}
		}
	}

	fn view(&self) -> Element<'_, Self::Message> {
		let search = tob_bar(
			&self.search_parameters,
			&self.search_progress,
			&self.download_progress
		);
		
		let images = self.posts.iter().map(|(_, _, handle)| handle);
		let images = post_image_list(images, 6);

		column![search, images].into()
	}
}
