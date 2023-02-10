use crate::application::{Message, Philia, Source};
use std::time::{Duration, SystemTime};
use iced_native::Command;
use philia::prelude::*;
use std::sync::Arc;

pub enum TagSelectorContext {
	New,
	LoadingTagList,
	ShowTagSelector {
		search: String,
		search_timestamp: Option<SystemTime>,

		shown_tags: Vec<String>,
		available_tags: Arc<Vec<String>>,
	},
}

impl TagSelectorContext {
	pub fn new_or_cached(source: Source) -> Self {
		let cache_patch = format!("cache/{:?}_tags.json", source);
		match std::fs::read_to_string(cache_patch) {
			Err(_) => Self::New,
			Ok(cache) => match serde_json::from_str(&cache) {
				Err(_) => Self::New,
				Ok(cache) => Self::ShowTagSelector {
					search: String::new(),
					search_timestamp: None,

					shown_tags: vec![],
					available_tags: Arc::new(cache),
				},
			},
		}
	}
}

#[derive(Debug, Clone)]
pub enum TagSelectorMessage {
	ReloadRequested,
	ReloadCompleted(Vec<String>),
	TagIgnored(String),
	TagIncluded(String),
	TagExcluded(String),
	SearchChanged(String),
	SearchCompleted(SystemTime, Vec<String>),
}

impl From<TagSelectorMessage> for Message {
	fn from(value: TagSelectorMessage) -> Self {
		Message::TagSelectorMessage(value)
	}
}

impl TagSelectorMessage {
	pub fn handle(self, context: &mut Philia) -> Command<Message> {
		match self {
			TagSelectorMessage::ReloadRequested => match context.source {
				Source::E621 => {
					let source = context.source;
					context.tag_selector = TagSelectorContext::LoadingTagList;
					Command::perform(
						async move {
							println!("Loading tag list for {:?}...", source);
							let mut tags = vec![];

							for page in 1..=50 {
								let result = TagsAsync::get_tags_async(&E621, 320, page).await;
								match result {
									Ok(page_tags) => {
										println! {
											"({}) Extending tags by {} elements",
											page, page_tags.len()
										};
										tags.extend(page_tags);
										async_std::task::sleep(Duration::from_millis(600)).await;
									}

									_ => break,
								}
							}

							tags.sort_by_key(|tag| usize::MAX - tag.post_count);
							let tags = tags.into_iter().map(|tag| tag.name).collect();

							let _ = std::fs::create_dir("cache");
							let cache_patch = format!("cache/{:?}_tags.json", source);
							let cache_value = serde_json::to_string_pretty(&tags).unwrap();
							let cache_result = std::fs::write(cache_patch, cache_value);
							println!("Tag caching result: {:?}", cache_result);

							TagSelectorMessage::ReloadCompleted(tags).into()
						},
						|message| message,
					)
				} // Source::Rule34 => Command::none(),
			},

			TagSelectorMessage::ReloadCompleted(tags) => {
				context.tag_selector = TagSelectorContext::ShowTagSelector {
					search: String::new(),
					search_timestamp: None,
					shown_tags: vec![],
					available_tags: Arc::new(tags),
				};

				Command::none()
			}

			TagSelectorMessage::SearchChanged(new_search) => {
				if let TagSelectorContext::ShowTagSelector {
					search,
					search_timestamp,
					shown_tags,
					available_tags,
				} = &mut context.tag_selector
				{
					let timestamp = SystemTime::now();

					*search = new_search.clone();
					*search_timestamp = Some(timestamp);

					let available_tags = available_tags.clone();
					let mut populate_shown_tags = std::mem::take(shown_tags);
					Command::perform(
						async move {
							populate_shown_tags.clear();

							if !new_search.is_empty() {
								println!("Searching for {:?}...", new_search);

								let results = available_tags
									.iter()
									.filter(|tag| tag.starts_with(&new_search))
									.cloned();

								populate_shown_tags.extend(results);

								println! {
									"Search completed in {:?} ({} / {})",
									timestamp.elapsed().unwrap(),
									populate_shown_tags.len(),
									available_tags.len(),
								}
							}

							TagSelectorMessage::SearchCompleted(timestamp, populate_shown_tags).into()
						},
						|message| message,
					)
				} else {
					unreachable!()
				}
			}

			TagSelectorMessage::SearchCompleted(timestamp, tags) => {
				if let TagSelectorContext::ShowTagSelector {
					shown_tags,
					search_timestamp,
					..
				} = &mut context.tag_selector
				{
					if Some(timestamp) == *search_timestamp {
						*shown_tags = tags;
						*search_timestamp = None;
					}
				}

				Command::none()
			}

			TagSelectorMessage::TagIgnored(tag) => {
				context.search.include.remove(&tag);
				context.search.exclude.remove(&tag);
				Command::none()
			}

			TagSelectorMessage::TagIncluded(tag) => {
				context.search.exclude.remove(&tag);
				context.search.include.insert(tag);
				Command::none()
			}

			TagSelectorMessage::TagExcluded(tag) => {
				context.search.include.remove(&tag);
				context.search.exclude.insert(tag);
				Command::none()
			}
		}
	}
}
