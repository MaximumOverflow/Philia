use iced::widget::{Scrollable, Text, row, column, Button, PickList, Column, Row, Image, Tooltip, Container, Checkbox};
use crate::search::{PostPreview, SearchMessage, SearchStatus, Sorting};
use crate::download::{DownloadContext, DownloadMessage};
use crate::application::{Element, Message, Philia};
use iced_aw::{FloatingElement, NumberInput};
use iced_native::widget::tooltip::Position;
use crate::post_viewer::PostViewerMessage;
use iced::{Alignment, Length, Padding};
use iced_native::alignment::Horizontal;
use crate::settings::SettingsMessage;
use std::iter::{repeat, repeat_with};
use crate::style::ButtonStyle;
use strum::IntoEnumIterator;
use std::ops::Deref;

pub fn post_list(context: &Philia) -> Element {
	let client = context.client.upgrade();
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
		SearchStatus::Complete => match &client {
			None => Button::new(Text::new("Search")).into(),

			Some(_) => Button::new(Text::new("Search"))
				.on_press(SearchMessage::SearchRequested.into())
				.into(),
		},

		SearchStatus::Searching => Button::new(Text::new("Searching...")).into(),

		SearchStatus::LoadingPosts { loaded, total } => {
			let text = format!("Loading posts... {} / {}", loaded, total);
			let button: Element = Button::new(Text::new(text))
				.on_press(SearchMessage::SearchCanceled.into())
				.style(ButtonStyle::Cancellable)
				.into();

			Tooltip::new(button, "Stop loading previews", Position::Top).into()
		}
	};

	let settings: Element = Button::new(Text::new("Settings"))
		.on_press(SettingsMessage::SettingsOpened.into())
		.into();

	let download: Element = match context.download {
		DownloadContext::Complete if context.search.status == SearchStatus::Complete && client.is_some() => {
			match context.search.selected.is_empty() {
				true => {
					Button::new(Text::new("Download all"))
						.on_press(DownloadMessage::DownloadRequested(context.search.results.clone()).into())
						.into()
				},
				
				false => {
					Button::new(Text::new("Download selected"))
						.on_press(DownloadMessage::FilteredDownloadRequested.into())
						.into()
				},
			}
		}

		DownloadContext::Downloading { total, downloaded, .. } => {
			let text = format!("Downloading... {} / {}", downloaded, total);
			let button: Element = Button::new(Text::new(text))
				.on_press(DownloadMessage::DownloadCanceled.into())
				.style(ButtonStyle::Cancellable)
				.into();

			Tooltip::new(button, "Cancel download", Position::Top).into()
		}

		_ => Button::new(Text::new("Download all")).into(),
	};

	let source = {
		let options: Vec<_> = context
			.clients
			.iter()
			.map(|client| client.source().name.clone())
			.collect();

		let selected = client.as_ref().map(|client| client.source().name.clone());

		PickList::new(options, selected, Message::SourceChanged)
	};

	let sorting: Element =
		PickList::new(Sorting::iter().collect::<Vec<_>>(), Some(context.search.sorting), |sorting| {
			SearchMessage::SortingChanged(sorting).into()
		})
		.into();

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
	.padding(Padding::new(8.0))
	.into();

	let list: Element = {
		const COLUMNS: usize = 6;
		let mut column_sizes: Vec<u32> = repeat(0).take(COLUMNS).collect();
		let mut columns: Vec<_> = repeat_with(Vec::new).take(COLUMNS).collect();

		let posts = context.search.results.deref();
		for (i, post) in posts.iter().enumerate() {
			let smallest = column_sizes.iter().min().unwrap();
			let smallest = column_sizes.iter().position(|i| *i == *smallest).unwrap();

			let preview: Element = match post.preview.clone() {
				PostPreview::Loaded(handle) => {
					column_sizes[smallest] += post.size.1;
					let image: Element = Image::new(handle).into();
					let selected = context.search.selected.contains(&i);

					FloatingElement::new(image, move || {
						Checkbox::new("", selected, move |v| match v {
							true => SearchMessage::PostSelected(i).into(),
							false => SearchMessage::PostDeselected(i).into(),
						}).into()
					}).into()
				}

				PostPreview::Pending => {
					column_sizes[smallest] += 420;

					let text =
						Text::new(format!("Loading post\n{}", post.info.id)).horizontal_alignment(Horizontal::Center);

					Container::new(text)
						.height(Length::Fixed(420.0))
						.width(Length::Fixed(512.0))
						.center_x()
						.center_y()
						.into()
				}

				PostPreview::Failed => {
					column_sizes[smallest] += 420;

					let text = Text::new(format!("Could not load post\n{}", post.info.id))
						.horizontal_alignment(Horizontal::Center);

					Container::new(text)
						.height(Length::Fixed(420.0))
						.width(Length::Fixed(512.0))
						.center_x()
						.center_y()
						.into()
				}
			};

			let button = Button::new(preview)
				.on_press(PostViewerMessage::Opened(i).into())
				.style(ButtonStyle::Transparent)
				.into();

			columns[smallest].push(button);
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
