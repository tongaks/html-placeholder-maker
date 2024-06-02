use std::{io, io::Write, path::Path, fs, process::exit};
mod functions;
mod placeholders;
mod select;

fn main() {
    println!("\n\n====== Web Project Manager by Bandi2 ======");

    // check if project folder exists
    if !Path::new(&functions::return_path()).exists() {
        println!("[!] Projects folder does not exist. Creating...");
        if let Err(err) = fs::create_dir_all(&functions::return_path()) {
            eprintln!("[!] Failed to create project folder: {}", err);
            return;
        }

        println!("[+] Projects folder successfully created.");
    }

    let mut choice = String::new();
    println!("[~] Commands: (1)Create (2)Delete (3)Select (4)Exit");
    print!("[+] Enter here: ");
    io::stdout().flush().expect("Failed to flush output.");
    io::stdin().read_line(&mut choice).expect("Failed to get input.");

    if choice.trim() == "1" {
        if !functions::create_webpage_folder(&functions::get_project_name(), &functions::return_path()) {
            println!("\n[!] Error: Failed to create project.");
            println!("[!] Exiting.");
        } else {
            println!("[+] Finished.");            
        }
        main();
    } else if choice.trim() == "2" {
        println!("[+] Existing projects: ");
        let projects = select::get_projects(&functions::return_path());
        select::show_projects(&projects);
        if !functions::remove_project(&functions::get_project_name(), &functions::return_path()) {
            println!("\n[!] Error: Failed to delete project.");
            println!("[!] Exiting.");                
        } else {
            println!("[+] Project removed.");
        }        
        main();
    } else if choice.trim() == "3" {
        let projects = select::get_projects(&functions::return_path());
        select::show_projects(&projects);

        let mut select = String::new(); 
        print!("[~] Select: ");
        io::stdout().flush().expect("Failed to flush output.");
        io::stdin().read_line(&mut select).expect("Failed to select project.");

        if !projects.contains(&select.trim().to_string()) {
            println!("[!] Error: Project not found.");
            main();
        }

        if !select::handler(&select) == true {
            select::handler(&select);
        }

        main();
    } else if choice.trim() == "4" {
        println!("[!] Byee");
        exit(0);
    } 

    else { 
        println!("\n[!] Error: Input not in the scope");
        main();
    }

}