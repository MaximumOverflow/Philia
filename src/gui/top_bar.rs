use std::str::FromStr;
use iced::Length;
use iced::widget::{Button, PickList, Row, Text, TextInput};
use crate::application::Message;
use crate::download::DownloadProgress;
use crate::search::{SearchParameters, SearchProgress, Source};

pub fn tob_bar<'l>(
	search_params: &'l SearchParameters,
	search_progress: &'l SearchProgress,
	download_progress: &'l DownloadProgress,
) -> Row<'l, Message> {
	let can_search = *search_progress == SearchProgress::Complete && search_params.count != 0;

	let search_query = {
		let search_query = TextInput::new(
			"Enter tags to search",
			&search_params.tags,
			Message::SearchQueryChanged,
		);

		match can_search {
			false => search_query,
			true => search_query.on_submit(Message::SearchRequested),
		}.into()
	};

	let search_count = {
		let value = format!("{}", search_params.count);

		let search_count = TextInput::new("Count", &value, |value| {
			Message::SearchCountChanged(usize::from_str(&value).ok())
		}).width(Length::Units(64));

		match can_search {
			false => search_count,
			true => search_count.on_submit(Message::SearchRequested),
		}.into()
	};

	let search_source = PickList::new(
		vec![Source::E621, Source::Rule34, Source::Danbooru],
		Some(search_params.source),
		Message::SearchSourceChanged,
	)
		.into();

	let search_button = match search_progress {
		SearchProgress::Complete => match can_search {
			false => Button::new("Search"),
			true => Button::new("Search").on_press(Message::SearchRequested),
		},
		SearchProgress::Searching => Button::new("Searching"),
		SearchProgress::LoadingPosts { loaded, total } => {
			Button::new(Text::new(format!("Loaded {} posts of {}", loaded, total)))
		}
	}
		.into();

	let download_button = match download_progress {
		DownloadProgress::DownloadingPosts { downloaded, total } => {
			let text = format!("Downloaded {} of {}", downloaded, total);
			Button::new(Text::new(text))
		}

		DownloadProgress::Complete => {
			if can_search {
				Button::new("Download All").on_press(Message::DownloadPosts)
			} else {
				Button::new("Download All")
			}
		}
	}
		.into();

	let search = Row::with_children(vec![
		search_query,
		search_count,
		search_source,
		search_button,
		download_button,
	]).spacing(4);
	
	search
}