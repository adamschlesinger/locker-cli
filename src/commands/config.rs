use crate::commands::{ Config, CLICommand };
use crate::lfs::*;

use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders},
    layout::{Layout, Constraint, Direction},
    Terminal,
    Frame,
};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{fs, io, thread, time::Duration};
use std::error::Error;
use tui::backend::Backend;
use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::text::Span;
use tui::widgets::BorderType;


impl CLICommand for Config {
    fn exec(&self) {
        show_config();
    }
}

pub fn show_config() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();

    // setup terminal
    enable_raw_mode()?;
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture
    )?;

    let mut terminal = Terminal::new(
        CrosstermBackend::new(stdout)
    )?;

    // create app and run it
    let res = run_app(&mut terminal);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("{:?}", err)
    }

    Ok(())
}

fn load_git_attributes() {
    let contents = fs::read_to_string("")
        .expect("Something went wrong reading the file");
}

fn locked_types() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();

    vec
}

fn locked_files() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    vec
}

fn unlocked_types() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    vec
}

fn unlocked_files() -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    vec
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    loop {
        terminal.draw(ui)?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();

    // Surrounding block
    f.render_widget(
        Block::default()
            .borders(Borders::ALL)
            .title("Main block with round corners")
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded),
        size
    );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(4)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(f.size());

    // Top two inner blocks
    let top_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[0]);

    // Top left inner block with green background
    let block = Block::default()
        .title(vec![
            Span::styled("With", Style::default().fg(Color::Red)),
            Span::from(" background"),
        ])
        .style(Style::default().bg(Color::Green));
    f.render_widget(block, top_chunks[0]);

    // Top right inner block with styled title aligned to the right
    let block = Block::default()
        .title(Span::styled(
            "Styled title",
            Style::default()
                .fg(Color::White)
                .bg(Color::Red)
                .add_modifier(Modifier::BOLD),
        ))
        .title_alignment(Alignment::Right);
    f.render_widget(block, top_chunks[1]);

    // Bottom two inner blocks
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Bottom left block with all default borders
    let block = Block::default().title("With borders").borders(Borders::ALL);
    f.render_widget(block, bottom_chunks[0]);

    // Bottom right block with styled left and right border
    let block = Block::default()
        .title("With styled borders and doubled borders")
        .border_style(Style::default().fg(Color::Cyan))
        .borders(Borders::LEFT | Borders::RIGHT)
        .border_type(BorderType::Double);
    f.render_widget(block, bottom_chunks[1]);
}