use std::{fs, fs::File, io, io::Write, io::Read, io::BufReader};
use crate::functions;

pub fn handler(_select: &str) -> bool {
    let result = select_options(&_select);
    if handle_option(&result.trim(), &functions::return_path(), &_select) == false {
    	return false
    }
    return true
}

pub fn get_projects(_path: &str) -> Vec<String> {
	let projects_name = fs::read_dir(_path).unwrap();
	let mut projects_name_vector: Vec<String> = Vec::new();

	for project in projects_name {
		let name = project.unwrap().path().display().to_string();
		projects_name_vector.push(name.replace(&format!("{_path}/"), ""));
	}

	projects_name_vector
}

pub fn show_projects(_project_names: &Vec<String>) {
	println!("\n=====================");
	for project in _project_names {
		println!("Project: {}", project);
	}
	println!("=====================\n");
}

pub fn insert_note(_path: &str, _project_name: &str, _note: &str) {
	let conf_path = format!("{}/{}/{}_config.txt", _path.trim(), _project_name.trim(), _project_name.trim());
	println!("{:?}", conf_path);
	// let mut config = fs::write(&conf_path, format!("{}\n", _note)).expect("Failed to open config file.");
	println!("{}", _note);

	let mut file = fs::OpenOptions::new().write(true).append(true).open(&conf_path).expect("failed");
	writeln!(file, "{}", _note).expect("Failed");
}

pub fn select_options(_project_name: &str) -> String {
	println!("\n[+] Command: (1)Insert note/timeline (2)See content (3)Close");
	print!("[~ {}] Enter here: ", _project_name.trim());
	io::stdout().flush().expect("Failed to flush.");
	let mut choice = String::new();
	io::stdin().read_line(&mut choice).expect("Failed to read line.");
	choice
}

pub fn handle_option(_option: &str, _path: &str, _project_name: &str) -> bool {
	if _option == "1" {
		let mut note = String::new();
		print!("[ ~ {_project_name}] Enter here: ");
		io::stdout().flush().expect("Failed to flush");
		io::stdin().read_line(&mut note).expect("Failed to read line.");
		insert_note(_path, _project_name, &note);
	} else if _option == "2" {
		// let file_content = fs::read_to_string(&format!("{_path}/{_project_name}_config"));		
		println!("{}/{}_config", _path.trim(), _project_name.trim());
		let file = File::open(&format!("{}/{}/{}_config.txt", _path.trim(), _project_name.trim(), _project_name.trim())).expect("failed to read.");
	    let mut buf_reader = BufReader::new(file);
	    let mut contents = String::new();
	    buf_reader.read_to_string(&mut contents).expect("failed to read string");

		println!("======== Content Start =========");
		println!("{}", &contents);
		println!("======== Content End =========");
	} else if _option == "3" {
		return true
	}
	 else {
		handle_option(_option, _path, _project_name);
	}

	return false
}