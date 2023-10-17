use app::{add, list, open, remove, AppConfig, Cli};
use clap::Parser;

mod app;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = AppConfig { home: cli.dir.try_into().unwrap() };

    match &cli.command {
        app::Commands::Remove { id } => remove(id.clone(), config),
        app::Commands::Open { id } => open(id.clone(), config),
        app::Commands::Add { id, file } => add(id.clone(), file.to_path_buf(), config),
        app::Commands::List => list(config),
    }
}
