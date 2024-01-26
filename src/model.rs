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
    Right,
}

impl Model {
    pub fn default() -> Self {
        let puzzel = [[None; 9]; 9];
        Model {
            puzzel: puzzel,
            state: RunningState::Presolve,
            pos: Position::default(),
        }
    }

    pub fn get_number(&self, i: usize, j: usize) -> String {
        self.puzzel[i][j].map_or("".into(), |x| x.to_string())
    }

    pub fn get_position(&self) -> &Position {
        &self.pos
    }

    pub fn get_position_mut(&mut self) -> &mut Position {
        &mut self.pos
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
        KeyCode::Char(c) => {
            if c >= '1' && c <= '9' {
                if let Position::Left(x, y) = model.get_position() {
                    model.puzzel[*x][*y] = Some(c.to_digit(10).unwrap() as u8);
                }
            }
        }
        KeyCode::Up => {
            if let Position::Left(_, y) = model.get_position_mut() {
                *y = (*y + 8) % 9;
            }
        }
        KeyCode::Down => {
            if let Position::Left(_, y) = model.get_position_mut() {
                *y = (*y + 1) % 9;
            }
        }
        KeyCode::Left => {
            if let Position::Left(x, _) = model.get_position_mut() {
                *x = (*x + 8) % 9;
            }
        }
        KeyCode::Right => {
            if let Position::Left(x, _) = model.get_position_mut() {
                *x = (*x + 1) % 9;
            }
        }
        _ => {}
    };
}
