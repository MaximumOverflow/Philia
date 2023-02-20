use crate::application::{Message, Philia};
use serde::{Deserialize, Serialize};
use iced_native::Command;

pub const SETTINGS_PATH: &str = "settings.json";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
	#[serde(skip_serializing, default = "Default::default")]
	pub show: bool,
	pub tag_settings: TagSettings,
	pub image_settings: ImageSettings,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct TagSettings {
	pub save_tags: bool,
	pub remove_underscores: bool,
	pub escape_parentheses: bool,
	pub ignore_categories: String,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ImageSettings {
	pub apply_letterboxing: bool,
}

#[derive(Debug, Clone)]
pub enum SettingsMessage {
	SettingsOpened,
	SettingsClosed,
	ToggleSaveTags(bool),
	ToggleRemoveUnderscores(bool),
	ToggleEscapeParentheses(bool),
	ToggleApplyLetterboxing(bool),
	IgnoredCategoriesChanged(String),
}

impl From<SettingsMessage> for Message {
	fn from(value: SettingsMessage) -> Self {
		Self::SettingsMessage(value)
	}
}

impl SettingsMessage {
	pub fn handle(self, context: &mut Philia) -> Command<Message> {
		match self {
			SettingsMessage::SettingsOpened => {
				context.settings.show = true;
			}

			SettingsMessage::SettingsClosed => {
				context.settings.show = false;
				if let Ok(json) = serde_json::to_string_pretty(&context.settings) {
					let _ = std::fs::write(SETTINGS_PATH, json);
				}
			}

			SettingsMessage::ToggleSaveTags(value) => {
				context.settings.tag_settings.save_tags = value;
			}

			SettingsMessage::ToggleRemoveUnderscores(value) => {
				context.settings.tag_settings.remove_underscores = value;
			}

			SettingsMessage::ToggleEscapeParentheses(value) => {
				context.settings.tag_settings.escape_parentheses = value;
			}

			SettingsMessage::ToggleApplyLetterboxing(value) => {
				context.settings.image_settings.apply_letterboxing = value;
			}
			
			SettingsMessage::IgnoredCategoriesChanged(value) => {
				context.settings.tag_settings.ignore_categories = value.to_lowercase();
			}
		}

		Command::none()
	}
}
