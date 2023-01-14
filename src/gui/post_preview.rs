use iced::widget::{Button, Container, Image, Scrollable, Text};
use iced_native::alignment::Horizontal;
use iced_native::widget::{Column, Row};
use philia::prelude::{GenericPost, Post};
use crate::application::Message;
use iced_native::image::Handle;
use iced::alignment::Vertical;
use iced::{Element, Length};

#[derive(Debug, Default)]
pub enum PostPreview {
	#[default]
	None,
	Loading,
	Loaded {
		handle: Handle,
		post: GenericPost,
	},
}

pub fn post_preview(preview: &PostPreview) -> Option<Element<'_, Message>> {
	match preview {
		PostPreview::None => None,
		PostPreview::Loading => {
			let text = Text::new("Loading...")
				.height(Length::Fill)
				.width(Length::FillPortion(1))
				.vertical_alignment(Vertical::Center)
				.horizontal_alignment(Horizontal::Center);

			Some(text.into())
		}
		PostPreview::Loaded { handle, post } => {
			let image = Image::new(handle.clone())
				.width(Length::Fill)
				.height(Length::FillPortion(7))
				.into();

			let tags_string = post.tags_owned().join(", ");
			let tags = Scrollable::new(Text::new(tags_string.clone()));
			let tags = Container::new(tags).height(Length::FillPortion(3)).into();

			fn make_button(text: &str, msg: Message) -> Element<'_, Message> {
				let text = Text::new(text).horizontal_alignment(Horizontal::Center);
				let button = Button::new(text).width(Length::Fill).on_press(msg);
				button.into()
			}

			let buttons = {
				let close = make_button("Close", Message::HidePost);
				let copy_tags = make_button("Copy tags", Message::CopyTags(tags_string));
				let download = make_button("Download", Message::DownloadPreview(handle.clone()));
				Row::with_children(vec![copy_tags, download, close]).into()
			};

			let column = Column::with_children(vec![image, tags, buttons])
				.spacing(8)
				.width(Length::FillPortion(1));

			Some(column.into())
		}
	}
}
