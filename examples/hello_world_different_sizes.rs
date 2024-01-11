use std::{io::stdout, thread::sleep, time::Duration};

use anyhow::Result;
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};
use tui_big_text::BigTextBuilder;

fn main() -> Result<()> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.draw(|frame| render(frame).expect("failed to render"))?;
    sleep(Duration::from_secs(5));
    terminal.clear()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn render(frame: &mut Frame) -> Result<()> {
    // Setup layout for 4 blocks
    let outer_layout =
        Layout::new(Direction::Vertical, [Constraint::Ratio(1, 2); 2]).split(frame.size());

    let inner_layout_top =
        Layout::new(Direction::Horizontal, [Constraint::Ratio(1, 2); 2]).split(outer_layout[0]);

    let inner_layout_bottom =
        Layout::new(Direction::Horizontal, [Constraint::Ratio(1, 2); 2]).split(outer_layout[1]);

    // render one block for each font size
    {
        // Draw block showing Full size
        let big_text_full = BigTextBuilder::default()
            .font_size(tui_big_text::BigTextSize::Full)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        frame.render_widget(big_text_full, inner_layout_top[0]);
    }

    {
        // Draw block showing HalfWidth size
        let big_text_half_width = BigTextBuilder::default()
            .font_size(tui_big_text::BigTextSize::HalfWidth)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        frame.render_widget(big_text_half_width, inner_layout_top[1]);
    }

    {
        // Draw block showing HalfHeight size
        let big_text_half_height = BigTextBuilder::default()
            .font_size(tui_big_text::BigTextSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        frame.render_widget(big_text_half_height, inner_layout_bottom[0]);
    }

    {
        // Draw block showing Half size
        let big_text_half_size = BigTextBuilder::default()
            .font_size(tui_big_text::BigTextSize::Half)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        frame.render_widget(big_text_half_size, inner_layout_bottom[1]);
    }

    Ok(())
}
