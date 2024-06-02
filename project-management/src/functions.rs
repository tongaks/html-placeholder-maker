use std::{io, io::Write, path::Path, fs::File, fs};

use crate::placeholders::{return_css, return_php, format_html};

pub fn get_project_name() -> String {
    let mut project_name = String::new();

    print!("[~] Project name: ");
    io::stdout().flush().expect("Failed to flush output.");
    io::stdin().read_line(&mut project_name).expect("Failed to get project_name.");
    if project_name.trim() == "" { 
        println!("[!] Error: No project name provided"); 
        get_project_name();
    }

    project_name.trim().to_string()
}

pub fn return_path() -> String {
    let home_dir = std::env::var("HOME").expect("HOME environment variable not found");
    let path = format!("{}/Documents/WebProjects", home_dir);
    path.to_string()
}

pub fn check_project(_path: &str, _proj_name: &str) -> bool {
    if Path::new(&format!("{}/{}", _path, _proj_name)).exists() {
        return true;
    }
    return false;
}

pub fn remove_project(_proj_name: &str, _path: &str) -> bool{
    if !check_project(_path, _proj_name) {
        println!("[!] Error: Project does not exists.");
        return false;
    }

    let _ = fs::remove_dir_all(format!("{}/{}", _path, _proj_name)).expect("Failed to remove project.");
    return true;
}

pub fn create_webpage_folder(_fname: &str, _path: &str) -> bool {    
    if check_project(_path, _fname) {
        println!("[!] Error:  Project already exists.");
        return false;
    }

    let mut confirm = String::new();
    println!("[+] Type 'yes' to confirm or 'no'\n");
    print!("[~] Enter here: ");
    io::stdout().flush().expect("Failed to flush output.");
    io::stdin().read_line(&mut confirm).expect("Failed to get confirm.");

    if confirm.trim() == "no" {
        return false
    }

    println!("\n[+] Creating {_fname} folder...");
    let _ = fs::create_dir_all(format!("{}/{}", _path, _fname));
    println!("[+] Created {_fname} folder.");

    println!("[+] Creating placeholders...");
    let html_code = format_html(&(_fname.to_owned() + ".css"), &(_fname.to_owned() + ".js"));

    let mut main_html = File::create(format!("{}/{}/{}.html", _path, _fname, _fname)).expect("Failed to create html file.");
    let _ = main_html.write_all(html_code.as_bytes());

    let css_code = return_css();
    let mut main_css = File::create(format!("{}/{}/{}.css", _path, _fname, _fname)).expect("Failed to create html file.");
    let _ = main_css.write_all(css_code.as_bytes());

    let php_content = return_php();
    let mut create_php = File::create(format!("{}/{}/{}.php", _path, _fname, _fname)).expect("Failed to create php file.");
    let _ = create_php.write_all(php_content.as_bytes());       

    let _create_config = File::create(format!("{}/{}/{}_config.txt", _path, _fname, _fname)).expect("Failed to create php file.");

    println!("[+] Created placeholder.");

    println!("[+] Creating image folder.");
    let _ = fs::create_dir_all(format!("{}/{}/{}-images", _path, _fname, _fname));
    println!("[+] Created image folder.");

    return true;
}