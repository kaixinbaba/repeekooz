use crate::app::{App, AppMode};

use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};

use tui::style::Color;
use tui::style::Style;
use tui::text::Text;
use tui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn redraw(app: &mut App) {
    let terminal = &mut app.terminal;

    terminal
        .draw(|f| {
            let layout = Layout::default()
                .margin(1)
                .constraints(
                    [
                        Constraint::Length(8),
                        Constraint::Max(80),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(f.size());

            let top = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [
                        Constraint::Percentage(30),
                        Constraint::Percentage(40),
                        Constraint::Percentage(30),
                    ]
                    .as_ref(),
                )
                .split(layout[0]);

            f.render_widget(Block::default().borders(Borders::ALL), top[0]);

            let top_mid = Layout::default()
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(top[1]);

            f.render_widget(title(), top_mid[0]);
            f.render_widget(Block::default().borders(Borders::ALL), top_mid[1]);

            f.render_widget(Block::default().borders(Borders::ALL), top[2]);

            let body = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                .split(layout[1]);

            f.render_widget(Block::default().borders(Borders::ALL), body[0]);
            f.render_widget(Block::default().borders(Borders::ALL), body[1]);

            f.render_widget(Block::default().borders(Borders::ALL), layout[2]);
        })
        .unwrap();
}

fn title() -> Paragraph<'static> {
    Paragraph::new(Text::styled(
        "REPEEKOOZ\n\nhttps://github.com/kaixinbaba/repeekooz",
        Style::default().fg(Color::Yellow),
    ))
    .alignment(Alignment::Center)
    .block(Block::default())
}
