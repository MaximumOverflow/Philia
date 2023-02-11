use crate::post_viewer::{PostViewerContext, PostViewerMessage, post_viewer};
use crate::settings::{Settings, settings, SettingsMessage};
use crate::download::{DownloadContext, DownloadMessage};
use iced::{Application, Renderer};
use strum::{Display, EnumIter};
use iced_aw::{Modal, Split};
use iced_aw::split::Axis;
use iced_native::Command;
use crate::search::*;
use crate::style::*;
use crate::tags::*;

pub struct Philia {
	split: u16,
	pub source: Source,
	pub settings: Settings,
	pub search: SearchContext,
	pub download: DownloadContext,
	pub preview: PostViewerContext,
	pub tag_selector: TagSelectorContext,
}

#[derive(Debug, Clone)]
pub enum Message {
	SplitChanged(u16),
	SourceChanged(Source),
	SearchMessage(SearchMessage),
	DownloadMessage(DownloadMessage),
	SettingsMessage(SettingsMessage),
	TagSelectorMessage(TagSelectorMessage),
	PostPreviewMessage(PostViewerMessage),
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, EnumIter, Display)]
pub enum Source {
	#[default]
	E621,
	// Rule34,
}

pub type Element<'l> = iced::Element<'l, Message, Renderer<Theme>>;

impl Application for Philia {
	type Executor = iced::executor::Default;
	type Message = Message;
	type Theme = Theme;
	type Flags = Settings;

	fn new(settings: Self::Flags) -> (Self, Command<Self::Message>) {
		let philia = Self {
			settings,
			split: u16::MAX,
			source: Default::default(),
			search: Default::default(),
			download: Default::default(),
			preview: PostViewerContext::None,
			tag_selector: TagSelectorContext::new_or_cached(Default::default()),
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
				self.source = source;
				self.tag_selector = TagSelectorContext::new_or_cached(source);
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
