use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    terminal::Frame,
    widgets::{Block, Borders, Paragraph},
};

use crate::handlers::config::CompleteConfig;

#[derive(Debug, Clone)]
pub struct Verticals {
    pub chunks: Vec<Rect>,
    pub constraints: Vec<Constraint>,
}

impl Verticals {
    pub fn new(chunks: Vec<Rect>, constraints: Vec<Constraint>) -> Self {
        Self {
            chunks,
            constraints,
        }
    }
}

pub fn draw_ui<T: Backend>(frame: &mut Frame<T>, config: &CompleteConfig) {
    let vertical_chunk_constraints = vec![Constraint::Min(1)];

    let margin = config.frontend.margin;

    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(margin)
        .constraints(vertical_chunk_constraints.as_ref())
        .split(frame.size());

    let verticals = Verticals::new(vertical_chunks, vertical_chunk_constraints);

    let table = Paragraph::new("Some text").block(Block::default().borders(Borders::ALL));

    frame.render_widget(table, verticals.chunks[0]);
}
