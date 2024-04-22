use termimad::{
    crossterm::style::{Attribute, Color},
    MadSkin,
};

pub struct RendererProps {
    normal_color: Color,
    bold_color: Color,
    italic_color: Color,
    bold_underline: bool,
    italic_underline: bool,
}

impl RendererProps {
    pub fn default() -> RendererProps {
        RendererProps {
            normal_color: Color::White,
            bold_color: Color::DarkGreen,
            italic_color: Color::Yellow,
            bold_underline: false,
            italic_underline: false,
        }
    }

    pub fn bold() -> RendererProps {
        RendererProps {
            normal_color: Color::White,
            bold_color: Color::DarkGreen,
            italic_color: Color::Yellow,
            bold_underline: true,
            italic_underline: false,
        }
    }

    pub fn error() -> RendererProps {
        RendererProps {
            normal_color: Color::Red,
            bold_color: Color::DarkRed,
            italic_color: Color::Red,
            bold_underline: false,
            italic_underline: true,
        }
    }

    pub fn warn() -> RendererProps {
        RendererProps {
            normal_color: Color::Yellow,
            bold_color: Color::DarkYellow,
            italic_color: Color::Yellow,
            bold_underline: true,
            italic_underline: true,
        }
    }
}

pub fn render_title() {
    let mut skin = MadSkin::default();
    skin.bold.set_fg(Color::DarkGreen);
    println!("{}", skin.inline("** Todo CLI**"));
}

pub fn render_text(text: &str, props: RendererProps) {
    let mut skin = MadSkin::default();
    skin.set_fg(props.normal_color);
    skin.italic.set_fg(props.italic_color);
    skin.bold.set_fg(props.bold_color);
    if props.bold_underline {
        skin.bold.add_attr(Attribute::Underlined);
    }
    if props.italic_underline {
        skin.italic.add_attr(Attribute::Underlined);
    }
    println!("{}", skin.inline(&text));
}
