use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub struct Model {
    puzzel: [[Option<u8>; 9]; 9],
    state: RunningState,
    pos: Position,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Presolve,
    Solving,
    Done,
    Leaving,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Position {
    Left(usize, usize),
    Right
}

impl Model {
    pub fn default() -> Self {
        let puzzel = [[None; 9]; 9];
        Model { puzzel: puzzel, state: RunningState::Presolve, pos: Position::default() }
    }

    pub fn get_number(&self, i: usize, j: usize) -> String {
        self.puzzel[i][j].map_or("".into(), |x| x.to_string())
    }

    pub fn get_position(&self) -> &Position {
        &self.pos
    }

    pub fn quit(&mut self) {
        self.state = RunningState::Leaving;
    }

    pub fn should_quit(&self) -> bool {
        if let RunningState::Leaving = self.state {
            return true;
        }
        false
    }
}

impl Position {
    pub fn default() -> Self {
        Self::Left(0, 0)
    }
}

pub fn update(model: &mut Model, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => model.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                model.quit()
            }
        }
        _ => {}
    };
}