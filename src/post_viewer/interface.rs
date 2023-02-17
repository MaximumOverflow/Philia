use iced::widget::{Image, Text, Scrollable, Column, Button, Row, Container};
use crate::post_viewer::{PostImage, PostViewerMessage};
use iced::{Alignment, Length, Padding};
use crate::tags::TagSelectorMessage;
use crate::search::SearchContext;
use crate::application::Element;
use crate::style::ButtonStyle;
use std::iter::repeat_with;
use philia::prelude::Post;
use iced_native::row;
use iced_aw::Card;

pub fn post_viewer(search: &SearchContext, info: &Post, image: PostImage) -> Element<'static> {
	let image: Element = match image {
		PostImage::Pending => Container::new(Text::new("Loading image..."))
			.width(Length::Fill)
			.height(Length::Fill)
			.center_x()
			.center_y()
			.into(),

		PostImage::Missing => Container::new(Text::new("Could not load image."))
			.width(Length::Fill)
			.height(Length::Fill)
			.center_x()
			.center_y()
			.into(),

		PostImage::Loaded(handle) | PostImage::PreviewOnly(handle) => {
			let image = Image::new(handle).width(Length::Fill);
			Scrollable::new(image).into()
		}
	};

	let tag_buttons = info
		.tags
		.iter()
		.map(|tag| {
			if search.exclude.contains(tag) {
				Button::new(Text::new(tag.to_string()))
					.style(ButtonStyle::ExcludeTag)
					.on_press(TagSelectorMessage::TagIgnored(tag.to_string()).into())
					.into()
			} else if search.include.contains(tag) {
				Button::new(Text::new(tag.to_string()))
					.style(ButtonStyle::IncludeTag)
					.on_press(TagSelectorMessage::TagExcluded(tag.to_string()).into())
					.into()
			} else {
				Button::new(Text::new(tag.to_string()))
					.style(ButtonStyle::IgnoreTag)
					.on_press(TagSelectorMessage::TagIncluded(tag.to_string()).into())
					.into()
			}
		})
		.collect::<Vec<Element>>();

	let mut tags = repeat_with(|| Vec::with_capacity(3))
		.take((tag_buttons.len() as f32 / 3.0).ceil() as usize)
		.collect::<Vec<_>>();

	for (i, tag) in tag_buttons.into_iter().enumerate() {
		tags[i / 3].push(tag)
	}

	let tags = tags
		.into_iter()
		.map(|row| Row::with_children(row).spacing(8).into())
		.collect();

	let info: Element = Scrollable::new(
		Column::with_children(tags)
			.align_items(Alignment::Center)
			.width(Length::Shrink)
			.spacing(8),
	)
	.into();

	let content = row![image, info].align_items(Alignment::Center).spacing(16);

	let card = Card::new(Text::new("Post post_viewer"), content).on_close(PostViewerMessage::Closed.into());

	Column::new().push(card).padding(Padding::new(100)).into()
}
