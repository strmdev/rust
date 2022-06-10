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
        ListState,
        Paragraph
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

fn fill_vector_with_random_numbers(numbers: &mut Vec<u64>) {
    let mut _random_number: u64 = 0;

    for _index in 0..VECTOR_SIZE {
        _random_number = rand::thread_rng()
                              .gen_range(1..10000);
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

    let current_time = time::Instant::now();

    for i_index in 0..numbers.len() - 1 {
        for j_index in i_index + 1..numbers.len() {
            if numbers[i_index] > numbers[j_index] {
                swap(numbers, i_index, j_index);
                terminal.draw(|frame| chart_screen(frame, numbers, "Egyszeru cseres rendezes", String::from(""))).ok();
            }
        }
    }
    
    let elapsed_time = current_time.elapsed().as_millis();
    let elapsed_time = String::from("A rendezési algoritmus végrehajtási ideje: ") + &elapsed_time.to_string() + &String::from(" ms");

    terminal.draw(|frame| chart_screen(frame, numbers, "Egyszeru cseres rendezes", elapsed_time)).ok();
    thread::sleep(time::Duration::from_millis(5000));

    numbers.clear();
    terminal.draw(|frame| chart_screen(frame, numbers, "", String::from(""))).ok();
}

fn min_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
    fill_vector_with_random_numbers(numbers);

    let current_time = time::Instant::now();

    let mut _min_value_index: usize  = 0;

    for i_index in 0..numbers.len() - 1 {
        _min_value_index = i_index;
        for j_index in i_index + 1..numbers.len() {
            if numbers[_min_value_index] > numbers[j_index] {
                _min_value_index = j_index;
            }
        }
        swap(numbers, i_index, _min_value_index);
        terminal.draw(|frame| chart_screen(frame, numbers, "Minimum kivalasztasos rendezes", String::from(""))).ok();
    }

    let elapsed_time = current_time.elapsed().as_millis();
    let elapsed_time = String::from("A rendezési algoritmus végrehajtási ideje: ") + &elapsed_time.to_string() + &String::from(" ms");

    terminal.draw(|frame| chart_screen(frame, numbers, "Minimum kivalasztasos rendezes", elapsed_time)).ok();
    thread::sleep(time::Duration::from_millis(5000));
    
    numbers.clear();
    terminal.draw(|frame| chart_screen(frame, numbers, "", String::from(""))).ok();
}

fn bubble_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
    fill_vector_with_random_numbers(numbers);

    let current_time = time::Instant::now();

    for i_index in (1..numbers.len()).rev() {
        for j_index in 0..i_index {
            if numbers[j_index] > numbers[j_index + 1] {
                swap(numbers, j_index, j_index + 1);
                terminal.draw(|frame| chart_screen(frame, numbers, "Buborekos rendezes", String::from(""))).ok();
            }
        }
    }

    let elapsed_time = current_time.elapsed().as_millis();
    let elapsed_time = String::from("A rendezési algoritmus végrehajtási ideje: ") + &elapsed_time.to_string() + &String::from(" ms");

    terminal.draw(|frame| chart_screen(frame, numbers, "Buborekos rendezes", elapsed_time)).ok();
    thread::sleep(time::Duration::from_millis(5000));

    numbers.clear();
    terminal.draw(|frame| chart_screen(frame, numbers, "", String::from(""))).ok();
}

fn opt_bubble_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
    fill_vector_with_random_numbers(numbers);

    let current_time = time::Instant::now();
    
    let mut _last_swap_index = 0;
    let mut i_index = numbers.len() - 1;
    while i_index >= 1 {
        _last_swap_index = 0;
        for j_index in 0..i_index {
            if numbers[j_index] > numbers[j_index + 1] {
                swap(numbers, j_index, j_index + 1);
                _last_swap_index = j_index;
                terminal.draw(|frame| chart_screen(frame, numbers, "Javitott buborekos rendezes", String::from(""))).ok();
            }
        }
        i_index = _last_swap_index;
    }

    let elapsed_time = current_time.elapsed().as_millis();
    let elapsed_time = String::from("A rendezési algoritmus végrehajtási ideje: ") + &elapsed_time.to_string() + &String::from(" ms");

    terminal.draw(|frame| chart_screen(frame, numbers, "Javitott buborekos rendezes", elapsed_time)).ok();
    thread::sleep(time::Duration::from_millis(5000));

    numbers.clear();
    terminal.draw(|frame| chart_screen(frame, numbers, "", String::from(""))).ok();
}

fn insert_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
    fill_vector_with_random_numbers(numbers);

    let current_time = time::Instant::now();

    for i_index in 1..numbers.len() {
        let mut j_index = i_index - 1;
        while j_index >= 0 && numbers[j_index] > numbers[j_index + 1] {
            swap(numbers, j_index, j_index + 1);
            terminal.draw(|frame| chart_screen(frame, numbers, "Beilleszteses rendezes", String::from(""))).ok();

            if j_index == 0 {
                break;
            } else {
                j_index -= 1;
            }
        }
    }

    let elapsed_time = current_time.elapsed().as_millis();
    let elapsed_time = String::from("A rendezési algoritmus végrehajtási ideje: ") + &elapsed_time.to_string() + &String::from(" ms");

    terminal.draw(|frame| chart_screen(frame, numbers, "Beilleszteses rendezes", elapsed_time)).ok();
    thread::sleep(time::Duration::from_millis(5000));

    numbers.clear();
    terminal.draw(|frame| chart_screen(frame, numbers, "", String::from(""))).ok();
}

fn gnome_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
    fill_vector_with_random_numbers(numbers);

    let current_time = time::Instant::now();

    let mut index = 0;
    while index < numbers.len() {
        if index == 0 {
            index += 1;
        }

        if numbers[index] >= numbers[index - 1] {
            index += 1;
        } else {
            swap(numbers, index, index - 1);
            terminal.draw(|frame| chart_screen(frame, numbers, "Gnome rendezes", String::from(""))).ok();
            index -= 1;
        }
    }

    let elapsed_time = current_time.elapsed().as_millis();
    let elapsed_time = String::from("A rendezési algoritmus végrehajtási ideje: ") + &elapsed_time.to_string() + &String::from(" ms");

    terminal.draw(|frame| chart_screen(frame, numbers, "Gnome rendezes", elapsed_time)).ok();
    thread::sleep(time::Duration::from_millis(5000));

    numbers.clear();
    terminal.draw(|frame| chart_screen(frame, numbers, "", String::from(""))).ok();
}

fn convert_vector_to_tuple_vector(numbers: &Vec<u64>) -> Vec<(&str, u64)> {
    let mut numbers_with_tuple: Vec<(&str, u64)> = Vec::new();

    for number in numbers {
        numbers_with_tuple.push(("", *number));
    }

    numbers_with_tuple
}

fn chart_screen<B: Backend>(frame: &mut Frame<B>, numbers: &Vec<u64>, title: &str, label_elapsed_time: String) {
    let chart_layout = Layout::default()
                              .direction(Direction::Vertical)
                              .constraints(
                                  [
                                      Constraint::Percentage(5),
                                      Constraint::Percentage(95)
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
                                                         .fg(Color::LightGreen))
                                        .value_style(Style::default()
                                                           .fg(Color::White)
                                                           .add_modifier(Modifier::BOLD));
    frame.render_widget(sort_chart, chart_layout[1]);

    let time_block = Block::default()
                           .title(Span::styled("Vegrehajtasi-ido eredmenyablak", Style::default()
                                                                                       .fg(Color::Cyan)
                                                                                       .add_modifier(Modifier::BOLD)))
                            .borders(Borders::ALL)
                            .border_type(BorderType::Rounded)
                            .style(Style::default()
                                         .fg(Color::LightGreen));

    let time_span  = Paragraph::new(Span::from(label_elapsed_time))
                               .style(Style::default()
                                            .fg(Color::LightRed))
                               .block(time_block); 
    frame.render_widget(time_span, chart_layout[0]);
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
                    Some(0) => simple_sort(numbers, terminal),
                    Some(1) => min_sort(numbers, terminal),
                    Some(2) => bubble_sort(numbers, terminal),
                    Some(3) => opt_bubble_sort(numbers, terminal),
                    Some(4) => insert_sort(numbers, terminal),
                    Some(5) => gnome_sort(numbers, terminal),
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