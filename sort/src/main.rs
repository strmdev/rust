use std::{ error::Error, io };
use crossterm::{
    execute,
    terminal::{ enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    event::{ self, Event, KeyCode }
};
use tui::{
    backend::{ Backend, CrosstermBackend },
    layout::{ Layout, Direction, Constraint },
    style::{ Color, Style, Modifier },
    widgets::{ Block, Borders, BorderType, List, ListItem },
    text::Span,
    Terminal,
    Frame
};
use tui_sort_app::sort_algorithms;
use tui_sort_app::sort_algorithms::list_type;

struct AlgorithmNameList<'a> {
    items: list_type::StatefulList<&'a str>
}

impl<'a> AlgorithmNameList<'a> {
    fn new() -> AlgorithmNameList<'a> {
        AlgorithmNameList {
            items: list_type::StatefulList::with_items(vec![
                "Egyszeru cseres rendezes",
                "Minimum kivalasztasos rendezes",
                "Buborekos rendezes",
                "Javitott buborekos rendezes",
                "Beilleszteses rendezes",
                "Gnome rendezes",
                "------------------------------",
                r#"Kilepes ("q")"#
            ])
        }
    }
}

fn main_screen<B: Backend>(frame: &mut Frame<B>, list_of_sort_algorithm_names: &mut AlgorithmNameList) {
    let main_layout = Layout::default()
                             .direction(Direction::Vertical)
                             .constraints([Constraint::Percentage(100)].as_ref())
                             .split(frame.size());

    let items: Vec<ListItem> = list_of_sort_algorithm_names.items
                                                           .items
                                                           .iter()
                                                           .map(|text| { ListItem::new(&**text).style(Style::default()
                                                                                                            .fg(Color::White)
                                                                                                            .bg(Color::DarkGray))
                                                                       }
                                                           )
                                                           .collect();

    let list_block = List::new(items)
                          .block(Block::default()
                                       .borders(Borders::ALL)
                                       .border_type(BorderType::Rounded)
                                       .border_style(Style::default()
                                                           .fg(Color::Green))
                                       .title(Span::styled("Rendezesi algoritmusok", Style::default()
                                                                                           .fg(Color::Cyan)
                                                                                           .add_modifier(Modifier::BOLD))))
                          .highlight_style(Style::default()
                                                 .bg(Color::Cyan)
                                                 .fg(Color::Black)
                                                 .add_modifier(Modifier::BOLD))
                          .highlight_symbol(">> ");
    
    frame.render_stateful_widget(list_block, main_layout[0], &mut list_of_sort_algorithm_names.items.state);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, numbers: &mut Vec<u64>) -> io::Result<()> { 
    let mut list_of_sort_algorithm_names = AlgorithmNameList::new();
    list_of_sort_algorithm_names.items.state.select(Some(0));  

    loop {     
        terminal.draw(|frame| main_screen(frame, &mut list_of_sort_algorithm_names))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down      => list_of_sort_algorithm_names.items.next(),
                KeyCode::Up        => list_of_sort_algorithm_names.items.previous(),
                KeyCode::Enter     => match list_of_sort_algorithm_names.items.state.selected() {
                    Some(0) => sort_algorithms::simple_sort(numbers, terminal),
                    Some(1) => sort_algorithms::min_sort(numbers, terminal),
                    Some(2) => sort_algorithms::bubble_sort(numbers, terminal),
                    Some(3) => sort_algorithms::opt_bubble_sort(numbers, terminal),
                    Some(4) => sort_algorithms::insert_sort(numbers, terminal),
                    Some(5) => sort_algorithms::gnome_sort(numbers, terminal),
                    Some(7) => return Ok(()),
                    _ => {}
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend      = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut numbers: Vec<u64> = Vec::new();
    let run_app_result = run_app(&mut terminal, &mut numbers);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(error_message) = run_app_result {
        println!("Hiba: {}", error_message)
    }

    Ok(())
}