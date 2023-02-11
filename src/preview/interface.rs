use iced::widget::{Image, Text, Scrollable, Column, Button, Row};
use crate::preview::PostPreviewMessage;
use iced::{Alignment, Length, Padding};
use crate::tags::TagSelectorMessage;
use philia::prelude::GenericPost;
use crate::search::SearchContext;
use crate::application::Element;
use iced_native::image::Handle;
use crate::style::ButtonStyle;
use iced_native::row;
use iced_aw::Card;

pub fn preview(search: &SearchContext, info: &GenericPost, handle: Handle) -> Element<'static> {
	let image = Image::new(handle).width(Length::FillPortion(7));

	let image: Element = Scrollable::new(image).into();

	let mut tags = vec![];
	for chunk in info.tags.chunks(3) {
		let row = Row::with_children(
			chunk
				.iter()
				.map(|tag| {
					if search.exclude.contains(tag) {
						Button::new(Text::new(tag.clone()))
							.style(ButtonStyle::ExcludeTag)
							.on_press(TagSelectorMessage::TagIgnored(tag.clone()).into())
							.into()
					} else if search.include.contains(tag) {
						Button::new(Text::new(tag.clone()))
							.style(ButtonStyle::IncludeTag)
							.on_press(TagSelectorMessage::TagExcluded(tag.clone()).into())
							.into()
					} else {
						Button::new(Text::new(tag.clone()))
							.style(ButtonStyle::IgnoreTag)
							.on_press(TagSelectorMessage::TagIncluded(tag.clone()).into())
							.into()
					}
				})
				.collect(),
		)
		.spacing(8);

		tags.push(row.into())
	}

	let info: Element = Scrollable::new(
		Column::with_children(tags)
			.align_items(Alignment::Center)
			.width(Length::FillPortion(3))
			.spacing(8),
	)
	.into();

	let content = row![image, info].align_items(Alignment::Center).spacing(16);

	let card = Card::new(Text::new("Post preview"), content)
		.on_close(PostPreviewMessage::PostPreviewClosed.into());

	Column::new().push(card).padding(Padding::new(100)).into()
}
