use std::fs;

pub fn execute() -> std::io::Result<()> {
    let result = fs::create_dir_all("/tmp/todo/");
    match result {
        Ok(_) => Ok(()),
        Err(err) => Err(err) 
    }
}
