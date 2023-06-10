use tempfile::TempDir;
use std::error::Error;
use std::fs::File;

pub fn check_for_updates() -> Result<(), Box<dyn Error>> {
	if !cfg!(windows) {
		return Err("Updates not supported on this OS.".into());
	}
	
	const ASSET: &str = match cfg!(debug_asserts) {
		false => "x86_64-pc-windows-gnu.zip",
		true => "x86_64-pc-windows-gnu_debug.zip",
	};

	const EXE: &str = match cfg!(debug_asserts) {
		false => "Philia.exe",
		true => "Philia-Debug.exe",
	};

	let releases = self_update::backends::github::ReleaseList::configure()
		.repo_owner("MaximumOverflow")
		.repo_name("Philia-GUI")
		.build().unwrap()
		.fetch()?;

	let latest = releases.get(0).ok_or("No releases.")?;
	let asset = latest.asset_for(ASSET, None)
		.ok_or("Missing release asset.")?;

	let version = self_update::cargo_crate_version!();
	if !self_update::version::bump_is_greater(&version, &latest.version).unwrap_or(false) {
		return Ok(());
	}

	let confirm = tauri::api::dialog::blocking::ask::<tauri::Wry>(
		None, "New update found.", 
		"A new version of Philia is available.\nWould you like to download it?",
	);

	if !confirm {
		return Ok(());
	}

	let dir = TempDir::new()?;
	let zip = dir.path().join(&asset.name);
	let zip_file = File::create(&zip)?;
	self_update::Download::from_url(&asset.download_url)
		.set_header(tauri::http::header::ACCEPT, "application/octet-stream".parse()?)
		.show_progress(true)
		.download_to(zip_file)?;

	self_update::Extract::from_source(&zip)
		.archive(self_update::ArchiveKind::Zip)
		.extract_file(&dir.path(), EXE)?;

	self_update::Move::from_source(&dir.path().join(EXE))
		.to_dest(&std::env::current_exe()?)?;

	tauri::api::dialog::blocking::message::<tauri::Wry>(
		None, "Success.",
		"The update completed successfully.\nThe application will now close.",
	);

	std::process::exit(0);
}