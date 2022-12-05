#![windows_subsystem = "windows"]

use iced::{Application, Settings};
use crate::application::Philia;

mod application;
mod download;
mod search;
mod gui;

fn main() {
	Philia::run(Settings::default()).unwrap()
}
