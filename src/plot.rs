use plotlib::page::Page;
use plotlib::repr::{Histogram, HistogramBins};
use plotlib::view::ContinuousView;

pub fn plot_terminal(commit_times: Vec<String>) {
    let hours: Vec<f64> = commit_times
        .iter()
        .map(|time| {
            time.chars()
                .take(2)
                .collect::<String>()
                .parse::<f64>()
                .unwrap()
        })
        .collect();

    let hist = Histogram::from_slice(&hours, HistogramBins::Count(24));
    let v = ContinuousView::new().add(hist);
    println!("{}", Page::single(&v).dimensions(60, 15).to_text().unwrap());
}
