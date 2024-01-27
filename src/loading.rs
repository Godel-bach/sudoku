#[derive(Debug)]
pub struct LoadingIcon {
    pos: usize,
    size: usize,
    list: &'static [&'static str]
}

impl LoadingIcon {
    pub fn default() -> Self {
        Self { pos: 0, size: 8, list: &["⣧", "⣏", "⡟", "⠿", "⢻", "⣹", "⣼", "⣶"] }
    }

    pub fn on_tick(&mut self) {
        self.pos = (self.pos + 1) % self.size;
    }

    pub fn content(&self) -> &str {
        self.list[self.pos]
    }
}