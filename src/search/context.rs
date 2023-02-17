use philia::prelude::{Order, SearchBuilder};
use crate::application::{Message, Philia};
use std::time::{Duration, SystemTime};
use crate::tags::TagSelectorContext;
use std::fmt::{Display, Formatter};
use image::imageops::FilterType;
use iced_native::image::Handle;
use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use philia::prelude::Post;
use iced_native::Command;
use image::ImageFormat;
use std::error::Error;
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
	pub info: Post,
	pub size: (u32, u32),
	pub preview: PostPreview,
}

#[derive(Debug, Clone)]
pub enum PostPreview {
	Failed,
	Pending,
	Loaded(Handle),
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

#[derive(Debug, Clone)]
pub enum SearchMessage {
	SearchCanceled,
	SearchRequested,
	PageChanged(usize),
	PerPageChanged(usize),
	SortingChanged(Sorting),
	SearchReturned(Vec<Post>),
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
				if posts.is_empty() {
					context.search.status = SearchStatus::Complete;
					return Command::none();
				}

				if let TagSelectorContext::ShowTagSelector { tag_vec, tag_set, .. } = &mut context.tag_selector {
					let mut tags = (**tag_vec).clone();
					for tag in posts.iter().flat_map(|p| p.tags.iter()) {
						let tag = tag.to_string();
						if tag_set.insert(tag.clone()) {
							tags.push(tag);
						}
					}

					*tag_vec = Arc::new(tags)
				}

				let results = posts
					.iter()
					.map(|info| SearchResult {
						size: (0, 0),
						info: info.clone(),
						preview: PostPreview::Pending,
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

					fn handle_failed(i: usize, post: &Post) -> Message {
						println!("Failed downloading preview for post {}. Aborting...", post.id);
						SearchMessage::PushImage(i, (0, 0), None).into()
					}

					fn handle_failed_err(i: usize, post: &Post, err: impl Error) -> Message {
						println!("Failed downloading preview for post {}. Aborting... {:#?}", post.id, err);
						SearchMessage::PushImage(i, (0, 0), None).into()
					}

					fn handle_canceled(i: usize, post: &Post) -> Message {
						println!("Download of preview for post {} canceled. Aborting...", post.id);
						SearchMessage::PushImage(i, (0, 0), None).into()
					}

					async fn handle_retry(post: &Post, retry: &mut usize) {
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

								let Some(url) = &post.resource_url else {
									break handle_failed(i, &post)
								};

								match reqwest::get(url).await {
									Ok(result) => match result.bytes().await {
										Ok(bytes) => {
											let mut bytes = bytes.to_vec();
											let image = match image::load_from_memory(&bytes) {
												Ok(image) => image,
												Err(err) => break handle_failed_err(i, &post, err),
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

										Err(err) if retry == RETRY_COUNT => break handle_failed_err(i, &post, err),
										Err(_) => handle_retry(&post, &mut retry).await,
									},

									Err(err) if retry == RETRY_COUNT => break handle_failed_err(i, &post, err),
									Err(_) => handle_retry(&post, &mut retry).await,
								}
							}
						},
						|message| message,
					)
				}))
			}

			SearchMessage::SearchRequested => {
				let Some(client) = context.client.upgrade() else {
					return Command::none();
				};

				let timestamp = SystemTime::now();
				context.search.results = Arc::new(vec![]);
				context.search.status = SearchStatus::Searching;
				*context.search.timestamp.lock().unwrap() = timestamp;

				let page = context.search.page;
				let per_page = context.search.per_page;
				let include = context.search.include.clone();
				let exclude = context.search.exclude.clone();

				let order = match context.search.sorting {
					Sorting::Date => Order::Newest,
					Sorting::DateAsc => Order::Oldest,
					Sorting::Score => Order::MostLiked,
					Sorting::ScoreAsc => Order::LeastLiked,
				};

				Command::perform(
					async move {
						println!("Staring search...");

						let mut search_builder = SearchBuilder::new(&client);
						search_builder
							.exclude_tag("animated")
							.include_tags(include)
							.exclude_tags(exclude)
							.order(order)
							.limit(per_page)
							.page(page);

						let search = search_builder.search_async().await;

						let posts = match search {
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
					result.preview = match handle {
						None => PostPreview::Failed,
						Some(handle) => PostPreview::Loaded(handle),
					}
				}

				Command::none()
			}
		}
	}
}
