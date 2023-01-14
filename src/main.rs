// #![windows_subsystem = "windows"]

use crate::application::Philia;
use iced::{Application, Settings};

mod gui;
mod search;
mod download;
mod application;

fn main() {
	Philia::run(Settings::default()).unwrap()
}
