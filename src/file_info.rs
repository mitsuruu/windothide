use std::{fs, os::windows::fs::MetadataExt, path::Path};

use windows::{
	core::PCSTR,
	Win32::Storage::FileSystem::{self, SetFileAttributesA},
};

pub struct FileInfo
{
	pub path: String,
	pub hidden: bool,
}

impl FileInfo
{
	pub fn hide(self)
	{
		unsafe {
			SetFileAttributesA(
				PCSTR::from_raw(::std::format!("{}{}", self.path, '\0').as_ptr()),
				FileSystem::FILE_ATTRIBUTE_HIDDEN,
			);
		}
	}

	pub fn from_path(path: &str) -> Vec<FileInfo>
	{
		let mut collection: Vec<FileInfo> = Vec::new();
		let paths: Vec<String> = fs::read_dir(path)
			.unwrap()
			.filter_map(|e| e.ok())
			.map(|e| e.path().to_string_lossy().into_owned())
			.filter_map(|e| {
				match Path::new(&e)
					.file_name()
					.unwrap()
					.to_os_string()
					.into_string()
					.unwrap()
					.starts_with(".")
				{
					true => Some(e),
					false => None,
				}
			})
			.collect::<Vec<String>>();

		for path in paths
		{
			let metadata = fs::metadata(path.clone());
			match metadata
			{
				Ok(metadata) =>
				{
					let attributes = metadata.file_attributes();
					let hidden = attributes & 2 == 2; // if only there were an easier way for this to be implemented that wouldn't require a bunch of manual work from me
					collection.push(FileInfo { path, hidden });
				}
				Err(_) => println!("Attributes for file `{}` was unable to be read.", path),
			}
		}

		collection
	}
}
