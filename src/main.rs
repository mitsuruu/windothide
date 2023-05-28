use std::{env::current_dir, fs, io, os::windows::fs::MetadataExt, path::Path};

use windows::{
	core::PCSTR,
	Win32::Storage::FileSystem::{self, SetFileAttributesA},
};

fn main()
{
	let working_dir: String = get_working_directory();
	println!("Hiding all files starting with a `.` in {}", working_dir);

	let paths: Vec<String> = fs::read_dir(working_dir)
		.unwrap()
		.filter_map(|e| e.ok())
		.map(|e| e.path().to_string_lossy().into_owned())
		.filter_map(|e| does_file_start_with_dot(e))
		.collect::<Vec<String>>();

	for path in paths
	{
		let file_hidden = is_file_hidden(&path);
		match file_hidden
		{
			Ok(hidden) => hide_file(hidden, &path),
			Err(_) => println!("File {} is not hidden but should be", path),
		}
	}
}

fn get_working_directory() -> String
{
	let path = current_dir();
	match path
	{
		Ok(path) => path.into_os_string().into_string().unwrap(),
		Err(_) => "FAILED".to_string(),
	}
}

fn is_file_hidden(file: &str) -> io::Result<bool>
{
	let metadata = fs::metadata(file)?;
	let attributes = metadata.file_attributes();
	let hidden: bool = attributes & 2 == 2; // if only there were an easier way for this to be implemented that wouldn't require a bunch of manual work from me
	Ok(hidden)
}

fn does_file_start_with_dot(path: String) -> Option<String>
{
	match Path::new(&path)
		.file_name()
		.unwrap()
		.to_os_string()
		.into_string()
		.unwrap()
		.starts_with(".")
	{
		true => Some(path),
		false => None,
	}
}

fn hide_file(hidden: bool, file: &str)
{
	if hidden
	{
		println!("File {} is already hidden", file);
		return;
	}
	println!("File {} is not yet hidden", file);
	unsafe {
		SetFileAttributesA(
			PCSTR::from_raw(::std::format!("{}{}", file, '\0').as_ptr()),
			FileSystem::FILE_ATTRIBUTE_HIDDEN,
		);
	}
	println!("File {} has now been hidden", file);
}
