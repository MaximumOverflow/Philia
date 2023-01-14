use iced::widget::{Button, Column, Image, Row, Scrollable};
use philia::prelude::GenericPost;
use crate::application::Message;
use iced_native::image::Handle;
use std::iter::repeat_with;
use iced::{Length, Padding};

pub fn post_image_list<'l>(
	images: impl Iterator<Item = &'l (GenericPost, Handle)>,
	columns: usize,
) -> Scrollable<'l, Message> {
	let mut columns: Vec<_> = repeat_with(Vec::new).take(columns).collect();
	for (i, (GenericPost { id, .. }, handle)) in images.enumerate() {
		let image = Image::new(handle.clone()).width(Length::Fill);

		let image = Button::new(image)
			.on_press(Message::ShowPostWithId(*id))
			.padding(Padding::new(0));

		let column = i % columns.len();
		columns[column].push(image.into());
	}

	let images = Row::with_children(
		columns
			.into_iter()
			.map(|i| Column::with_children(i).width(Length::Fill).into())
			.collect(),
	)
	.width(Length::FillPortion(2));

	Scrollable::new(images)
}
