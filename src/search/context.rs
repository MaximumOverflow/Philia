use crate::application::{Message, Philia, Source};
use philia::prelude::{E621, GenericPost};
use std::time::{Duration, SystemTime};
use std::fmt::{Display, Formatter};
use philia::search::SearchBuilder;
use image::imageops::FilterType;
use iced_native::image::Handle;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use iced_native::Command;
use image::ImageFormat;
use std::io::Cursor;
use strum::EnumIter;

pub struct SearchContext {
	pub page: usize,
	pub per_page: usize,
	pub sorting: Sorting,
	pub status: SearchStatus,
	pub include: HashSet<String>,
	pub exclude: HashSet<String>,
	pub results: Arc<Vec<SearchResult>>,
	pub timestamp: Arc<Mutex<SystemTime>>,
}

impl Default for SearchContext {
	fn default() -> Self {
		Self {
			page: 1,
			per_page: 16,
			results: Arc::new(vec![]),
			sorting: Default::default(),
			status: Default::default(),
			include: Default::default(),
			exclude: Default::default(),
			timestamp: Arc::new(Mutex::new(SystemTime::UNIX_EPOCH)),
		}
	}
}

#[derive(Default, Debug, Copy, Clone, Eq, PartialEq)]
pub enum SearchStatus {
	#[default]
	Complete,
	Searching,
	LoadingPosts {
		loaded: usize,
		total: usize,
	},
}

#[derive(Debug, Clone)]
pub struct SearchResult {
	pub size: (u32, u32),
	pub info: GenericPost,
	pub preview: Option<Handle>,
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, EnumIter)]
pub enum Sorting {
	#[default]
	Date,
	DateAsc,
	Score,
	ScoreAsc,
}

impl Display for Sorting {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Sorting::Date => f.write_str("Newest"),
			Sorting::DateAsc => f.write_str("Oldest"),
			Sorting::Score => f.write_str("Most liked"),
			Sorting::ScoreAsc => f.write_str("Least liked"),
		}
	}
}

impl Sorting {
	pub fn as_tag(&self, source: Source) -> &str {
		match source {
			Source::E621 => match self {
				Sorting::Date => "",
				Sorting::DateAsc => "order:id",
				Sorting::Score => "order:score",
				Sorting::ScoreAsc => "order:score_asc",
			}, 
			// Source::Rule34 => match self {
			// 	Sorting::Date => "",
			// 	Sorting::DateAsc => "sort:id",
			// 	Sorting::Score => "sort:score",
			// 	Sorting::ScoreAsc => "sort:score_asc",
			// }
		}
	}
}

#[derive(Debug, Clone)]
pub enum SearchMessage {
	SearchCanceled,
	SearchRequested,
	PageChanged(usize),
	PerPageChanged(usize),
	SortingChanged(Sorting),
	SearchReturned(Vec<GenericPost>),
	PushImage(usize, (u32, u32), Option<Handle>),
}

impl From<SearchMessage> for Message {
	fn from(value: SearchMessage) -> Self {
		Message::SearchMessage(value)
	}
}

impl SearchMessage {
	pub fn handle(self, context: &mut Philia) -> Command<Message> {
		match self {
			SearchMessage::PageChanged(value) => {
				context.search.page = value.clamp(1, usize::MAX);
				Command::none()
			}

			SearchMessage::PerPageChanged(value) => {
				context.search.per_page = value.clamp(1, 320);
				Command::none()
			}

			SearchMessage::SortingChanged(value) => {
				context.search.sorting = value;
				Command::none()
			}
			
			SearchMessage::SearchCanceled => {
				println!("Search canceled");
				context.search.status = SearchStatus::Complete;
				*context.search.timestamp.lock().unwrap() = SystemTime::now();
				Command::none()
			}

			SearchMessage::SearchReturned(posts) => {
				let results = posts
					.iter()
					.map(|info| SearchResult {
						size: (0, 0),
						info: info.clone(),
						preview: None,
					})
					.collect();

				context.search.results = Arc::new(results);

				context.search.status = SearchStatus::LoadingPosts {
					loaded: 0,
					total: posts.len(),
				};
				
				let current_timestamp = context.search.timestamp.clone();
				let initial_timestamp = *current_timestamp.lock().unwrap();

				Command::batch(posts.into_iter().enumerate().map(|(i, post)| {
					const RETRY_COUNT: usize = 8;

					fn handle_failed(i: usize, post: &GenericPost) -> Message {
						println!("Failed downloading preview for post {}. Aborting...", post.id);
						SearchMessage::PushImage(i, (0, 0), None).into()
					}

					fn handle_canceled(i: usize, post: &GenericPost) -> Message {
						println!("Download of preview for post {} canceled. Aborting...", post.id);
						SearchMessage::PushImage(i, (0, 0), None).into()
					}

					async fn handle_retry(post: &GenericPost, retry: &mut usize) {
						println!(
							"Failed downloading preview for post {}. Retry {} of {}",
							post.id, retry, RETRY_COUNT
						);

						async_std::task::sleep(Duration::from_millis(500)).await;
						*retry += 1;
					}

					let current_timestamp = current_timestamp.clone();
					
					Command::perform(
						async move {
							let mut retry = 0;

							loop {
								if *current_timestamp.lock().unwrap() != initial_timestamp {
									break handle_canceled(i, &post);
								}
								
								match reqwest::get(&post.resource_url).await {
									Ok(result) => match result.bytes().await {
										Ok(bytes) => {
											let mut bytes = bytes.to_vec();
											let image = match image::load_from_memory(&bytes) {
												Ok(image) => image,
												Err(_) => break handle_failed(i, &post),
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

											if *current_timestamp.lock().unwrap() != initial_timestamp {
												break handle_canceled(i, &post);
											}

											let handle = Handle::from_memory(bytes);
											break SearchMessage::PushImage(
												i,
												(image.width(), image.height()),
												Some(handle),
											)
											.into();
										}

										Err(_) if retry == RETRY_COUNT => break handle_failed(i, &post),
										Err(_) => handle_retry(&post, &mut retry).await,
									},

									Err(_) if retry == RETRY_COUNT => break handle_failed(i, &post),
									Err(_) => handle_retry(&post, &mut retry).await,
								}
							}
						},
						|message| message,
					)
				}))
			}

			SearchMessage::SearchRequested => {
				let source = context.source;
				let timestamp = SystemTime::now();
				context.search.results = Arc::new(vec![]);
				context.search.status = SearchStatus::Searching;
				*context.search.timestamp.lock().unwrap() = timestamp;

				let mut search_builder = SearchBuilder::default();
				search_builder
					.exclude_tag("animated")
					.include_tags(&context.search.include)
					.exclude_tags(&context.search.exclude)
					.include_tag(context.search.sorting.as_tag(source))
					.limit(context.search.per_page)
					.page(context.search.page);

				Command::perform(
					async move {
						println!("Staring search...");

						let search = match source {
							Source::E621 => search_builder.dyn_search_async(&E621),
							// Source::Rule34 => search_builder.dyn_search_async(&Rule34),
						};

						let posts = match search.await {
							Ok(posts) => posts,
							Err(err) => {
								println!("{:?}", err);
								vec![]
							}
						};

						println!("Found {} posts", posts.len());

						SearchMessage::SearchReturned(posts).into()
					},
					|message| message,
				)
			}

			SearchMessage::PushImage(i, size, handle) => {
				if let SearchStatus::LoadingPosts { loaded, total } = &mut context.search.status {
					if handle.is_some() {
						*loaded += 1
					} else {
						*total -= 1;
					}

					if *loaded == *total {
						println!("Search completed. {} posts loaded", loaded);
						context.search.status = SearchStatus::Complete;
					}
				}

				let results = Arc::get_mut(&mut context.search.results).unwrap();
				if let Some(result) = results.get_mut(i) {
					result.size = size;
					result.preview = handle
				}

				Command::none()
			}
		}
	}
}
