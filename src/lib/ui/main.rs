use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tokio;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame, Terminal,
};

use crate::retrieval::city::City;
use crate::retrieval::facts::CityFacts;

struct AppData<'a> {
    pub city: City,
    pub facts: CityFacts<'a>,
}

pub async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let city = City::new(args[1].clone()).await;
    let facts = CityFacts::new(&city).await;
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, &AppData { city, facts });

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, data: &AppData) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &data))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, data: &AppData) {
    // Wrapping block for a group
    // Just draw the block and the group on the same area and build the group
    // with at least a margin of 1
    let size = f.size();
    let theme_color = Color::Cyan;
    // Surrounding block
    let block = Block::default()
        .borders(Borders::empty())
        .title(Span::styled(
            format!(" Query: {} ", data.city.query),
            Style::default()
                .add_modifier(Modifier::UNDERLINED)
                .fg(theme_color),
        ))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme_color));
    f.render_widget(block, size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Percentage(25), Constraint::Percentage(75)].as_ref())
        .split(f.size());

    // Top two inner blocks
    let top_chunk = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(chunks[0]);

    // Top left inner block with green background
    let block = Block::default()
        .borders(Borders::ALL)
        .title(Span::styled(
            format!(" Facts : {} ", data.facts.full_name),
            Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(theme_color),
        ))
        .title_alignment(Alignment::Center)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme_color));
    let text = format!(
        "Country: {}\nFullname: {}\nPopulation: {}\nTimezone: {}\n24 hour temperature average: {:.1}°C\nLocation:\n    latitude: {},\n    longitude: {}\n    Geohash: {}",
        data.facts.country, data.facts.full_name, data.facts.population, data.facts.timezone,data.facts.weather.avg_24_h,data.facts.latitude,data.facts.longitude,data.facts.geohash
    );

    let paragraph = Paragraph::new(text)
        .style(Style::default())
        .block(block)
        .alignment(Alignment::Left);
    f.render_widget(paragraph, top_chunk[0]);

    // Top right inner block with styled title aligned to the right

    // Bottom two inner blocks
    let bottom_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(chunks[1]);

    // Bottom left block with all default border
    let block = Block::default()
        .title(Span::styled(
            format!(" Salaries Percentile 50%: {} ", data.facts.full_name),
            Style::default()
                .fg(theme_color)
                .add_modifier(Modifier::BOLD),
        ))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme_color))
        .title_alignment(Alignment::Center);
    let paragraph = Paragraph::new(data.facts.salaries.data.clone())
        .style(Style::default())
        .block(block)
        .alignment(Alignment::Left);
    f.render_widget(paragraph, bottom_chunks[1]);

    // Bottom right block with styled left and right border
    let block = Block::default()
        .title(Span::styled(
            format!(" Scores: {} ", data.facts.full_name),
            Style::default()
                .fg(theme_color)
                .add_modifier(Modifier::BOLD),
        ))
        .border_style(Style::default())
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(theme_color))
        .title_alignment(Alignment::Center);

    let paragraph = Paragraph::new(data.facts.scores.data.clone())
        .style(Style::default())
        .block(block)
        .alignment(Alignment::Left);
    f.render_widget(paragraph, bottom_chunks[0]);
}
