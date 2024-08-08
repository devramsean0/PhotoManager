use inquire::Text;
use std::fs;

pub fn run(input_path: Option<String>, collection_name: Option<String>, output_path: String) {
    let input_path = input_path.unwrap_or_else(|| {
        let input = Text::new("What is the path to the input directory?").with_autocomplete(crate::utils::file_path_autocomplete::FilePathCompleter::default()).prompt();
        match input {
            Ok(path) => path,
            Err(e) => {
                println!("Error: {}", e);
                return String::new();
            }
        }
    });
    let collection_name = collection_name.unwrap_or_else(|| {
        let input = Text::new("What is the name of the collection?").prompt();
        match input {
            Ok(name) => name,
            Err(e) => {
                eprintln!("Error: {}", e);
                return String::new();
            }
        }
    });
    println!("Creating collection {} from {} to {}", collection_name, input_path, output_path);
    let complete_output_path = format!("{}/{}", output_path, collection_name);
    // Now create folder structure
    fs::create_dir_all(complete_output_path.clone()).expect("Failed to create output directory");
    // Copy files from input to output.
    let input_contents = fs::read_dir(input_path).expect("Failed to read input path");
    for entry in input_contents {
        let entry = entry.expect("Failed to read entry");
        let entry_path = entry.path();
        let entry_name = entry.file_name().into_string().expect("Failed to convert entry name to string");
        let output_file = format!("{}/{}", complete_output_path.to_owned(), entry_name);
        fs::copy(entry_path, output_file.clone()).expect("Failed to copy file");
        println!("Copied {} to {}", entry_name, output_file);
    }
}