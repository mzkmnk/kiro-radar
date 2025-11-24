mod app;
mod events;
mod spec;
mod ui;

use app::App;
use clap::Parser;

#[derive(Parser)]
#[command(name = "kiro-radar")]
#[command(version)]
#[command(about, long_about = None)]
struct Cli {}

fn main() -> color_eyre::Result<()> {
    let _cli = Cli::parse();

    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new(".").run(terminal);
    ratatui::restore();
    result
}
