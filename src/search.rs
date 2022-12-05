use std::fmt::{Debug, Display, Formatter};
use philia::search::SearchBuilder;
use crate::application::Message;
use image::imageops::FilterType;
use iced_native::image::Handle;
use iced_native::Command;
use philia::prelude::*;
use image::ImageFormat;
use std::io::Cursor;

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
	LoadingPosts { loaded: usize, total: usize }
}

impl Display for Source {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		<Self as Debug>::fmt(self, f)
	}
}

pub fn search(tags: String, count: usize, source: Source) -> Command<Message> {
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
				println!("{:#?}", err);
				vec![]
			}
		};
		Message::SearchReturned(posts)
	}
	
	Command::perform(search(tags, count, source), |f| f)
}

pub fn load_posts(posts: Vec<GenericPost>) -> Command<Message> {
	Command::batch(posts.into_iter().enumerate().map(|(i, post)| Command::perform(
		async move {
			match reqwest::get(&post.resource_url).await {
				Ok(result) => match result.bytes().await {
					Ok(bytes) => {
						let mut bytes = bytes.to_vec();
						let image = image::load_from_memory(&bytes).unwrap();

						const HORIZONTAL_PIXELS: u32 = 512;
						let aspect_ratio = image.height() as f32 / image.width() as f32;

						bytes.clear();
						image
							.resize(
								HORIZONTAL_PIXELS,
								(HORIZONTAL_PIXELS as f32 * aspect_ratio) as u32,
								FilterType::Nearest
							)
							.write_to(
								&mut Cursor::new(&mut bytes),
								ImageFormat::Png
							).unwrap();

						let handle = Handle::from_memory(bytes);
						Message::PushPost((i, post, handle))
					},
					Err(_) => Message::SearchProgressUp,
				},
				Err(_) => Message::SearchProgressUp,
			}
		},
		|msg| msg
	)))
}