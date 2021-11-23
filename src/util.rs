use directories::ProjectDirs;
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io;
use std::path::{Path, PathBuf};

pub fn get_base_dir() -> Result<PathBuf, Box<dyn Error>> {
	let dir = ProjectDirs::from("eu", "mq1", "runmc").ok_or("Error getting base directory")?;
	let dir = dir.data_dir().to_path_buf();

	Ok(dir)
}

pub fn download_file<S: AsRef<str>, P: AsRef<Path>>(url: S, path: P) -> Result<(), Box<dyn Error>> {
	if path.as_ref().exists() {
		println!("{:?} already present", path.as_ref());
		return Ok(());
	}

	let dir = path.as_ref().parent().ok_or("error getting parent dir")?;
	create_dir_all(dir)?;

	let mut resp = ureq::get(url.as_ref()).call()?.into_reader();
	let mut out = File::create(path.as_ref())?;
	io::copy(&mut resp, &mut out)?;

	println!("downloaded file {} to {:?}", url.as_ref(), path.as_ref());

	Ok(())
}
