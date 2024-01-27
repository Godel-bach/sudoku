mod event;
mod model;
mod tui;
mod ui;
mod loading;
mod solver;

use event::{Event, EventHandler};
use model::{update_keyevent, update_tick, Model};
use ratatui::prelude::{CrosstermBackend, Terminal};
use tui::Tui;

fn main() -> color_eyre::Result<()> {
    let mut model = Model::default();

    let backend = CrosstermBackend::new(std::io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.enter()?;

    while !model.should_quit() {
        // Render the user interface.
        tui.draw(&mut model)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => update_tick(&mut model),
            Event::Key(key_event) => update_keyevent(&mut model, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}
