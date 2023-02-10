use std::iter::{repeat, repeat_with};
use std::ops::Deref;
use iced::{Alignment, Length, Padding};
use iced::widget::{Scrollable, Text, row, column, Button, PickList, Column, Row, Image};
use iced_aw::NumberInput;
use strum::IntoEnumIterator;
use crate::application::{Element, Message, Philia, Source};
use crate::download::{DownloadContext, DownloadMessage};
use crate::search::{SearchMessage, SearchStatus, Sorting};
use crate::settings::SettingsMessage;
use crate::style::{ButtonStyle, NumberInputStyle, PickListStyle, ScrollableStyle, TextStyle};

pub fn post_list(context: &Philia) -> Element {
	let page: Element = NumberInput::new(context.search.page, usize::MAX, |value| {
		SearchMessage::PageChanged(value).into()
	})
	.min(1)
	.style(NumberInputStyle)
	.into();

	let per_page: Element = NumberInput::new(context.search.per_page, 320, |value| {
		SearchMessage::PerPageChanged(value).into()
	})
	.min(1)
	.style(NumberInputStyle)
	.into();

	let search: Element = match context.search.status {
		SearchStatus::Complete => Button::new(Text::new("Search").style(TextStyle::White))
			.on_press(SearchMessage::SearchRequested.into())
			.style(ButtonStyle::Default)
			.into(),

		SearchStatus::Searching => Button::new(Text::new("Searching...").style(TextStyle::White))
			.style(ButtonStyle::Default)
			.into(),

		SearchStatus::LoadingPosts { loaded, total } => {
			let text = format!("Loading posts... {} / {}", loaded, total);
			Button::new(Text::new(text).style(TextStyle::White))
				.style(ButtonStyle::Default)
				.into()
		}
	};

	let settings: Element = Button::new(Text::new("Settings").style(TextStyle::White))
		.on_press(SettingsMessage::SettingsOpened.into())
		.style(ButtonStyle::Default)
		.into();

	let download: Element = match context.download {
		DownloadContext::Complete if context.search.status == SearchStatus::Complete => {
			Button::new(Text::new("Download all").style(TextStyle::White))
				.on_press(DownloadMessage::DownloadRequested(context.search.results.clone()).into())
				.style(ButtonStyle::Default)
				.into()
		}

		DownloadContext::Downloading { total, downloaded } => {
			let text = format!("Downloading... {} / {}", downloaded, total);
			Button::new(Text::new(text).style(TextStyle::White))
				.style(ButtonStyle::Default)
				.into()
		}

		_ => Button::new(Text::new("Download all").style(TextStyle::White))
			.style(ButtonStyle::Default)
			.into(),
	};

	let source: Element = PickList::new(Source::iter().collect::<Vec<_>>(), Some(context.source), |source| {
		Message::SourceChanged(source)
	})
	.style(PickListStyle)
	.into();

	let sorting: Element =
		PickList::new(Sorting::iter().collect::<Vec<_>>(), Some(context.search.sorting), |sorting| {
			SearchMessage::SortingChanged(sorting).into()
		})
		.style(PickListStyle)
		.into();

	let search_bar: Element = row![
		Into::<Element>::into(Text::new("Page: ").style(TextStyle::White)),
		page,
		Into::<Element>::into(Text::new("Posts per page: ").style(TextStyle::White)),
		per_page,
		source,
		sorting,
		search,
		download,
		settings,
	]
	.spacing(8)
	.align_items(Alignment::Center)
	.padding(Padding::new(8))
	.into();

	let list: Element = {
		const COLUMNS: usize = 6;
		let mut column_sizes: Vec<u32> = repeat(0).take(COLUMNS).collect();
		let mut columns: Vec<_> = repeat_with(Vec::new).take(COLUMNS).collect();

		let posts = context.search.results.deref();
		for post in posts.iter().filter(|post| post.preview.is_some()) {
			let smallest = column_sizes.iter().min().unwrap();
			let smallest = column_sizes.iter().position(|i| *i == *smallest).unwrap();
			columns[smallest].push(Image::new(post.preview.clone().unwrap()).into());
			column_sizes[smallest] += post.size.1;
		}

		Row::with_children(
			columns
				.into_iter()
				.map(|i| Column::with_children(i).width(Length::Fill).into())
				.collect(),
		)
		.into()
	};

	let posts: Element = Scrollable::new(list).style(ScrollableStyle).into();

	column![search_bar, posts].align_items(Alignment::Center).into()
}
