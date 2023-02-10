use iced::Length;
use iced::widget::{Text, Toggler, column};
use iced_native::row;
use crate::application::Element;
use crate::settings::{Settings, SettingsMessage};
use crate::style::{TextStyle, TogglerStyle};

pub fn settings(context: &Settings) -> Element {
	let save_tags: Element = Toggler::new(
		Some("Save tags".into()), context.save_tags,
		|value| SettingsMessage::ToggleSaveTags(value).into(),
	).style(TogglerStyle).into();

	let apply_letterboxing: Element = Toggler::new(
		Some("Apply letterboxing".into()), context.apply_letterboxing,
		|value| SettingsMessage::ToggleApplyLetterboxing(value).into(),
	).style(TogglerStyle).into();
	
	column![
		save_tags,
		apply_letterboxing
	].into()
}