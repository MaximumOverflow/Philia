use crate::application::{Message, Philia};
use serde::{Deserialize, Serialize};
use iced_native::Command;

pub const SETTINGS_PATH: &str = "settings.json";

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Settings {
	#[serde(skip_serializing, default = "Default::default")]
	pub show: bool,
	pub save_tags: bool,
	pub apply_letterboxing: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum SettingsMessage {
	SettingsOpened,
	SettingsClosed,
	ToggleSaveTags(bool),
	ToggleApplyLetterboxing(bool),
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
				context.settings.save_tags = value;
			}

			SettingsMessage::ToggleApplyLetterboxing(value) => {
				context.settings.apply_letterboxing = value;
			}
		}

		Command::none()
	}
}
