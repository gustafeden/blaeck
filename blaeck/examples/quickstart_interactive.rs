//! Interactive quickstart example - task selector with visual feedback.

#[path = "previews/mod.rs"]
mod previews;

use blaeck::reactive::*;

fn main() -> std::io::Result<()> {
    ReactiveApp::run(previews::quickstart_interactive::task_selector)?;
    Ok(())
}
