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
    sleep(Duration::from_secs(25));
    terminal.clear()?;
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

fn render(frame: &mut Frame) -> Result<()> {
    // Setup layout for 4 blocks
    let outer_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(frame.size());

    let inner_layout_top = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(outer_layout[0]);

    let inner_layout_bottom = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(outer_layout[1]);

    // render one block for each font size
    {
        // Draw block showing Full size
        let block_full = Block::new()
            .borders(Borders::ALL)
            .title(" BigTextSize::Full ");
        let big_text_full = BigTextBuilder::default()
            .font_size(tui_big_text::BigTextSize::Full)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        let text_area = block_full.inner(inner_layout_top[0]);
        frame.render_widget(block_full, inner_layout_top[0]);
        frame.render_widget(big_text_full, text_area);
    }

    {
        // Draw block showing HalfHeight size
        let block_half_height = Block::new()
            .borders(Borders::ALL)
            .title(" BigTextSize::HalfHeight ");
        let big_text_half_height = BigTextBuilder::default()
            .font_size(tui_big_text::BigTextSize::HalfHeight)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        let text_area = block_half_height.inner(inner_layout_top[1]);
        frame.render_widget(block_half_height, inner_layout_top[1]);
        frame.render_widget(big_text_half_height, text_area);
    }

    {
        // Draw block showing HalfWidth size
        let block_half_width = Block::new()
            .borders(Borders::ALL)
            .title(" BigTextSize::HalfWidth ");
        let big_text_half_width = BigTextBuilder::default()
            .font_size(tui_big_text::BigTextSize::HalfWidth)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        let text_area = block_half_width.inner(inner_layout_bottom[0]);
        frame.render_widget(block_half_width, inner_layout_bottom[0]);
        frame.render_widget(big_text_half_width, text_area);
    }

    {
        // Draw block showing Half size
        let block_half_size = Block::new()
            .borders(Borders::ALL)
            .title(" BigTextSize::Half ");
        let big_text_half_size = BigTextBuilder::default()
            .font_size(tui_big_text::BigTextSize::Half)
            .style(Style::new().blue())
            .lines(vec![
                "Hello".red().into(),
                "World".white().into(),
                "~~~~~".into(),
            ])
            .build()?;
        let text_area = block_half_size.inner(inner_layout_bottom[1]);
        frame.render_widget(block_half_size, inner_layout_bottom[1]);
        frame.render_widget(big_text_half_size, text_area);
    }

    Ok(())
}
