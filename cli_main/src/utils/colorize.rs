use std::io::{Error, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

macro_rules! register_color {
    ($fn_name: ident, $color: expr) => {
        pub fn $fn_name(s: String) -> Result<(), Error> {
            let mut stdout = StandardStream::stdout(ColorChoice::Auto);
            stdout.set_color(ColorSpec::new().set_fg(Some($color)))?;
            write!(&mut stdout, "{}", s)?;
            stdout.reset()?;
            write!(&mut stdout, "")?;
            Ok(())
        }
    };
}

register_color!(blue, Color::Blue);
register_color!(red, Color::Red);
register_color!(green, Color::Green);
register_color!(cyan, Color::Cyan);
register_color!(gray, Color::Rgb(119, 136, 153));

pub struct Auto {}

impl Auto {
    pub fn importance(s: String) -> Result<(), Error> {
        match s.as_str() {
            "No Importance" | "Tiny Importance" => gray(s),
            "Normal Importance" | "Big Importance" => green(s),
            _ => red(s),
        }
    }
}
