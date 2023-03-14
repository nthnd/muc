use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Display top n commands
    #[arg(short, long)]
    pub count: Option<usize>,

    /// Make output pretty
    #[arg(short, long, default_value_t = true)]
    pub pretty: bool,

    /// Show debug messages
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// Bar opening character
    #[arg(long, default_value_t = '[')]
    pub bar_open: char,

    /// Bar closing character
    #[arg(long, default_value_t = ']')]
    pub bar_close: char,

    /// Bar character
    #[arg(long, default_value_t = '▮')]
    pub bar: char,

    /// Regular expression to allow for the removal of prefixes in shells like zsh. Default value is for zsh. NOTE: shell overrides this argument
    #[arg(short, long, default_value_t = String::from(""))]
    pub regexp: String,

    /// Preset regular expressions for common shells: Bash, ZSH, Fish.
    #[arg(long, default_value_t = String::from(""))]
    pub shell: String,

}
