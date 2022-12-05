use iced::widget::{Row, TextInput, column, Button, Image, Column, Scrollable, PickList, Text};
use iced::{Application, Command, Element, Length, Theme};
use philia::prelude::{DownloadAsync, GenericPost, Post};
use crate::search::{SearchProgress, Source};
use iced::widget::image::Handle;
use native_dialog::FileDialog;
use std::iter::repeat_with;
use std::str::FromStr;

#[derive(Default)]
pub struct Philia {
	search_parameters: (
		String,
		Source,
		Option<usize>
	),
	
	search_progress: SearchProgress,
	download_progress: Option<(usize, usize)>,
	posts: Vec<(usize, GenericPost, Handle)>,
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
	PushPost((usize, GenericPost, Handle))
}

impl Application for Philia {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = ();

	fn new(_: Self::Flags) -> (Self, Command<Self::Message>) {
		(
			Self {
				search_parameters: (
					String::new(),
					Source::Danbooru,
					Some(16)
				),
				..Default::default()
			},
			Command::none()
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
				self.search_parameters.0 = query;
				Command::none()
			},
			
			SearchSourceChanged(source) => {
				self.search_parameters.1 = source;
				Command::none()
			}
			
			SearchCountChanged(count) => {
				self.search_parameters.2 = count;
				Command::none()
			},
			
			SearchRequested => {
				self.posts.clear();
				self.search_progress = SearchProgress::Searching;
				crate::search::search(
					self.search_parameters.0.clone(),
					self.search_parameters.2.unwrap(),
					self.search_parameters.1,
				)
			},
			
			SearchReturned(posts) => {
				if posts.len() != 0 {
					self.search_progress = SearchProgress::LoadingPosts {
						loaded: 0,
						total: posts.len()
					};

					crate::search::load_posts(posts)
				} else {
					Command::none()
				}
			},
			
			SearchProgressUp => {
				self.search_progress_up();
				Command::none()
			}
			
			PushPost((i, post, handle)) => {
				let index = self.posts.partition_point(|(idx, _, _)| *idx < i);
				self.posts.insert(index, (i, post, handle));
				self.search_progress_up();
				Command::none()
			},
			
			DownloadPosts => {
				let path = FileDialog::new()
					.show_open_single_dir()
					.unwrap();
				
				match path {
					Option::None => Command::none(),
					Some(dir) => {
						self.download_progress = Some((0, self.posts.len()));
						Command::batch(self.posts.iter().map(|(_, p, _)| {
							let post = p.clone();
							let dir = dir.clone();
							Command::perform(async move {
								if !dir.exists() {
									std::fs::create_dir(&dir).unwrap();
								}

								match post.download_async().await {
									Ok(bytes) => {
										let txt = format!("{}.txt", post.id);
										let img = format!("{}.{}", post.id, post.file_ext().unwrap());

										let tags = post.tags.join(", ")
											.replace(|c| c == '(', "\\(")
											.replace(|c| c == ')', "\\)");

										std::fs::write(dir.join(img), bytes).unwrap();
										std::fs::write(dir.join(txt), &tags).unwrap();
									}
									_ => unimplemented!(),
								}
							}, |_| DownloadProgressUp)
						}))
					}
				}
			},

			DownloadProgressUp => {
				let progress = self.download_progress.as_mut().unwrap();
				progress.0 += 1;
				
				if progress.0 == progress.1 {
					self.download_progress = Option::None;
				}

				Command::none()
			}
		}
	}
	
	fn view(&self) -> Element<'_, Self::Message> {
		let can_search 
			= self.search_progress == SearchProgress::Complete 
			&& self.search_parameters.2.unwrap_or_default() != 0;
		
		let search_query = {
			let search_query = TextInput::new(
				"Enter tags to search",
				&self.search_parameters.0,
				|value| Message::SearchQueryChanged(value)
			);

			match can_search {
				false => search_query,
				true => search_query.on_submit(Message::SearchRequested),
			}.into()
		};

		let search_count = {
			let value = match self.search_parameters.2 {
				None => String::new(),
				Some(v) => format!("{}", v)
			};

			let search_count = TextInput::new("Count", &value, |value| {
				Message::SearchCountChanged(usize::from_str(&value).ok())
			}).width(Length::Units(64));

			match can_search {
				false => search_count,
				true => search_count.on_submit(Message::SearchRequested),
			}.into()
		};
		
		let search_source = PickList::new(
			vec![Source::E621, Source::Rule34, Source::Danbooru],
			Some(self.search_parameters.1),
			|source| Message::SearchSourceChanged(source),
		).into();
		
		let search_button = match self.search_progress {
			SearchProgress::Complete => match can_search {
				false => Button::new("Search"),
				true => Button::new("Search").on_press(Message::SearchRequested),
			},
			SearchProgress::Searching => Button::new("Searching"),
			SearchProgress::LoadingPosts { loaded, total } => {
				Button::new(Text::new(format!("Loaded {} posts of {}", loaded, total)))
			}
		}.into();
		
		let download_button = match self.download_progress {
			Some((current, count)) => {
				let text = format!("Downloaded {} of {}", current, count);
				Button::new(Text::new(text))
			},
			None => if can_search {
				Button::new("Download All").on_press(Message::DownloadPosts)
			}
			else {
				Button::new("Download All")
			},
		}.into();
		
		let search = Row::with_children(vec![
			search_query,
			search_count,
			search_source,
			search_button,
			download_button,
		]).spacing(4);
		
		let mut columns: Vec<_> = repeat_with(|| vec![]).take(6).collect();
		for (i, _, handle) in self.posts.iter() {
			let image = Image::new(handle.clone())
				.width(Length::Fill);
			
			let column = *i % columns.len();
			columns[column].push(image.into());
		}
		
		let images = Row::with_children(columns
			.into_iter()
			.map(|i| {
				Column::with_children(i)
					.width(Length::Fill)
					.into()
			})
			.collect()
		).width(Length::Fill);
		
		let scroll = Scrollable::new(images);
		
		column![
			search,
			scroll,
		].into()
	}
}

impl Philia {
	fn search_progress_up(&mut self) {
		if let SearchProgress::LoadingPosts { loaded, total } = &mut self.search_progress {
			*loaded += 1;
			if *loaded == *total {
				self.search_progress = SearchProgress::Complete;
			}
		}
	}
}