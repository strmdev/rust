use rand::Rng;
use std::{
    error::Error,
    io,
    thread,
    time
};
use crossterm::{
    execute,
    terminal::{
        enable_raw_mode,
        disable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    event::{
        self,
        Event,
        KeyCode
    }
};
use tui::{
    backend::{
        Backend,
        CrosstermBackend
    },
    layout::{
        Layout,
        Direction,
        Constraint
    },
    style::{
        Color,
        Style,
        Modifier
    },
    layout::{
        Alignment
    },
    widgets::{
        Block,
        Borders,
        BorderType,
        BarChart,
        List,
        ListItem,
        ListState
    },
    text::Span,
    Terminal,
    Frame
};

const VECTOR_SIZE: i32 = 39;

struct StatefulList<T> {
    state: ListState,
    items: Vec<T>,
}

impl<T> StatefulList<T> {
    fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    fn next(&mut self) {
        let index = match self.state.selected() {
            Some(index) => {
                if index >= self.items.len() - 1 {
                    0
                } else {
                    index + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(index));
    }

    fn previous(&mut self) {
        let index = match self.state.selected() {
            Some(index) => {
                if index == 0 {
                    self.items.len() - 1
                } else {
                    index - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(index));
    }
}

struct AlgorithmNameList<'a> {
    items: StatefulList<&'a str>
}

impl<'a> AlgorithmNameList<'a> {
    fn new() -> AlgorithmNameList<'a> {
        AlgorithmNameList {
            items: StatefulList::with_items(vec![
                "Egyszeru cseres rendezes",
                "Minimum kivalasztasos rendezes"
            ])
        }
    }
}

fn fill_vector_with_random_numbers(numbers: &mut Vec<u64>) {
    let mut _random_number: u64 = 0;

    for _index in 0..VECTOR_SIZE {
        _random_number = rand::thread_rng()
                              .gen_range(1..1000);
        numbers.push(_random_number);
    }
}

fn swap(numbers: &mut Vec<u64>, i_index: usize, j_index: usize) {
    let value        = numbers[i_index];
    numbers[i_index] = numbers[j_index];
    numbers[j_index] = value;
}

fn simple_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
    fill_vector_with_random_numbers(numbers);

    for i_index in 0..numbers.len() - 1 {
        for j_index in i_index + 1..numbers.len() {
            if numbers[i_index] > numbers[j_index] {
                swap(numbers, i_index, j_index);
                terminal.draw(|frame| chart_screen(frame, numbers, "Egyszeru cseres rendezes")).ok();
                thread::sleep(time::Duration::from_millis(50));
            }
        }
    }
    
    numbers.clear();
    terminal.draw(|frame| chart_screen(frame, numbers, "")).ok();
}

fn min_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
    fill_vector_with_random_numbers(numbers);

    let mut _min_value_index: usize  = 0;

    for i_index in 0..numbers.len() - 1 {
        _min_value_index = i_index;
        for j_index in i_index + 1..numbers.len() {
            if numbers[_min_value_index] > numbers[j_index] {
                _min_value_index = j_index;
            }
        }
        swap(numbers, i_index, _min_value_index);
        terminal.draw(|frame| chart_screen(frame, numbers, "Minimum kivalasztasos rendezes")).ok();
        thread::sleep(time::Duration::from_millis(50));
    }
    
    numbers.clear();
    terminal.draw(|frame| chart_screen(frame, numbers, "")).ok();
}

fn convert_vector_to_tuple_vector(numbers: &Vec<u64>) -> Vec<(&str, u64)> {
    let mut numbers_with_tuple: Vec<(&str, u64)> = Vec::new();

    for number in numbers {
        numbers_with_tuple.push(("", *number));
    }

    numbers_with_tuple
}

fn chart_screen<B: Backend>(frame: &mut Frame<B>, numbers: &Vec<u64>, title: &str) {
    let chart_layout = Layout::default()
                              .direction(Direction::Vertical)
                              .constraints(
                                  [
                                      Constraint::Percentage(100)
                                  ]
                                  .as_ref(),
                              )
                              .split(frame.size());
    
    let sort_block = Block::default()
                           .title(Span::styled(title, Style::default()
                                                            .fg(Color::Cyan)
                                                            .add_modifier(Modifier::BOLD)))
                           .title_alignment(Alignment::Center)
                           .borders(Borders::ALL)
                           .border_type(BorderType::Rounded)
                           .style(Style::default()
                                        .fg(Color::LightGreen));

    let tuple_vector: Vec<(&str, u64)> = convert_vector_to_tuple_vector(numbers);
    let sort_chart: BarChart = BarChart::default()
                                        .block(sort_block)
                                        .data(&tuple_vector)
                                        .bar_width(5)
                                        .bar_style(Style::default()
                                                         .fg(Color::Red))
                                        .value_style(Style::default()
                                                           .fg(Color::Cyan)
                                                           .add_modifier(Modifier::BOLD));
    frame.render_widget(sort_chart, chart_layout[0]);
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
                                                                                                            .fg(Color::Black)
                                                                                                            .bg(Color::White))
                                                                       }
                                                           )
                                                           .collect();

    let list_block = List::new(items)
                          .block(Block::default()
                                       .borders(Borders::ALL)
                                       .title("Rendezesi algoritmusok"))
                          .highlight_style(Style::default()
                                                 .bg(Color::LightGreen)
                                                 .add_modifier(Modifier::BOLD))
                          .highlight_symbol(">> ");
    frame.render_stateful_widget(list_block, main_layout[0], &mut list_of_sort_algorithm_names.items.state);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, numbers: &mut Vec<u64>) -> io::Result<()> { 
    let mut list_of_sort_algorithm_names = AlgorithmNameList::new();

    loop {       
        terminal.draw(|frame| main_screen(frame, &mut list_of_sort_algorithm_names))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down      => list_of_sort_algorithm_names.items.next(),
                KeyCode::Up        => list_of_sort_algorithm_names.items.previous(),
                KeyCode::Enter     => match list_of_sort_algorithm_names.items.state.selected() {
                    Some(0) => simple_sort(numbers, terminal),
                    Some(1) => min_sort(numbers, terminal),
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