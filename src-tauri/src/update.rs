use crate::settings::{Settings, UpdateBranch};
use serde::Deserialize;
use tempfile::TempDir;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use reqwest::Url;

const EXE: &str = match cfg!(debug_asserts) {
	false => "Philia.exe",
	true => "Philia-Debug.exe",
};

const ASSET: &str = match cfg!(debug_asserts) {
	false => "Windows-x86_64.zip",
	true => "Windows-x86_64-Debug.zip",
};

pub fn check_for_updates() -> Result<(), Box<dyn Error>> {
	if !cfg!(windows) {
		return Err("Updates not supported on this OS.".into());
	}
	
	if cfg!(debug_assertions) {
		return Ok(());
	}
	
	let settings = serde_json::from_slice::<Settings>(&std::fs::read("./settings.json")?)?;

	let dir = TempDir::new()?;
	let dir = dir.path();
	let zip = dir.join(ASSET);
	
	let do_update = match settings.update_branch {
		UpdateBranch::Stable => fetch_latest_stable(&zip)?,
		UpdateBranch::Nightly => fetch_latest_nightly(&zip)?,
	};
	
	if !do_update {
		return Ok(());
	}
	
	println!("Extracting update files to {:?}...", dir);
	self_update::Extract::from_source(&zip)
		.archive(self_update::ArchiveKind::Zip)
		.extract_file(dir, EXE)?;

	self_update::Move::from_source(&dir.join(EXE))
		.to_dest(&std::env::current_exe()?)?;

	tauri::api::dialog::blocking::message::<tauri::Wry>(
		None, "Success.",
		"The update completed successfully.\nThe application will now close.",
	);

	std::process::exit(0);
}

fn fetch_latest_stable(out_path: &Path) -> Result<bool, Box<dyn Error>> {
	let releases = self_update::backends::github::ReleaseList::configure()
		.repo_owner("MaximumOverflow")
		.repo_name("Philia")
		.build().unwrap()
		.fetch()?;

	let latest = releases.get(0).ok_or("No releases.")?;
	let asset = latest.asset_for(ASSET, None)
		.ok_or("Missing release asset.")?;

	let version = self_update::cargo_crate_version!();
	if !self_update::version::bump_is_greater(&version, &latest.version).unwrap_or(false) {
		return Ok(false);
	}

	if !confirm() {
		return Ok(false);
	}

	let zip_file = File::create(out_path)?;
	
	println!("Downloading the latest stable build...");
	self_update::Download::from_url(&asset.download_url)
		.set_header(reqwest::header::ACCEPT, "application/octet-stream".parse()?)
		.show_progress(true)
		.download_to(zip_file)?;
	
	Ok(true)
}

fn fetch_latest_nightly(out_path: &Path) -> Result<bool, Box<dyn Error>> {
	if !cfg!(windows) {
		return Err("Updates not supported on this OS.".into());
	}

	let client = reqwest::blocking::Client::builder()
		.user_agent(format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")))
		.build()?;

	#[derive(Debug, Deserialize)]
	struct Runs {
		#[serde(default = "Default::default")]
		workflow_runs: Vec<Run>,
	}

	#[derive(Debug, Deserialize)]
	struct Run {
		#[serde(default = "Default::default")]
		id: i64,
		#[serde(default = "Default::default")]
		name: String,
		#[serde(default = "Default::default")]
		status: String,
		#[serde(default = "Default::default")]
		conclusion: Option<String>,
	}

	{
		let url = Url::parse("https://api.github.com/repos/MaximumOverflow/Philia/actions/runs").unwrap();
		let result = client.get(url).send()?;
		let json = result.bytes()?.to_vec();

		let runs = serde_json::from_slice::<Runs>(&json)?.workflow_runs;
		let Some(latest) = runs.into_iter().find(|run| {
			let conclusion = run.conclusion.as_ref().map(String::as_str);
			run.name == "Continuous Build" && run.status == "completed" && conclusion == Some("success")
		}) else {
			return Ok(false);
		};

		println!("BUILD_ID: {}", env!("BUILD_ID"));
		println!("LATEST_BUILD_ID: {}", latest.id);
		if latest.id.to_string() == env!("BUILD_ID") {
			return Ok(false);
		}

		latest
	};
	
	if !confirm() {
		return Ok(false);
	}

	println!("Downloading the latest nightly build...");
	let url = Url::parse("https://nightly.link/MaximumOverflow/Philia/workflows/continuous_build/continous_build/Windows-x86_64.zip").unwrap();
	let result = client.get(url).send()?;
	let bytes = result.bytes()?.to_vec();
	
	println!("Writing update zip to {:?}...", out_path);
	std::fs::write(out_path, bytes)?;
	Ok(true)
}

fn confirm() -> bool {
	tauri::api::dialog::blocking::ask::<tauri::Wry>(
		None, "New update found.",
		"A new version of Philia is available.\nWould you like to download it?",
	)
}