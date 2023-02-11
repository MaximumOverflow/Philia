use iced::widget::{Image, Text, Scrollable, Column, Button, Row, Container};
use crate::post_viewer::{PostImage, PostViewerMessage};
use iced::{Alignment, Length, Padding};
use crate::tags::TagSelectorMessage;
use crate::search::SearchContext;
use philia::prelude::GenericPost;
use crate::application::Element;
use crate::style::ButtonStyle;
use iced_native::row;
use iced_aw::Card;

pub fn post_viewer(search: &SearchContext, info: &GenericPost, image: PostImage) -> Element<'static> {
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
			.width(Length::Shrink)
			.spacing(8),
	)
	.into();

	let content = row![image, info].align_items(Alignment::Center).spacing(16);

	let card = Card::new(Text::new("Post post_viewer"), content).on_close(PostViewerMessage::Closed.into());

	Column::new().push(card).padding(Padding::new(100)).into()
}
