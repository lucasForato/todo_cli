use clap::Args;
use std::fs;
use std::fs::File;

#[derive(Args, Clone, Debug)]
pub struct ListArgs {
    #[arg(short = 'n', long = "new")]
    new: Option<String>,
}

pub fn execute(args: ListArgs) -> std::io::Result<()> {
    if Option::is_some(&args.new) {
        let file_name = args.new.unwrap();
        let file_created = create_new_list(file_name);
        return match file_created {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        };
    }

    list_todo_lists()
}

fn create_new_list(file_name: String) -> std::io::Result<()> {
    let path = format!("/tmp/todo/{}.md", file_name);
    let file = File::create(path);
    match file {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

fn list_todo_lists() -> std::io::Result<()> {
    let paths = fs::read_dir("/tmp/todo/");
    if paths.is_err() {
        let err = paths.err().unwrap();
        return Err(err);
    }

    for path in paths.unwrap() {
        let list_name = path
            .unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap()
            .replace("/tmp/todo/", "")
            .replace(".md", "");

        println!("- {}", list_name)
    }
    Ok(())
}
