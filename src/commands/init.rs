use std::fs;

use crate::utils::{render_text, render_title, RendererProps};

pub fn execute() -> std::io::Result<()> {
    let mut dir = home::home_dir().expect("Cannot get your home directory");
    dir.push("todo");

    render_title();

    if !dir.exists() {
        match fs::create_dir_all(&dir) {
            Ok(_) => {
                render_text("**Initialized successfully!**", RendererProps::default());
                Ok(())
            }
            Err(err) => Err(err),
        }
    } else {
        render_text("**Todo CLI already initialized!**", RendererProps::warn());
        Ok(())
    }
}
