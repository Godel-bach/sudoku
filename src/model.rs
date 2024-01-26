#[derive(Debug)]
pub struct Model {
    puzzel: [[Option<u8>; 9]; 9],
    state: RunningState,
}

#[derive(Debug, Default, PartialEq, Eq)]
enum RunningState {
    #[default]
    Presolve,
    Solving,
    Done,
}

impl Model {
    pub fn default() -> Self {
        let puzzel = [[None; 9]; 9];
        Model { puzzel: puzzel, state: RunningState::Presolve }
    }

    pub fn get(&self, i: usize, j: usize) -> String {
        self.puzzel[i][j].map_or("".into(), |x| x.to_string())
    }
}