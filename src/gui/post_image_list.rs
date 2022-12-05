use iced::widget::{Column, Image, Row, Scrollable};
use crate::application::Message;
use iced_native::image::Handle;
use std::iter::repeat_with;
use iced::Length;

pub fn post_image_list<'l>(
	images: impl Iterator<Item=&'l Handle>,
	columns: usize,
) -> Scrollable<'l, Message> {
	let mut columns: Vec<_> = repeat_with(Vec::new).take(columns).collect();
	for (i, handle) in images.enumerate() {
		let image = Image::new(handle.clone()).width(Length::Fill);

		let column = i % columns.len();
		columns[column].push(image.into());
	}

	let images = Row::with_children(
		columns
			.into_iter()
			.map(|i| Column::with_children(i).width(Length::Fill).into())
			.collect(),
	).width(Length::Fill);

	Scrollable::new(images)
}