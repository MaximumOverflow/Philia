use iced::widget::{Row, TextInput, column, Button, Image, Column, Scrollable, PickList, Text};
use crate::search::{SearchProgress, Source, SearchParameters};
use iced::{Application, Command, Element, Length, Theme};
use crate::download::DownloadProgress;
use philia::prelude::GenericPost;
use iced::widget::image::Handle;
use std::iter::repeat_with;
use std::str::FromStr;

#[derive(Default)]
pub struct Philia {
	pub search_parameters: SearchParameters,
	pub search_progress: SearchProgress,
	pub download_progress: DownloadProgress,
	pub posts: Vec<(usize, GenericPost, Handle)>,
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
		let can_search =
			self.search_progress == SearchProgress::Complete && self.search_parameters.count != 0;

		let search_query = {
			let search_query = TextInput::new(
				"Enter tags to search",
				&self.search_parameters.tags,
				|value| Message::SearchQueryChanged(value),
			);

			match can_search {
				false => search_query,
				true => search_query.on_submit(Message::SearchRequested),
			}
			.into()
		};

		let search_count = {
			let value = format!("{}", self.search_parameters.count);

			let search_count = TextInput::new("Count", &value, |value| {
				Message::SearchCountChanged(usize::from_str(&value).ok())
			})
			.width(Length::Units(64));

			match can_search {
				false => search_count,
				true => search_count.on_submit(Message::SearchRequested),
			}
			.into()
		};

		let search_source = PickList::new(
			vec![Source::E621, Source::Rule34, Source::Danbooru],
			Some(self.search_parameters.source),
			|source| Message::SearchSourceChanged(source),
		)
		.into();

		let search_button = match self.search_progress {
			SearchProgress::Complete => match can_search {
				false => Button::new("Search"),
				true => Button::new("Search").on_press(Message::SearchRequested),
			},
			SearchProgress::Searching => Button::new("Searching"),
			SearchProgress::LoadingPosts { loaded, total } => {
				Button::new(Text::new(format!("Loaded {} posts of {}", loaded, total)))
			}
		}
		.into();

		let download_button = match self.download_progress {
			DownloadProgress::DownloadingPosts { downloaded, total } => {
				let text = format!("Downloaded {} of {}", downloaded, total);
				Button::new(Text::new(text))
			}

			DownloadProgress::Complete => {
				if can_search {
					Button::new("Download All").on_press(Message::DownloadPosts)
				} else {
					Button::new("Download All")
				}
			}
		}
		.into();

		let search = Row::with_children(vec![
			search_query,
			search_count,
			search_source,
			search_button,
			download_button,
		])
		.spacing(4);

		let mut columns: Vec<_> = repeat_with(|| vec![]).take(6).collect();
		for (i, _, handle) in self.posts.iter() {
			let image = Image::new(handle.clone()).width(Length::Fill);

			let column = *i % columns.len();
			columns[column].push(image.into());
		}

		let images = Row::with_children(
			columns
				.into_iter()
				.map(|i| Column::with_children(i).width(Length::Fill).into())
				.collect(),
		)
		.width(Length::Fill);

		let scroll = Scrollable::new(images);

		column![search, scroll,].into()
	}
}
