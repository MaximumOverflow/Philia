use crate::settings::{Settings, SettingsMessage};
use iced::widget::{Toggler, column};
use crate::application::Element;
use iced_aw::Card;

pub fn settings(context: &Settings) -> Element {
	let save_tags: Element = Toggler::new(Some("Save tags".into()), context.save_tags, |value| {
		SettingsMessage::ToggleSaveTags(value).into()
	}).into();

	let apply_letterboxing: Element =
		Toggler::new(Some("Apply letterboxing".into()), context.apply_letterboxing, |value| {
			SettingsMessage::ToggleApplyLetterboxing(value).into()
		}).into();

	Card::new("Settings", column![save_tags, apply_letterboxing])
		.on_close(SettingsMessage::SettingsClosed.into())
		.max_width(512)
		.into()
}
