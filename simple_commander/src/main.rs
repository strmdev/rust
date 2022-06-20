use std::{ io, error::Error };
use crossterm::{
    execute,
    terminal::{ enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    event::{ self, Event, KeyCode }
};
use tui::{
    backend::{ CrosstermBackend, Backend },
    layout::{ Layout, Direction, Constraint, Alignment },
    style::{ Style, Color, Modifier },
    widgets::{ Block, Borders, BorderType },
    text::Span,
    Terminal,
    Frame
};

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend      = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
 
    let simple_commander_app_result = run_simple_commander_app(&mut terminal);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(error_message) = simple_commander_app_result {
        println!("Hiba: {}", error_message)        
    }

    Ok(())
}

fn run_simple_commander_app<B>(terminal: &mut Terminal<B>) -> io::Result<()>
                           where B: Backend {
    loop {
        terminal.draw(|frame| file_browser_screen(frame))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
}

fn file_browser_screen<B>(frame: &mut Frame<B>) where B: Backend {
    let main_block = Block::default()
                           .title(Span::styled("Simple Commander - Basic Edition", Style::default()
                                                                                         .fg(Color::Cyan)
                                                                                         .add_modifier(Modifier::BOLD)))
                           .title_alignment(Alignment::Center)
                           .borders(Borders::ALL)
                           .border_type(BorderType::Rounded)
                           .style(Style::default()
                                        .fg(Color::LightGreen));
    frame.render_widget(main_block, frame.size());

    let file_browser_layout = Layout::default()
                                     .direction(Direction::Vertical)
                                     .margin(2)
                                     .constraints([Constraint::Percentage(95), Constraint::Percentage(5)].as_ref())
                                     .split(frame.size());
    let file_browser_windows_layout = Layout::default()
                                             .direction(Direction::Horizontal)
                                             .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
                                             .split(file_browser_layout[0]);
    
    let left_file_window_block = Block::default()
                                       .title(Span::styled("Forras utvonal: ", Style::default()
                                                                                   .fg(Color::Cyan)
                                                                                   .add_modifier(Modifier::BOLD)))
                                       .borders(Borders::ALL)
                                       .border_type(BorderType::Rounded)
                                       .style(Style::default()
                                                    .fg(Color::LightGreen));
    frame.render_widget(left_file_window_block, file_browser_windows_layout[0]);


    let right_file_window_block = Block::default()
                                        .title(Span::styled("Cel utvonal: ", Style::default()
                                                                                    .fg(Color::Cyan)
                                                                                    .add_modifier(Modifier::BOLD)))
                                        .borders(Borders::ALL)
                                        .border_type(BorderType::Rounded)
                                        .style(Style::default()
                                                     .fg(Color::LightGreen));
    frame.render_widget(right_file_window_block, file_browser_windows_layout[1]);

    let operations_block = Block::default()
                                 .title(Span::styled("Muveletek", Style::default()
                                                                        .fg(Color::Cyan)
                                                                        .add_modifier(Modifier::BOLD)))
                                 .title_alignment(Alignment::Center)
                                 .borders(Borders::ALL)
                                 .border_type(BorderType::Rounded)
                                 .style(Style::default()
                                              .fg(Color::LightGreen));
    frame.render_widget(operations_block,file_browser_layout[1]);
}
