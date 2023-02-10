use iced::{Application, Renderer};
use iced::widget::Text;
use iced_aw::{Card, Modal, Split};
use iced_aw::split::Axis;
use iced_native::Command;
use strum::{Display, EnumIter};
use crate::download::{DownloadContext, DownloadMessage};
use crate::search::*;
use crate::settings::{Settings, settings, SettingsMessage};
use crate::style::*;
use crate::tags::*;

pub struct Philia {
	split: u16,
	pub source: Source,
	pub settings: Settings,
	pub search: SearchContext,
	pub download: DownloadContext,
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

		Modal::new(self.settings.show, content, || {
			Card::new(Text::new("Settings"), settings(&self.settings))
				.on_close(SettingsMessage::SettingsClosed.into())
				.max_width(512)
				.style(CardStyle)
				.into()
		})
		.backdrop(SettingsMessage::SettingsClosed.into())
		.on_esc(SettingsMessage::SettingsClosed.into())
		.style(ModalStyle)
		.into()
	}
}
