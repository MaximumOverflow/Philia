use iced::widget::{Image, Text, Scrollable, Column, Button, Row, Container};
use crate::post_viewer::{PostImage, PostViewerMessage};
use crate::search::{SearchContext, SearchResult};
use iced::{Alignment, Length, Padding};
use crate::tags::TagSelectorMessage;
use crate::application::Element;
use crate::style::ButtonStyle;
use std::iter::repeat_with;
use itertools::Itertools;
use philia::data::Tags;
use iced_aw::Card;

pub fn post_viewer(search: &SearchContext, post: &SearchResult, post_image: PostImage) -> Element<'static> {
	let image: Element = match post_image {
		PostImage::Pending => Container::new(Text::new("Loading image...")),
		PostImage::Missing => Container::new(Text::new("Could not load image.")),
		PostImage::Loaded(ref handle) | PostImage::PreviewOnly(ref handle) => {
			let image = Image::new(handle.clone()).width(Length::Fill);
			Container::new(Scrollable::new(image))
		}
	}
	.width(Length::Fill)
	.height(Length::Fill)
	.center_x()
	.center_y()
	.into();

	let tags = match &post.info.tags {
		Tags::All(tags) => {
			let tags = make_tag_buttons(
				tags.iter().map(String::as_str),
				search
			);

			Column::with_children(tags)
				.align_items(Alignment::Center)
				.spacing(8)
		},

		Tags::Categorized(cats) => {
			let tags = cats.iter()
				.filter(|(category, tags)| {
					!category.is_empty() && !tags.is_empty()
				})
				.sorted_by(|(a, _), (b, _)| {
					a.cmp(b)
				})
				.map(|(category, tags)| {
					let mut category = category.clone();
					let first_char = category.chars().next().unwrap();

					if first_char.is_ascii() {
						unsafe {
							let bytes = category.as_bytes_mut();
							bytes[0] = first_char.to_uppercase().next().unwrap() as u8;
						}
					}

					let label: Element = Text::new(category).into();

					let tags = make_tag_buttons(
						tags.iter().map(String::as_str),
						search
					);

					let tags: Element = Column::with_children(tags)
						.align_items(Alignment::Center)
						.spacing(8)
						.into();

					Column::with_children(vec![label, tags])
						.align_items(Alignment::Center)
						.spacing(8)
						.into()
				}).collect();

			Column::with_children(tags)
				.align_items(Alignment::Center)
				.spacing(8)
		}
	};

	let info: Element = Container::new(Scrollable::new(tags))
	.height(Length::Fill)
	.center_x()
	.center_y()
	.padding(16)
	.into();

	let buttons: Element = Container::new(
		Row::with_children(vec![
			if let PostImage::Loaded(_) = &post_image {
				Button::new("Save").on_press(PostViewerMessage::Save.into()).into()
			} else {
				Button::new("Save").into()
			},
			Button::new("Copy tags")
				.on_press(PostViewerMessage::CopyTags.into())
				.into(),
		])
		.spacing(8),
	)
	.center_x()
	.padding(16)
	.into();

	let right_panel: Element = Column::with_children(vec![info, buttons])
		.align_items(Alignment::Center)
		.spacing(8)
		.into();

	let content = Row::with_children(vec![image, right_panel]).padding(4).spacing(16);

	let card = Card::new(Text::new("Post post_viewer"), content).on_close(PostViewerMessage::Closed.into());

	Column::new().push(card).padding(Padding::new(100)).into()
}

fn make_tag_buttons<'l>(tags: impl Iterator<Item=&'l str>, search: &SearchContext) -> Vec<Element<'static>> {
	let tag_buttons = tags.map(|tag| {
		let tag = tag.to_string();
		if search.exclude.contains(&tag) {
			Button::new(Text::new(tag.clone()))
				.style(ButtonStyle::ExcludeTag)
				.on_press(TagSelectorMessage::TagIgnored(tag.clone()).into())
				.into()
		} else if search.include.contains(&tag) {
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
	}).collect::<Vec<Element>>();

	let mut tags = repeat_with(|| Vec::with_capacity(3))
		.take((tag_buttons.len() as f32 / 3.0).ceil() as usize)
		.collect::<Vec<_>>();

	for (i, tag) in tag_buttons.into_iter().enumerate() {
		tags[i / 3].push(tag)
	}

	tags
		.into_iter()
		.map(|row| Row::with_children(row).spacing(8).into())
		.collect()
}