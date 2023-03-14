use std::str::FromStr;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Display top n commands
    #[arg(short, long, default_value_t = 10)]
    pub count: usize,

    /// Show debug messages
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Change how the bar looks --bar [,▮, ,]
    #[arg(long, default_value_t = Default::default())]
    pub bar: Bar,

    /// Regular expression to allow for the removal of prefixes in shells like zsh. Default value is for zsh. NOTE: shell overrides this argument
    #[arg(short, long, default_value_t = String::from(""))]
    pub regexp: String,

    /// Preset regular expressions for common shells: Bash, ZSH, Fish.
    #[arg(long, default_value_t = String::from(""))]
    pub shell: String,
}

#[derive(Debug, Clone)]
pub struct Bar {
    pub opening: String,
    pub closing: String,
    pub fill: String,
    pub empty: String,
}
impl Default for Bar {
    fn default() -> Self {
        Bar {
            opening: "[".to_owned(),
            fill: "▮".to_owned(),
            empty: " ".to_owned(),
            closing: "]".to_owned(),
        }
    }
}

impl FromStr for Bar {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.split(',').collect::<Vec<&str>>();
        match chars.len() {
            4 => Ok(Bar {
                opening: chars[0].to_string(),
                fill: chars[1].to_string(),
                empty: chars[2].to_string(),
                closing: chars[3].to_string(),
            }),
            _ => Err("Invalid bar length".to_string()),
        }
    }
}

impl std::fmt::Display for Bar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{}",
            self.opening, self.fill, self.empty, self.closing
        )
    }
}
