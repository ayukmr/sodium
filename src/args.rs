use clap::Parser;

#[derive(Parser)]
// arguments for displaying
pub struct Args {
    #[clap(
        default_value = ".",
        value_name = "PATH",
        help = "Path to display files",
    )]
    // path for displaying
    pub path: String,

    #[clap(
        short, long,
        default_value = "1",
        default_value_if("tree", None, Some("50")),
        value_name = "DEPTH",
        help = "Maximum depth to display",
    )]
    // maximum depth for walkdir
    pub depth: usize,

    #[clap(
        short, long,
        help = "Display files with colors",
    )]
    // enable colorify
    pub colored: bool,

    #[clap(
        short, long,
        help = "Display files with icons",
    )]
    // enable iconify
    pub icons: bool,

    #[clap(
        short, long,
        help = "Display files in tree",
    )]
    // display entries as tree
    pub tree: bool,
}

// parse args using clap
pub fn parse_args() -> Args {
    Args::parse()
}
