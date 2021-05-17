use std::path::Path;

pub fn read_to_string<P: AsRef<Path>>(_: P) -> Result<String, Box<dyn std::error::Error + 'static>> {
    Ok(String::from("\
        A tree\n\
        In the forest\n\
        Between many others"))
}