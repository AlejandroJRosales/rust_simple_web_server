use std::env;
use std::fs;
use std::io;

pub fn initialize() {
    let mut gotten_name = false;
    while !gotten_name {
        println!("Please enter a name for the new component.");
        let mut name = String::new();
        io::stdin()
            .read_line(&mut name)
            .expect("Failed to read line");
        name = name.trim().to_owned();
        let check = match verify_name(&name) {
            Ok(_) => true,
            Err(_) => false,
        };
        if check {
            create_files(&name);
            gotten_name = true;
        } else {
            println!(
                "There was an error in the input name or the component folder already exists."
            );
        }
    }
}

fn verify_name(name: &String) -> Result<bool, io::Error> {
    fn check_name_chars(n: &String) -> bool {
        for c in n.chars() {
            if c != '_' && !c.is_alphabetic() {
                return false;
            }
        }
        return true;
    }
    if std::path::Path::new(&format!(
        "{}/src/{}",
        env::current_dir()
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
            .to_owned(),
        name
    ))
    .exists()
    {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Directory already exists",
        ));
    } else {
        if !check_name_chars(&name) {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Directory name contains an invalid character.",
            ));
        } else {
            return Ok(true);
        }
    }
}

fn create_files(name: &String) {
    // Start directory creation
    let mut temp = fs::DirBuilder::new();
    temp.recursive(true);
    temp.create(format!(
        "{}/src/{}",
        env::current_dir()
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
            .to_owned(),
        name
    ))
    .unwrap();
    println!("Directory Created");
    // End directory creation
    // Start file creation
    fs::File::create(format!(
        "{}/src/{}/mod.rs",
        env::current_dir()
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
            .to_owned(),
        name
    ))
    .unwrap();
    println!("mod.rs successfully created.");
    fs::write(
        format!(
            "{}/src/{}/mod.rs",
            env::current_dir()
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
                .to_owned(),
            name
        ),
        "pub mod routes;",
    )
    .expect("Error writing mod.rs");
    println!("mod.rs successfully populated.");
    fs::File::create(format!(
        "{}/src/{}/routes.rs",
        env::current_dir()
            .unwrap()
            .as_path()
            .to_str()
            .unwrap()
            .to_owned(),
        name
    ))
    .unwrap();
    println!("routes.rs successfully created.");
    fs::write(
        format!(
            "{}/src/{}/routes.rs",
            env::current_dir()
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
                .to_owned(),
            name
        ),
        "pub fn get_routes() -> (String, String, Vec<(String, String, String)>) {\n\treturn (\"/\".to_owned(), \"app\".to_owned(), vec![\n\t\t//(\"\".to_owned(), \"index.html\".to_owned(), \"home_page\".to_owned()),\n\t]);\n}",
    )
    .expect("Error writing routes.rs");
    println!("routes.rs successfully populated.");
    // Modify the src/main.rs
    let mut main_data = String::from(format!("mod {};\n", name));
    let old_main_data = fs::read_to_string(
        format!(
            "{}/src/main.rs",
            env::current_dir()
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
                .to_owned()
        )
    ).expect("Error reading main.rs.");
    main_data = main_data + old_main_data.as_str();
    fs::write(format!(
            "{}/src/main.rs",
            env::current_dir()
                .unwrap()
                .as_path()
                .to_str()
                .unwrap()
                .to_owned()
        ), main_data).expect("Failed to modify src/main.rs");
    println!("Successfully modified src/main.rs");
    println!("The component {} has been added to the project.\nPlease refer to the comments within src/components/routes.rs on how to complete the addition of your new component to the application.", name);
}
