use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style, Stylize},
    symbols,
    widgets::{Block, BorderType, Borders, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::model::Model;

pub fn render(model: &Model, frame: &mut Frame) {
    let mut constraits = vec![Constraint::Percentage(11); 8];
    constraits.push(Constraint::Percentage(12));

    let horizontal_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraits)
        .split(frame.size());
    let mut layout = Vec::default();

    for i in 0..9 {
        let mut constraits = vec![Constraint::Percentage(11); 8];
        constraits.push(Constraint::Percentage(12));
        layout.push(
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(constraits)
                .split(horizontal_layout[i]),
        );
    }

    for i in 0..9 {
        for j in 0..9 {
            let block = 
            match (i, j) {
                (0, 0) => {
                    Block::new()
                                .border_set(symbols::border::PLAIN)
                                .borders(Borders::TOP | Borders::LEFT)
                                .border_type(BorderType::Thick)
                        
                }
                (1 | 2 | 4 | 5 | 7, 0 | 3 | 6) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_HORIZONTAL,
                        vertical_left: symbols::line::VERTICAL,
                        horizontal_top: symbols::line::THICK_HORIZONTAL,
                        ..symbols::border::PLAIN
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT)
                }
                (3 | 6, 0) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_HORIZONTAL_DOWN,
                        vertical_left: symbols::line::THICK_VERTICAL,
                        horizontal_top: symbols::line::THICK_HORIZONTAL,
                        ..symbols::border::PLAIN
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT)
                }
                (8, 0) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_HORIZONTAL,
                        vertical_left: symbols::line::VERTICAL,
                        ..symbols::border::THICK
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                }
                (0 | 3 | 6, 1 | 4 | 7) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_VERTICAL,
                        vertical_left: symbols::line::THICK_VERTICAL,
                        ..symbols::border::PLAIN
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT)
                }
                (1 | 4 | 7, 1 | 2 | 4 | 5 | 7) | (2 | 5, 1 | 2 | 4 | 5 | 7) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::CROSS,
                        ..symbols::border::PLAIN
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT)
                }
                (8, 1 | 2 | 4 | 5) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::CROSS,
                        top_right: symbols::line::THICK_VERTICAL,
                        vertical_right: symbols::line::THICK_VERTICAL,
                        ..symbols::border::PLAIN
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                }
                (0 | 3 | 6, 2 | 5) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_VERTICAL,
                        vertical_left: symbols::line::THICK_VERTICAL,
                        ..symbols::border::PLAIN
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT)
                }
                (3 | 6, 3 | 6) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_CROSS,
                        ..symbols::border::THICK
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT)
                }
                (0, 3 | 6) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_VERTICAL_RIGHT,
                        ..symbols::border::THICK
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT)
                }
                (8, 3 | 6) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_HORIZONTAL,
                        top_right: symbols::line::THICK_VERTICAL_LEFT,
                        vertical_left: symbols::line::VERTICAL,
                        ..symbols::border::THICK
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                }
                (8, 7) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::CROSS,
                        top_right: symbols::line::THICK_VERTICAL,
                        horizontal_top: symbols::line::HORIZONTAL,
                        vertical_left: symbols::line::VERTICAL,
                        ..symbols::border::THICK
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
                }
                (0, 8) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_VERTICAL,
                        horizontal_top: symbols::line::HORIZONTAL,
                        ..symbols::border::THICK
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                }
                (3 | 6, 8) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::THICK_VERTICAL,
                        horizontal_top: symbols::line::HORIZONTAL,
                        bottom_left: symbols::line::THICK_HORIZONTAL_UP,
                        ..symbols::border::THICK
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                }
                (1 | 2 | 4 | 5 | 7, 8) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::CROSS,
                        horizontal_bottom: symbols::line::THICK_HORIZONTAL,
                        bottom_left: symbols::line::THICK_HORIZONTAL,
                        ..symbols::border::PLAIN
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::TOP | Borders::LEFT | Borders::BOTTOM)
                }
                (8, 8) => {
                    let border_set = symbols::border::Set {
                        top_left: symbols::line::CROSS,
                        top_right: symbols::line::THICK_VERTICAL,
                        horizontal_top: symbols::line::HORIZONTAL,
                        vertical_left: symbols::line::VERTICAL,
                        bottom_left: symbols::line::THICK_HORIZONTAL,
                        ..symbols::border::THICK
                    };
                    Block::new()
                        .border_set(border_set)
                        .borders(Borders::ALL)
                }
                _ => {
                    Block::new()
                }
            };
            frame.render_widget(
                Paragraph::new(model.get(i, j)).block(
                    block,
                ),
                layout[i][j],
            );
        }
    }
}
