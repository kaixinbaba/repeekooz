use crate::app::{App, AppMode};

use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};

use tui::text::Text;
use tui::widgets::{Block, Paragraph};

pub fn redraw(app: &mut App) {
    let terminal = &mut app.terminal;

    terminal.draw(|f| {}).unwrap();
}
