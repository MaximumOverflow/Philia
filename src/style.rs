use iced::{application, widget::*, color, Vector, Color, Background};
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::widget::container::Appearance;
use iced::theme::Application;
use iced_aw::*;

#[derive(Default)]
pub struct Theme;

impl application::StyleSheet for Theme {
	type Style = Application;

	//noinspection RsUnnecessaryQualifications
	fn appearance(&self, _: &Self::Style) -> application::Appearance {
		application::Appearance {
			background_color: color!(0x262626),
			text_color: color!(0, 0, 0),
		}
	}
}

impl container::StyleSheet for Theme {
	type Style = ();

	fn appearance(&self, style: &Self::Style) -> Appearance {
		Appearance {
			border_width: 1.0,
			border_color: color!(0x3e3e42),
			background: color!(0x2b2d30).into(),
			..Default::default()
		}
	}
}

#[derive(Default, Copy, Clone)]
pub struct TextInputStyle;

impl text_input::StyleSheet for Theme {
	type Style = TextInputStyle;

	fn active(&self, _: &Self::Style) -> text_input::Appearance {
		text_input::Appearance {
			border_radius: 0.0,
			border_width: 0.0,
			border_color: Default::default(),
			background: color!(0x2b2d30).into(),
		}
	}

	fn focused(&self, _: &Self::Style) -> text_input::Appearance {
		text_input::Appearance {
			border_radius: 2.0,
			border_width: 1.0,
			background: color!(0x2b2d30).into(),
			border_color: color!(0x3574f0).into(),
		}
	}

	fn placeholder_color(&self, _: &Self::Style) -> Color {
		color!(0x6f747c)
	}

	fn value_color(&self, _: &Self::Style) -> Color {
		color!(0xdddddd)
	}

	fn selection_color(&self, _: &Self::Style) -> Color {
		color!(0x3574f0)
	}
}

#[derive(Default, Copy, Clone)]
pub enum ButtonStyle {
	#[default]
	Default,
	IncludeTag,
	ExcludeTag,
	Transparent,
}

impl button::StyleSheet for Theme {
	type Style = ButtonStyle;

	fn active(&self, style: &Self::Style) -> button::Appearance {
		match style {
			ButtonStyle::Default => button::Appearance {
				border_radius: 20.0,
				text_color: color!(0xFFFFFF),
				background: color!(0x366ace).into(),
				shadow_offset: Vector::new(2.5, 2.5),
				..Default::default()
			},
			ButtonStyle::IncludeTag => button::Appearance {
				border_radius: 20.0,
				text_color: color!(0xFFFFFF),
				background: color!(0x2D882D).into(),
				shadow_offset: Vector::new(2.5, 2.5),
				..Default::default()
			},
			ButtonStyle::ExcludeTag => button::Appearance {
				border_radius: 20.0,
				text_color: color!(0xFFFFFF),
				background: color!(0xAA3939).into(),
				shadow_offset: Vector::new(2.5, 2.5),
				..Default::default()
			},
			ButtonStyle::Transparent => button::Appearance {
				border_color: color!(0, 0, 0, 0.0),
				..Default::default()
			},
		}
	}

	fn hovered(&self, style: &Self::Style) -> button::Appearance {
		let mut appearance = self.active(style);
		if let Some(Background::Color(color)) = &mut appearance.background {
			color.r += 0.05;
			color.g += 0.05;
			color.b += 0.05;
		}

		appearance
	}

	fn pressed(&self, style: &Self::Style) -> button::Appearance {
		let mut appearance = self.active(style);
		if let Some(Background::Color(color)) = &mut appearance.background {
			color.r -= 0.05;
			color.g -= 0.05;
			color.b -= 0.05;
		}

		appearance
	}

	fn disabled(&self, style: &Self::Style) -> button::Appearance {
		let mut appearance = self.active(style);
		if let Some(Background::Color(color)) = &mut appearance.background {
			color.a -= 0.3;
		}

		appearance
	}
}

#[derive(Default, Copy, Clone)]
pub enum TextStyle {
	#[default]
	White,
	Black,
}

impl text::StyleSheet for Theme {
	type Style = TextStyle;

	fn appearance(&self, style: Self::Style) -> text::Appearance {
		match style {
			TextStyle::Black => text::Appearance {
				color: color!(0, 0, 0).into(),
			},
			TextStyle::White => text::Appearance {
				color: color!(255, 255, 255).into(),
			},
		}
	}
}

#[derive(Default, Copy, Clone)]
pub struct ScrollableStyle;

impl scrollable::StyleSheet for Theme {
	type Style = ScrollableStyle;

	fn active(&self, _: &Self::Style) -> Scrollbar {
		Scrollbar {
			background: None,
			border_radius: 0.0,
			border_width: 1.0,
			border_color: Default::default(),
			scroller: Scroller {
				color: color!(0x3e3e42),
				border_radius: 5.0,
				border_width: 0.0,
				border_color: Default::default(),
			},
		}
	}

	fn hovered(&self, _: &Self::Style) -> Scrollbar {
		Scrollbar {
			background: None,
			border_radius: 0.0,
			border_width: 1.0,
			border_color: Default::default(),
			scroller: Scroller {
				color: color!(0x616168),
				border_radius: 5.0,
				border_width: 0.0,
				border_color: Default::default(),
			},
		}
	}
}

#[derive(Default, Copy, Clone)]
pub struct PickListStyle;

impl pick_list::StyleSheet for Theme {
	type Style = PickListStyle;

	fn active(&self, style: &Self::Style) -> pick_list::Appearance {
		pick_list::Appearance {
			text_color: color!(0xffffff),
			placeholder_color: color!(0xdddddd),
			handle_color: color!(0xffffff),
			background: color!(0x366ace).into(),
			border_radius: 20.0,
			border_width: 0.0,
			border_color: Default::default(),
		}
	}

	fn hovered(&self, style: &Self::Style) -> pick_list::Appearance {
		pick_list::Appearance {
			text_color: color!(0xffffff),
			placeholder_color: color!(0xdddddd),
			handle_color: color!(0xffffff),
			background: color!(0x305fb9).into(),
			border_radius: 20.0,
			border_width: 0.0,
			border_color: Default::default(),
		}
	}
}

#[derive(Default, Copy, Clone)]
pub struct MenuStyle;

impl iced_native::overlay::menu::StyleSheet for Theme {
	type Style = MenuStyle;

	fn appearance(&self, style: &Self::Style) -> iced::overlay::menu::Appearance {
		iced::overlay::menu::Appearance {
			text_color: color!(0xdddddd),
			background: color!(0x366ace).into(),
			border_width: 0.0,
			border_radius: 0.0,
			border_color: Default::default(),
			selected_text_color: color!(0xffffff),
			selected_background: color!(0x366ace).into(),
		}
	}
}

impl From<PickListStyle> for MenuStyle {
	fn from(_: PickListStyle) -> Self {
		Self
	}
}

#[derive(Default, Copy, Clone)]
pub struct TogglerStyle;

impl toggler::StyleSheet for Theme {
	type Style = TogglerStyle;

	fn active(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
		toggler::Appearance {
			background: match is_active {
				true => color!(0x366ace),
				false => color!(0x305fb9),
			},
			background_border: None,
			foreground: color!(0xffffff),
			foreground_border: None,
		}
	}

	fn hovered(&self, style: &Self::Style, is_active: bool) -> toggler::Appearance {
		self.active(style, is_active)
	}
}

#[derive(Default, Copy, Clone)]
pub struct NumberInputStyle;

impl number_input::StyleSheet for Theme {
	type Style = NumberInputStyle;

	fn active(&self, style: Self::Style) -> number_input::Appearance {
		number_input::Appearance {
			button_background: None,
			icon_color: color!(0xffffff),
		}
	}
}

#[derive(Default, Copy, Clone)]
pub struct SplitStyle;

impl split::StyleSheet for Theme {
	type Style = SplitStyle;

	fn active(&self, style: Self::Style) -> split::Appearance {
		split::Appearance {
			background: None,
			first_background: None,
			second_background: None,
			border_width: 0.0,
			border_color: color!(0x303030).into(),
			divider_background: color!(0x3e3e42).into(),
			divider_border_width: 0.0,
			divider_border_color: Default::default(),
		}
	}

	fn hovered(&self, style: Self::Style) -> split::Appearance {
		self.active(style)
	}

	fn dragged(&self, style: Self::Style) -> split::Appearance {
		self.active(style)
	}
}

#[derive(Default, Copy, Clone)]
pub struct CardStyle;

impl card::StyleSheet for Theme {
	type Style = CardStyle;

	fn active(&self, style: Self::Style) -> card::Appearance {
		card::Appearance {
			border_width: 0.0,
			border_radius: 0.0,
			border_color: Default::default(),
			background: color!(0x2b2d30).into(),
			body_background: color!(0x565656).into(),
			foot_background: color!(0x2b2d30).into(),
			head_background: color!(0x366ace).into(),
			head_text_color: color!(0xffffff),
			body_text_color: color!(0xffffff),
			foot_text_color: color!(0xffffff),
			close_color: color!(0xffffff),
		}
	}
}

#[derive(Default, Copy, Clone)]
pub struct ModalStyle;

impl modal::StyleSheet for Theme {
	type Style = ModalStyle;

	fn active(&self, style: Self::Style) -> style::modal::Appearance {
		style::modal::Appearance {
			background: color!(48, 48, 48, 0.3).into(),
		}
	}
}