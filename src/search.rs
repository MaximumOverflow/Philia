use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use philia::search::SearchBuilder;
use crate::application::Message;
use image::imageops::FilterType;
use iced_native::image::Handle;
use iced_native::Command;
use philia::prelude::*;
use image::ImageFormat;
use std::io::Cursor;
use std::time::Duration;

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum Source {
	E621,
	Rule34,
	#[default]
	Danbooru,
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum SearchProgress {
	#[default]
	Complete,
	Searching,
	LoadingPosts {
		loaded: usize,
		total: usize,
	},
}

impl Display for Source {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		<Self as Debug>::fmt(self, f)
	}
}

#[derive(Default, Debug)]
pub struct SearchParameters {
	pub tags: String,
	pub source: Source,
	pub count: usize,
}

pub fn perform_search(
	posts: &mut HashMap<usize, (GenericPost, Handle)>,
	progress: &mut SearchProgress,
	parameters: &SearchParameters,
) -> Command<Message> {
	posts.clear();
	*progress = SearchProgress::Searching;
	async fn search(tags: String, count: usize, source: Source) -> Message {
		let tags = tags.split(|c| c == ' ');
		let mut search_builder = SearchBuilder::default();
		search_builder.exclude_tag("animated").include_tags(tags).limit(count);

		let search = match source {
			Source::E621 => search_builder.dyn_search_async(&E621),
			Source::Rule34 => search_builder.dyn_search_async(&Rule34),
			Source::Danbooru => search_builder.dyn_search_async(&Danbooru),
		};

		let posts = match search.await {
			Ok(posts) => posts,
			Err(err) => {
				let _ = native_dialog::MessageDialog::new()
					.set_title(&format!("{} returned an error", source))
					.set_text(&format!("{:?}", err))
					.show_alert();

				vec![]
			}
		};
		Message::SearchReturned(posts)
	}

	let search = search(parameters.tags.clone(), parameters.count, parameters.source);
	Command::perform(search, |f| f)
}

pub fn search_progress_up(progress: &mut SearchProgress) {
	if let SearchProgress::LoadingPosts { loaded, total } = progress {
		*loaded += 1;
		if *loaded == *total {
			*progress = SearchProgress::Complete;
		}
	}
}

pub fn load_posts(posts: Vec<GenericPost>, progress: &mut SearchProgress) -> Command<Message> {
	if posts.is_empty() {
		*progress = SearchProgress::Complete;
		return Command::none();
	}

	*progress = SearchProgress::LoadingPosts {
		loaded: 0,
		total: posts.len(),
	};

	Command::batch(posts.into_iter().map(|post| {
		const RETRY_COUNT: usize = 8;

		fn handle_failed(post: &GenericPost) -> Message {
			println!("Failed downloading preview for post {}. Aborting...", post.id);
			Message::SearchProgressUp
		}

		fn handle_retry(post: &GenericPost, retry: &mut usize) {
			println!(
				"Failed downloading preview for post {}. Retry {} of {}.",
				post.id, retry, RETRY_COUNT
			);

			std::thread::sleep(Duration::from_millis(500));
			*retry += 1;
		}

		Command::perform(
			async move {
				let mut retry = 0;

				loop {
					match reqwest::get(&post.resource_url).await {
						Ok(result) => match result.bytes().await {
							Ok(bytes) => {
								let mut bytes = bytes.to_vec();
								let image = match image::load_from_memory(&bytes) {
									Ok(image) => image,
									Err(_) => break handle_failed(&post),
								};

								const HORIZONTAL_PIXELS: u32 = 512;
								let aspect_ratio = image.height() as f32 / image.width() as f32;

								bytes.clear();
								image
									.resize(
										HORIZONTAL_PIXELS,
										(HORIZONTAL_PIXELS as f32 * aspect_ratio) as u32,
										FilterType::Nearest,
									)
									.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
									.unwrap();

								let handle = Handle::from_memory(bytes);
								break Message::PushPost((post, handle));
							}

							Err(_) if retry == RETRY_COUNT => break handle_failed(&post),
							Err(_) => handle_retry(&post, &mut retry),
						},

						Err(_) if retry == RETRY_COUNT => break handle_failed(&post),
						Err(_) => handle_retry(&post, &mut retry),
					}
				}
			},
			|msg| msg,
		)
	}))
}
