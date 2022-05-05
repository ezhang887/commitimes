mod parse;
mod plot;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(
    author = "ezhang887",
    version = "0.1.0",
    about = "Plot histogram of a git repo's commit times in the terminal!"
)]
struct Args {
    #[clap(short, long, default_value = ".")]
    repo_path: String,

    #[clap(short, long)]
    username: Option<String>,
}

fn main() {
    let args = Args::parse();

    let times = parse::get_commit_times(args.repo_path);
    plot::plot_terminal(times);
}
