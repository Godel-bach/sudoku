use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{loading::LoadingIcon, solver::SolverHandler};

#[derive(Debug)]
pub struct Model {
    puzzel: [[Option<u8>; 9]; 9],
    state: RunningState,
    pos: Position,
    icon: LoadingIcon,
    solver: Option<SolverHandler>,
    time: Option<Duration>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum RunningState {
    #[default]
    Presolve,
    Solving,
    Done,
    Leaving,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Position {
    Left(usize, usize),
    RightUp,
    RightDown,
}

impl Model {
    pub fn default() -> Self {
        let puzzel = [[None; 9]; 9];
        Model {
            puzzel: puzzel,
            state: RunningState::Presolve,
            pos: Position::default(),
            icon: LoadingIcon::default(),
            solver: None,
            time: None,
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

    pub fn get_state(&self) -> &RunningState {
        &self.state
    }

    pub fn get_icon(&self) -> &LoadingIcon {
        &self.icon
    }

    pub fn get_time(&self) -> &Duration {
        if let Some(time) = &self.time {
            return time;
        }
        unimplemented!()
    }

    pub fn get_puzzel(&self) -> [[Option<u8>; 9]; 9] {
        self.puzzel
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

pub fn update_keyevent(model: &mut Model, key_event: KeyEvent) {
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
        KeyCode::Backspace => {
            if let Position::Left(x, y) = model.get_position() {
                model.puzzel[*x][*y] = None;
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
            } else {
                *model.get_position_mut() = Position::default();
            }
        }
        KeyCode::Right => {
            if let Position::Left(x, _) = model.get_position_mut() {
                *x = (*x + 1) % 9;
            } else {
                *model.get_position_mut() = Position::default();
            }
        }
        KeyCode::Enter => {
            if let Position::Left(_, _) = model.get_position_mut() {
                *model.get_position_mut() = Position::RightUp;
            } else if let Position::RightUp = model.get_position_mut() {
                *model.get_position_mut() = Position::RightDown;
                model.state = RunningState::Solving;
                model.solver = Some(SolverHandler::new(model.get_puzzel()));
            }
        }
        _ => {}
    };
}

pub fn update_tick(model: &mut Model) {
    model.icon.on_tick();
    if let Some(handler) = &model.solver {
        if let Ok((time, result)) = handler.try_get() {
            model.time = Some(time);
            model.puzzel = result;
            model.state = RunningState::Done;
        }
    }
}
