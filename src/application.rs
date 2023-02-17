use crate::post_viewer::{PostViewerContext, PostViewerMessage, post_viewer};
use crate::settings::{Settings, settings, SettingsMessage};
use crate::download::{DownloadContext, DownloadMessage};
use philia::prelude::{Client, Source};
use iced::{Application, Renderer};
use iced_aw::{Modal, Split};
use std::sync::{Arc, Weak};
use iced_aw::split::Axis;
use iced_native::Command;
use crate::search::*;
use crate::style::*;
use crate::tags::*;

pub struct Philia {
	split: u16,

	pub client: Weak<Client>,
	pub clients: Vec<Arc<Client>>,

	pub settings: Settings,
	pub search: SearchContext,
	pub download: DownloadContext,
	pub preview: PostViewerContext,
	pub tag_selector: TagSelectorContext,
}

#[derive(Debug, Clone)]
pub enum Message {
	SplitChanged(u16),
	SourceChanged(String),
	SearchMessage(SearchMessage),
	DownloadMessage(DownloadMessage),
	SettingsMessage(SettingsMessage),
	TagSelectorMessage(TagSelectorMessage),
	PostPreviewMessage(PostViewerMessage),
}

pub type Element<'l> = iced::Element<'l, Message, Renderer<Theme>>;

impl Application for Philia {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = Settings;

	fn new(settings: Self::Flags) -> (Self, Command<Self::Message>) {
		let sources = read_sources();
		let client = match sources.get(0) {
			None => Weak::new(),
			Some(arc) => Arc::downgrade(arc),
		};

		let philia = Self {
			settings,

			client: client.clone(),
			clients: sources,

			split: u16::MAX,
			search: Default::default(),
			download: Default::default(),
			preview: PostViewerContext::None,
			tag_selector: TagSelectorContext::new_or_cached(client.upgrade()),
		};

		(philia, Command::none())
	}

	fn title(&self) -> String {
		"Philia".into()
	}

	fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
		match message {
			Message::SplitChanged(value) => {
				self.split = value;
				Command::none()
			}

			Message::SourceChanged(source) => {
				self.client = Arc::downgrade(self.clients.iter().find(|c| c.source().name == source).unwrap());

				self.search.include.clear();
				self.search.exclude.clear();
				self.tag_selector = TagSelectorContext::new_or_cached(self.client.upgrade());
				Command::none()
			}

			Message::SearchMessage(message) => message.handle(self),
			Message::DownloadMessage(message) => message.handle(self),
			Message::SettingsMessage(message) => message.handle(self),
			Message::TagSelectorMessage(message) => message.handle(self),
			Message::PostPreviewMessage(message) => message.handle(self),
		}
	}

	fn view(&self) -> Element {
		let post_list: Element = post_list(self);
		let tag_selector: Element = tag_selector(self);

		let content: Element = Split::new(post_list, tag_selector, Some(self.split), Axis::Horizontal, |value| {
			Message::SplitChanged(value)
		})
		.min_size_first(300)
		.min_size_second(150)
		.into();

		let show_modal = self.settings.show
			|| match &self.preview {
				PostViewerContext::None => false,
				PostViewerContext::Some { .. } => true,
			};

		Modal::new(show_modal, content, || match &self.preview {
			PostViewerContext::None => settings(&self.settings),
			PostViewerContext::Some { info, image, .. } => post_viewer(&self.search, info, image.clone()),
		})
		.backdrop(SettingsMessage::SettingsClosed.into())
		.on_esc(SettingsMessage::SettingsClosed.into())
		.into()
	}
}

fn read_sources() -> Vec<Arc<Client>> {
	let Ok(entries) = std::fs::read_dir("sources") else {
		return vec![];
	};

	entries
		.flatten()
		.flat_map(|entry| std::fs::read_to_string(entry.path()))
		.filter_map(|json| serde_json::from_str::<Source>(&json).ok())
		.map(|source| Arc::new(Client::new(source)))
		.collect()
}
