use iced::widget::{Scrollable, Text, row, column, Button, PickList, Column, Row, Image, Tooltip};
use crate::application::{Element, Message, Philia, Source};
use crate::search::{SearchMessage, SearchStatus, Sorting};
use crate::download::{DownloadContext, DownloadMessage};
use iced_native::widget::tooltip::Position;
use crate::preview::PostPreviewMessage;
use iced::{Alignment, Length, Padding};
use crate::settings::SettingsMessage;
use std::iter::{repeat, repeat_with};
use crate::style::ButtonStyle;
use strum::IntoEnumIterator;
use iced_aw::NumberInput;
use std::ops::Deref;

pub fn post_list(context: &Philia) -> Element {
	let page: Element = NumberInput::new(context.search.page, usize::MAX, |value| {
		SearchMessage::PageChanged(value).into()
	})
	.min(1)
	.into();

	let per_page: Element = NumberInput::new(context.search.per_page, 320, |value| {
		SearchMessage::PerPageChanged(value).into()
	})
	.min(1)
	.into();

	let search: Element = match context.search.status {
		SearchStatus::Complete => Button::new(Text::new("Search"))
			.on_press(SearchMessage::SearchRequested.into())
			.into(),

		SearchStatus::Searching => Button::new(Text::new("Searching..."))
			.into(),

		SearchStatus::LoadingPosts { loaded, total } => {
			let text = format!("Loading posts... {} / {}", loaded, total);
			let button: Element = Button::new(Text::new(text))
				.on_press(SearchMessage::SearchCanceled.into())
				.style(ButtonStyle::Cancellable)
				.into();
			
			Tooltip::new(button, "Cancel search", Position::Bottom)
				.into()
		}
	};

	let settings: Element = Button::new(Text::new("Settings"))
		.on_press(SettingsMessage::SettingsOpened.into())
		.into();

	let download: Element = match context.download {
		DownloadContext::Complete if context.search.status == SearchStatus::Complete => {
			Button::new(Text::new("Download all"))
				.on_press(DownloadMessage::DownloadRequested(context.search.results.clone()).into())
				.into()
		}

		DownloadContext::Downloading { total, downloaded } => {
			let text = format!("Downloading... {} / {}", downloaded, total);
			Button::new(Text::new(text)).into()
		}

		_ => Button::new(Text::new("Download all")).into(),
	};

	let source: Element = PickList::new(Source::iter().collect::<Vec<_>>(), Some(context.source), |source| {
		Message::SourceChanged(source)
	}).into();

	let sorting: Element =
		PickList::new(Sorting::iter().collect::<Vec<_>>(), Some(context.search.sorting), |sorting| {
			SearchMessage::SortingChanged(sorting).into()
		}).into();

	let search_bar: Element = row![
		Into::<Element>::into(Text::new("Page: ")),
		page,
		Into::<Element>::into(Text::new("Posts per page: ")),
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
		for (i, post) in posts.iter().enumerate().filter(|(_, post)| post.preview.is_some()) {
			let smallest = column_sizes.iter().min().unwrap();
			let smallest = column_sizes.iter().position(|i| *i == *smallest).unwrap();

			let image = Image::new(post.preview.clone().unwrap());
			let button = Button::new(image)
				.on_press(PostPreviewMessage::PostPreviewOpened(i).into())
				.style(ButtonStyle::Transparent)
				.into();

			columns[smallest].push(button);
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

	let posts: Element = Scrollable::new(list).into();

	column![search_bar, posts].align_items(Alignment::Center).into()
}
