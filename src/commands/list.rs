use clap::Args;
use std::fs;
use std::fs::File;

use crate::utils::{render_text, render_title, RendererProps};

#[derive(Args, Clone, Debug)]
pub struct ListArgs {
    #[arg(short = 'n', long = "new")]
    new: Option<String>,

    #[arg(short = 'd', long = "delete")]
    delete: Option<String>
}

pub fn execute(args: ListArgs) -> std::io::Result<()> {
    if Option::is_some(&args.new) {
        let dir_name = args.new.unwrap();
        let file_created = create_new_list(dir_name);
        return match file_created {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        };
    }

    if Option::is_some(&args.delete) {
        let dir_name = args.delete.unwrap();
        let file_deleted = delete_todo_list(dir_name);
            return match file_deleted {
                Ok(_) => Ok(()),
                Err(err) => Err(err)
            };
    }

    list_todo_lists()
}

fn create_new_list(file_name: String) -> std::io::Result<()> {
    let mut dir = home::home_dir().expect("Cannot get your home directory");
    dir.push("todo");
    dir.push(&file_name);

    let file = File::create(&dir);
    match file {
        Ok(_) => {
            let message = format!("List created: **{}**", file_name);
            render_text(&message, RendererProps::bold());
            Ok(())
        },
        Err(err) => Err(err),
    }
}

fn list_todo_lists() -> std::io::Result<()> {
    let mut dir = home::home_dir().expect("Cannot get your home directory");
    dir.push("todo");

    let paths = fs::read_dir(&dir);
    if paths.is_err() {
        let err = paths.err().unwrap();
        return Err(err);
    }

    render_title();

    for path in paths.unwrap() {
        let mut dir = home::home_dir().expect("Cannot get your home directory");
        dir.push("todo/");
        let home = dir.into_os_string().into_string().unwrap();

        let list_name = path
            .unwrap()
            .path()
            .into_os_string()
            .into_string()
            .unwrap()
            .replace(&home, "")
            .replace(".md", "");
        
        let item = format!(" **{}**", &list_name);
        render_text(&item, RendererProps::bold());
    }
    Ok(())
}

fn delete_todo_list(dir_name: String) -> std::io::Result<()> {
    let mut dir = home::home_dir().expect("Cannot get your home directory");
    dir.push("todo");
    dir.push(&dir_name);

    match fs::remove_file(&dir) {
        Ok(_) => {
            render_title();
            let message = format!("List deleted: **{}**", &dir_name);
            render_text(&message, RendererProps::bold());

            Ok(())
        },
        Err(err) => Err(err)
    }
}
