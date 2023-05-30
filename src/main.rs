mod file_info;

use std::env::current_dir;

fn main()
{
	let working_dir: String = get_working_directory();
	println!("Hiding all files starting with a `.` in {}", working_dir);

	let files = file_info::FileInfo::from_path(working_dir.as_str());
	for file in files
	{
		if file.hidden == false
		{
			file.hide();
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
