mod parse;
mod plot;

fn main() {
    let times = parse::get_commit_times();
    plot::plot_terminal(times);
}
