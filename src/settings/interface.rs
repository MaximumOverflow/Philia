use crate::settings::{Settings, SettingsMessage};
use iced::widget::{Toggler, Column};
use crate::application::Element;
use iced_aw::Card;
use iced::Padding;

pub fn settings(settings: &Settings) -> Element {
	let save_tags: Element = match settings.save_tags {
		false => Toggler::new(Some("Save tags".into()), settings.save_tags, |value| {
			SettingsMessage::ToggleSaveTags(value).into()
		})
		.into(),

		true => Column::with_children(vec![
			Toggler::new(Some("Save tags".into()), settings.save_tags, |value| {
				SettingsMessage::ToggleSaveTags(value).into()
			})
			.into(),
			Column::with_children(vec![
				Toggler::new(Some("\tRemove underscores".into()), settings.remove_tag_underscores, |value| {
					SettingsMessage::ToggleRemoveUnderscores(value).into()
				})
				.into(),
				Toggler::new(Some("\tEscape parentheses".into()), settings.escape_tag_parentheses, |value| {
					SettingsMessage::ToggleEscapeParentheses(value).into()
				})
				.into(),
			])
			.padding(Padding {
				top: 0,
				right: 0,
				bottom: 8,
				left: 16,
			})
			.spacing(4)
			.into(),
		])
		.spacing(4)
		.into(),
	};

	let apply_letterboxing: Element =
		Toggler::new(Some("Apply letterboxing".into()), settings.apply_letterboxing, |value| {
			SettingsMessage::ToggleApplyLetterboxing(value).into()
		})
		.into();

	let content = Column::with_children(vec![save_tags, apply_letterboxing]).spacing(4);

	Card::new("Settings", content)
		.on_close(SettingsMessage::SettingsClosed.into())
		.max_width(512)
		.into()
}
