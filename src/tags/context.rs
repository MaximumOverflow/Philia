use crate::application::{Message, Philia, Source};
use std::time::{Duration, SystemTime};
use std::collections::HashSet;
use iced_native::Command;
use philia::prelude::*;
use std::sync::Arc;

pub enum TagSelectorContext {
	New,
	LoadingTagList,
	ShowTagSelector {
		source: Source,
		search: String,
		search_timestamp: Option<SystemTime>,

		shown_tags: Vec<String>,
		tag_set: HashSet<String>,
		tag_vec: Arc<Vec<String>>,
	},
}

impl TagSelectorContext {
	pub fn new_or_cached(source: Source) -> Self {
		let cache_patch = format!("cache/{:?}_tags.json", source);
		match std::fs::read_to_string(cache_patch) {
			Err(_) => Self::New,
			Ok(cache) => match serde_json::from_str::<Vec<String>>(&cache) {
				Err(_) => Self::New,
				Ok(cache) => {
					let mut shown_tags = vec![];
					get_default_tags(&cache, &mut shown_tags);
					
					let tag_set = cache.iter().cloned().collect();
					let tag_vec = Arc::new(cache);

					Self::ShowTagSelector {
						source,
						search: String::new(),
						search_timestamp: None,
						shown_tags, tag_set, tag_vec,
					}
				}
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
				let mut shown_tags = vec![];
				get_default_tags(&tags, &mut shown_tags);
				let tag_set = tags.iter().cloned().collect();
				context.tag_selector = TagSelectorContext::ShowTagSelector {
					shown_tags,
					source: context.source,
					search: String::new(),
					search_timestamp: None,
					tag_vec: Arc::new(tags), tag_set
				};

				TagSelectorMessage::SearchChanged(String::new()).handle(context)
			}

			TagSelectorMessage::SearchChanged(new_search) => {
				if let TagSelectorContext::ShowTagSelector {
					search,
					tag_vec: available_tags,
					search_timestamp,
					..
				} = &mut context.tag_selector
				{
					let timestamp = SystemTime::now();

					*search = new_search.clone();
					*search_timestamp = Some(timestamp);

					let mut shown_tags = vec![];
					let available_tags = available_tags.clone();
					Command::perform(
						async move {
							if !new_search.is_empty() {
								let results = available_tags
									.iter()
									.filter(|tag| tag.starts_with(&new_search))
									.cloned();

								shown_tags.extend(results);
							} else {
								get_default_tags(&available_tags, &mut shown_tags);
							}

							TagSelectorMessage::SearchCompleted(timestamp, shown_tags).into()
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

impl Drop for TagSelectorContext {
	fn drop(&mut self) {
		if let TagSelectorContext::ShowTagSelector { tag_vec, source, .. } = self {
			if let Ok(json) = serde_json::to_string_pretty(&**tag_vec) {
				let cache_patch = format!("cache/{:?}_tags.json", source);
				let _ = std::fs::write(cache_patch, json);
			}
		}
	}
}

fn get_default_tags(available: &[String], vec: &mut Vec<String>) {
	vec.extend(available.iter().take(50).cloned())
}