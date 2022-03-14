pub fn load_file(filepath: &str) -> Result<std::fs::File, Box<dyn std::error::Error>> {
    let mut file_open_result = std::fs::File::open(filepath);
    return match file_open_result {
        Ok(file) => Ok(file),
        Err(_) => {
            let file_creation_result = std::fs::File::create(filepath);
            match file_creation_result {
                Ok(file) => Ok(file),
                Err(e) => Err(Box::new(e)),
            }
        },
    }
}