mod model;
mod ui;
mod event;
mod tui;

use event::{Event, EventHandler};
use model::{update, Model};
use tui::Tui;
use ratatui::prelude::{CrosstermBackend, Terminal};

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
            Event::Tick => {}
            Event::Key(key_event) => update(&mut model, key_event),
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        };
    }

    tui.exit()?;
    Ok(())
}

