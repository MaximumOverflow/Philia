use crate::settings::{Settings, SettingsMessage};
use iced::widget::{Toggler, column};
use crate::application::Element;
use crate::style::TogglerStyle;

pub fn settings(context: &Settings) -> Element {
	let save_tags: Element = Toggler::new(Some("Save tags".into()), context.save_tags, |value| {
		SettingsMessage::ToggleSaveTags(value).into()
	})
	.style(TogglerStyle)
	.into();

	let apply_letterboxing: Element =
		Toggler::new(Some("Apply letterboxing".into()), context.apply_letterboxing, |value| {
			SettingsMessage::ToggleApplyLetterboxing(value).into()
		})
		.style(TogglerStyle)
		.into();

	column![save_tags, apply_letterboxing].into()
}
