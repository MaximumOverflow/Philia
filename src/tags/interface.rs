use iced::widget::{Button, Container, Text, TextInput, column, Scrollable, Column};
use crate::tags::{TagSelectorContext, TagSelectorMessage};
use crate::application::{Element, Philia};
use iced::{Alignment, Length, Padding};
use iced::alignment::Horizontal;
use crate::style::ButtonStyle;
use iced_native::widget::Row;
use iced_native::row;

pub fn tag_selector(context: &Philia) -> Element {
	match &context.tag_selector {
		TagSelectorContext::New => {
			let content: Element = match context.client.upgrade() {
				None => Text::new("The tag list is not available for this source.")
					.horizontal_alignment(Horizontal::Center)
					.into(),

				Some(client) => {
					let text: Element = Text::new(match client.source().tag_list.is_some() {
						true => concat! {
							"The tag list for this source has not been downloaded yet.\n",
							"Would you like to download it? This process may take a while."
						},
						false => concat! {
							"The tag list for this source has not been created yet.\n",
							"Tags for this source cannot be downloaded automatically.\n",
							"Would you like to create an empty tag list?"
						},
					})
					.horizontal_alignment(Horizontal::Center)
					.into();

					let button: Element = Button::new(Text::new(match client.source().tag_list.is_some() {
						true => "Download tag list",
						false => "Create tag list",
					}))
					.on_press(TagSelectorMessage::DownloadRequested.into())
					.into();

					Column::with_children(vec![text, button])
						.spacing(16)
						.align_items(Alignment::Center)
						.into()
				}
			};

			Container::new(content)
				.width(Length::Fill)
				.height(Length::Fill)
				.center_x()
				.center_y()
				.into()
		}

		TagSelectorContext::LoadingTagList { page, tags } => Container::new(
			Text::new(format! {
				"Downloading tag list...\nTotal tags: {}\nPage: {} / 50",
				tags.len(), page,
			})
			.horizontal_alignment(Horizontal::Center),
		)
		.width(Length::Fill)
		.height(Length::Fill)
		.center_x()
		.center_y()
		.into(),

		TagSelectorContext::ShowTagSelector { search, shown_tags, .. } => {
			let search_bar: Element =
				TextInput::new("Search tags", search, |search| TagSelectorMessage::SearchChanged(search).into()).into();

			let search: Element = {
				let mut list = shown_tags
					.iter()
					.map(|tag| {
						Row::with_children(vec![
							Text::new(tag).into(),
							Button::new(Text::new("Include"))
								.on_press(TagSelectorMessage::TagIncluded(tag.clone()).into())
								.style(ButtonStyle::IncludeTag)
								.into(),
							Button::new(Text::new("Exclude"))
								.on_press(TagSelectorMessage::TagExcluded(tag.clone()).into())
								.style(ButtonStyle::ExcludeTag)
								.into(),
						])
						.align_items(Alignment::Center)
						.spacing(8)
						.into()
					})
					.collect::<Vec<Element>>();

				if !search.is_empty() && !shown_tags.contains(search) {
					list.push(
						Row::with_children(vec![
							Text::new(search.clone()).into(),
							Button::new(Text::new("Create"))
								.on_press(TagSelectorMessage::TagCreated(search.clone()).into())
								.style(ButtonStyle::IgnoreTag)
								.into(),
						])
						.align_items(Alignment::Center)
						.spacing(8)
						.into(),
					)
				}

				let content = Column::with_children(list).width(Length::Fill).spacing(5);
				let scroller: Element = Scrollable::new(content).into();
				let title: Element = Text::new("Search results:").into();

				column![title, scroller]
					.width(Length::FillPortion(6))
					.spacing(16)
					.into()
			};

			let included: Element = {
				let list = context
					.search
					.include
					.iter()
					.map(|tag| {
						Button::new(Text::new(tag))
							.on_press(TagSelectorMessage::TagIgnored(tag.clone()).into())
							.style(ButtonStyle::IncludeTag)
							.into()
					})
					.collect::<Vec<Element>>();

				let content = Column::with_children(list).width(Length::Fill).spacing(5);

				let scroller: Element = Scrollable::new(content).into();

				let title: Element = Text::new("Included:").into();

				column![title, scroller]
					.width(Length::FillPortion(2))
					.spacing(16)
					.into()
			};

			let excluded: Element = {
				let list = context
					.search
					.exclude
					.iter()
					.map(|tag| {
						Button::new(Text::new(tag))
							.on_press(TagSelectorMessage::TagIgnored(tag.clone()).into())
							.style(ButtonStyle::ExcludeTag)
							.into()
					})
					.collect::<Vec<Element>>();

				let content = Column::with_children(list).width(Length::Fill).spacing(5);

				let scroller: Element = Scrollable::new(content).into();

				let title: Element = Text::new("Excluded:").into();

				column![title, scroller]
					.width(Length::FillPortion(2))
					.spacing(16)
					.into()
			};

			let selector: Element = row![search, included, excluded]
				.padding(Padding {
					top: 8,
					right: 8,
					bottom: 8,
					left: 8,
				})
				.spacing(8)
				.into();

			column![search_bar, selector,].into()
		}
	}
}
