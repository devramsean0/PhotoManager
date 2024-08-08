use inquire::Text;

pub fn run(collection_name: Option<String>, output_path: String) {
    let collection_name = collection_name.unwrap_or_else(|| {
        let input = Text::new("What is the name of the collection?").prompt();
        match input {
            Ok(name) => name,
            Err(e) => {
                println!("Error: {}", e);
                return String::new();
            }
        }
    });
    println!("Syncing collection {} to {}", collection_name, output_path);
}