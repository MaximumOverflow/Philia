use iced::widget::{Button, Container, Text, TextInput, column, Scrollable, Column};
use crate::style::{ButtonStyle, ScrollableStyle, TextInputStyle, TextStyle};
use crate::tags::{TagSelectorContext, TagSelectorMessage};
use crate::application::{Element, Philia};
use iced::{Alignment, Length, Padding};
use iced_native::{row, Widget};
use iced_native::widget::Row;

pub fn tag_selector(context: &Philia) -> Element {
	match &context.tag_selector {
		TagSelectorContext::New => Button::new(Text::new("Load tag list").style(TextStyle::White))
			.on_press(TagSelectorMessage::ReloadRequested.into())
			.style(ButtonStyle::Default)
			.into(),

		TagSelectorContext::LoadingTagList => Container::new("Loading tag list...").center_x().center_y().into(),

		TagSelectorContext::ShowTagSelector {
			search,
			search_timestamp,
			shown_tags,
			..
		} => {
			let search_bar: Element = TextInput::new("Search tags", &search, |search| {
				TagSelectorMessage::SearchChanged(search).into()
			})
			.style(TextInputStyle)
			.into();

			match search_timestamp {
				Some(_) => {
					let search_text: Element = Text::new("Searching tags...").style(TextStyle::White).into();

					column![search_bar, search_text].into()
				}

				None => {
					let search: Element = {
						let list = shown_tags
							.iter()
							.map(|tag| {
								Row::with_children(vec![
									Text::new(tag).style(TextStyle::White).into(),
									Button::new(Text::new("Include").style(TextStyle::White))
										.on_press(TagSelectorMessage::TagIncluded(tag.clone()).into())
										.style(ButtonStyle::IncludeTag)
										.into(),
									Button::new(Text::new("Exclude").style(TextStyle::White))
										.on_press(TagSelectorMessage::TagExcluded(tag.clone()).into())
										.style(ButtonStyle::ExcludeTag)
										.into(),
								])
								.align_items(Alignment::Center)
								.spacing(8)
								.into()
							})
							.collect::<Vec<Element>>();

						let content = Column::with_children(list).width(Length::Fill).spacing(5);

						let scroller: Element = Scrollable::new(content).style(ScrollableStyle).into();

						let title: Element = Text::new("Search results:").style(TextStyle::White).into();

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
								Button::new(Text::new(tag).style(TextStyle::White))
									.on_press(TagSelectorMessage::TagIgnored(tag.clone()).into())
									.style(ButtonStyle::IncludeTag)
									.into()
							})
							.collect::<Vec<Element>>();

						let content = Column::with_children(list).width(Length::Fill).spacing(5);

						let scroller: Element = Scrollable::new(content).style(ScrollableStyle).into();

						let title: Element = Text::new("Included:").style(TextStyle::White).into();

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
								Button::new(Text::new(tag).style(TextStyle::White))
									.on_press(TagSelectorMessage::TagIgnored(tag.clone()).into())
									.style(ButtonStyle::ExcludeTag)
									.into()
							})
							.collect::<Vec<Element>>();

						let content = Column::with_children(list).width(Length::Fill).spacing(5);

						let scroller: Element = Scrollable::new(content).style(ScrollableStyle).into();

						let title: Element = Text::new("Excluded:").style(TextStyle::White).into();

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
	}
}
