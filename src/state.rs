struct State {
    input_path: String,
    root_output_path: String,
    collection: String,
}

impl State {
    const fn new() -> Self {
        Self {
            input_path: String::new(),
            root_output_path: String::new(),
            collection: String::new(),
        }
    }
}