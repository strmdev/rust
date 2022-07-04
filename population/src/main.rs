use crossterm::{
    event::{ self, Event, KeyCode },
    execute,
    terminal::{ disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    cursor,
    style::{ PrintStyledContent, Stylize }
};
use std::{ error::Error, io, time::Duration, vec::Vec };
use tui::{
    backend::{ Backend, CrosstermBackend },
    layout::{ Alignment, Constraint, Direction, Layout },
    widgets::{ Block, Borders, BarChart, BorderType, Paragraph },
    style::{ Color, Style, Modifier },
    text::Span,
    Frame,
    Terminal
};
use rand::Rng;
use std::time::{ Duration as TimeDuration, Instant };

const SIMULATION_SPACE_MAX_COLUMN_NUM: usize = 155;
const SIMULATION_SPACE_MAX_ROW_NUM: usize    = 8;
const POPULATION_CHART_MAX_COLUMN_NUM: usize     = 22;
const FOX_AND_RABBIT_CHART_MAX_COLUMN_NUM: usize = 11;

fn main() -> Result<(), Box<dyn Error>> {
    let fox_birth_probability: f64        = read_data("🦊 szuletesenek valoszinusege (0.0..1.0):", "Hiba tortent a beolvasas soran!");
    let fox_mortality_probability: f64    = read_data("🦊 halalozasanak valoszinusege (0.0..1.0):", "Hiba tortent a beolvasas soran!");
    let rabbit_mortality_probability: f64 = read_data("🐰 halalozasanak valoszinusege (0.0..1.0):", "Hiba tortent a beolvasas soran!");

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend      = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let population_app_result = run_population_app(&mut terminal, fox_birth_probability, fox_mortality_probability, rabbit_mortality_probability);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(error_message) = population_app_result {
        eprintln!("Hiba tortent az alkalmazas futtatasa soran: {}", error_message)
    } else {
        println!("\nSzimulacio vege...");
    }

    Ok(())
}

fn run_population_app<B: Backend>(terminal: &mut Terminal<B>, fox_birth_probability: f64, fox_mortality_probability: f64, rabbit_mortality_probability: f64) -> io::Result<()> {
    let mut population: Vec<Vec<&str>> = vec![vec![" ";SIMULATION_SPACE_MAX_COLUMN_NUM];SIMULATION_SPACE_MAX_ROW_NUM];    
    generate_population(&mut population, fox_birth_probability);

    let mut population_data: Vec<(&str, u64)>        = Vec::new();
    let mut fox_population_data: Vec<(&str, u64)>    = Vec::new();
    let mut rabbit_population_data: Vec<(&str, u64)> = Vec::new();
    
    let mut timer: Instant = Instant::now();

    population_data.push(("", get_symbol_count(&population, "R") + get_symbol_count(&population, "N")));
    fox_population_data.push(("", get_symbol_count(&population, "R")));
    rabbit_population_data.push(("", get_symbol_count(&population, "N")));

    loop {
        if timer.elapsed() >= TimeDuration::from_secs(3) { 
            population_data.push(("", get_symbol_count(&population, "R") + get_symbol_count(&population, "N")));
            fox_population_data.push(("", get_symbol_count(&population, "R")));
            rabbit_population_data.push(("", get_symbol_count(&population, "N")));
            
            timer = Instant::now();
        }     

        terminal.draw(|frame| ui(frame, &population, &population_data, &fox_population_data, &rabbit_population_data))?;
        
        if population_data.len() >= POPULATION_CHART_MAX_COLUMN_NUM {
            population_data.clear();
        }

        if fox_population_data.len() >= FOX_AND_RABBIT_CHART_MAX_COLUMN_NUM {
            fox_population_data.clear();
            rabbit_population_data.clear();
        }

        animal_mortality(&mut population, fox_mortality_probability, rabbit_mortality_probability);
        fox_eating(&mut population);

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    return Ok(());
                }
                if key.code == KeyCode::Char('s') || key.code == KeyCode::Char('S') {
                    event::read()?; 
                }
            }
        }
    }
}

fn read_data(message: &str, error_message: &str) -> f64 {
    loop {
        let mut data: String = String::new();
    
        println!("{}", message);
        io::stdin()
           .read_line(&mut data)
           .expect(error_message);
        
        let value: f64 = data.trim()
                             .parse::<f64>()
                             .unwrap();

        if value >= 0.0 && value <= 1.0  {
            break value
        } else {
            println!("Az ertek csak 0.0..1.0 kozott lehet!")
        }
    }
}

fn generate_population(population: &mut Vec<Vec<&str>>, fox_birth_probability: f64) {
    for simulation_space_row in population.iter_mut() {
        for simulation_space_cell in simulation_space_row.iter_mut() {
            let birth_probability: f64 = rand::thread_rng()
                                              .gen_range(0.0..1.0);

            if birth_probability < fox_birth_probability {
                *simulation_space_cell = "R";
            } else {
                *simulation_space_cell = "N";
            }
        }
    }    
}

fn fox_eating(population: &mut Vec<Vec<&str>>) {
    let x_position: usize = rand::thread_rng()
                                 .gen_range(0..155);
    let y_position: usize = rand::thread_rng()
                                 .gen_range(0..8);

    if population[y_position][x_position] == "R" {
        if y_position < 1 && x_position < 1 {
            if population[y_position][x_position + 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position + 1] = "R";
            } else if population[y_position + 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position + 1][x_position] = "R";    
            }
        } else if y_position < 1 && x_position < SIMULATION_SPACE_MAX_COLUMN_NUM - 1 && x_position >= 1 {
            if population[y_position][x_position + 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position + 1] = "R";
            } else if population[y_position][x_position - 1] == "N"{
                population[y_position][x_position]     = " ";
                population[y_position][x_position - 1] = "R";
            } else if population[y_position + 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position + 1][x_position] = "R";    
            }
        } else if x_position < 1 && y_position >= 1 && y_position < SIMULATION_SPACE_MAX_ROW_NUM - 1 {
            if population[y_position - 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position - 1][x_position] = "R";  
            } else if population[y_position + 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position + 1][x_position] = "R";  
            } else if population[y_position][x_position + 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position + 1] = "R";  
            }
        } else if y_position + 1 >= SIMULATION_SPACE_MAX_ROW_NUM && x_position + 1 >= SIMULATION_SPACE_MAX_COLUMN_NUM {
            if population[y_position][x_position - 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position - 1] = "R";
            }
            else if population[y_position - 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position - 1][x_position] = "R";
            }
        } else if y_position == SIMULATION_SPACE_MAX_ROW_NUM - 1 && x_position >= 1 && x_position < SIMULATION_SPACE_MAX_COLUMN_NUM - 1 {
            if population[y_position][x_position - 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position - 1] = "R";
            } else if population[y_position][x_position + 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position + 1] = "R";
            } else if population[y_position - 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position - 1][x_position] = "R";
            }
        } else if x_position == SIMULATION_SPACE_MAX_COLUMN_NUM - 1 && y_position >= 1 && y_position < SIMULATION_SPACE_MAX_ROW_NUM - 1 {
            if population[y_position - 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position - 1][x_position] = "R";
            } else if population[y_position][x_position - 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position - 1] = "R";
            } else if population[y_position + 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position + 1][x_position] = "R";
            }
        } else if y_position < 1 && x_position + 1 >= SIMULATION_SPACE_MAX_COLUMN_NUM {
            if population[y_position][x_position - 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position - 1] = "R";
            }
            else if population[y_position + 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position + 1][x_position] = "R";
            }
        } else if x_position < 1 && y_position + 1 >= SIMULATION_SPACE_MAX_ROW_NUM {
            if population[y_position][x_position + 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position + 1] = "R";
            } else if population[y_position - 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position - 1][x_position] = "R";
            }
        } else {
            if population[y_position - 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position - 1][x_position] = "R";
            } else if population[y_position][x_position - 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position - 1] = "R";
            } else if population[y_position + 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position + 1][x_position] = "R";
            } else if population[y_position][x_position + 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position + 1] = "R";
            }
        }
    }
}

fn animal_mortality(population: &mut Vec<Vec<&str>>, fox_mortality_probability: f64, rabbit_mortality_probability: f64) {
    let x_position: usize = rand::thread_rng()
                                 .gen_range(0..155);
    let y_position: usize = rand::thread_rng()
                                 .gen_range(0..8);

    let rabbit_mortality: f64 = rand::thread_rng()
                                     .gen_range(0.0..1.0);
    let fox_mortality: f64    = rand::thread_rng()
                                     .gen_range(0.0..1.0);
    
    if population[y_position][x_position] == "N" && rabbit_mortality < rabbit_mortality_probability {
        population[y_position][x_position] = "F";
    } else if population[y_position][x_position] == "R" && fox_mortality < fox_mortality_probability {
        population[y_position][x_position] = "F";
    }
}

fn get_symbol_count(population: &Vec<Vec<&str>>, symbol: &str) -> u64 {
    let mut symbol_count: u64 = 0;

    for row in 0..SIMULATION_SPACE_MAX_ROW_NUM {
        for cell in 0..SIMULATION_SPACE_MAX_COLUMN_NUM {
            let symbol_value = population[row][cell];
            if symbol_value == symbol {
                symbol_count += 1;
            } 
        }
    }

    symbol_count
}

fn print_symbol(simulation_space_cell: usize, simulation_space_row: usize, value: &str) {
    if value == "R" {
        execute!(io::stdout(),
                 cursor::MoveTo((simulation_space_cell + 1) as u16, (simulation_space_row + 1) as u16),
                 PrintStyledContent(value.with(crossterm::style::Color::DarkRed))).ok();
    }
    else if value == "N" {
        execute!(io::stdout(),
                 cursor::MoveTo((simulation_space_cell + 1) as u16, (simulation_space_row + 1) as u16),
                 PrintStyledContent(value.with(crossterm::style::Color::Grey))).ok();
    } else if value == "F" {
        execute!(io::stdout(),
                 cursor::MoveTo((simulation_space_cell + 1) as u16, (simulation_space_row + 1) as u16),
                 PrintStyledContent(value.with(crossterm::style::Color::Green))).ok();
    } else if value == " " {
        execute!(io::stdout(),
                 cursor::MoveTo((simulation_space_cell + 1) as u16, (simulation_space_row + 1) as u16),
                 PrintStyledContent(value.with(crossterm::style::Color::Black))).ok();
    }
}

fn show_population(population: &Vec<Vec<&str>>) {
    let mut value: &str;
    for simulation_space_row in 0..SIMULATION_SPACE_MAX_ROW_NUM {
        for simulation_space_cell in 0..SIMULATION_SPACE_MAX_COLUMN_NUM {
            value = population[simulation_space_row][simulation_space_cell];
            print_symbol(simulation_space_cell, simulation_space_row, value);     
        }
    }
}

fn clear_simulation_space() {
    let clear_symbol = " ";
    for simulation_space_row in 0..SIMULATION_SPACE_MAX_ROW_NUM {
        for simulation_space_cell in 0..SIMULATION_SPACE_MAX_COLUMN_NUM {
            print_symbol(simulation_space_cell, simulation_space_row, clear_symbol);     
        }
    }    
}

fn ui<B: Backend>(frame: &mut Frame<B>, population: &Vec<Vec<&str>>, population_data: &Vec<(&str, u64)>, fox_population_data: &Vec<(&str, u64)>, rabbit_population_data: &Vec<(&str, u64)>) {
    let main_screen = Layout::default()
                             .direction(Direction::Vertical)
                             .constraints(
                                             [
                                                 Constraint::Length(10),
                                                 Constraint::Length(20),
                                                 Constraint::Length(20),
                                                 Constraint::Length(2)
                                             ]
                                             .as_ref(),
                             )
                             .split(frame.size());
    
    let simulation_space = Block::default()
                                 .title(Span::styled("🦀 Populaciobiologiai Szimulacios Modell 🦀", Style::default()
                                                                                                          .fg(Color::Cyan)
                                                                                                          .add_modifier(Modifier::BOLD)))
                                 .title_alignment(Alignment::Center)
                                 .borders(Borders::ALL)
                                 .border_type(BorderType::Rounded)
                                 .style(Style::default().fg(Color::LightGreen));

    frame.render_widget(simulation_space, main_screen[0]);
    clear_simulation_space();
    show_population(&population);
        
    let chart_screen = Layout::default()
                              .direction(Direction::Horizontal)
                              .constraints(
                                  [
                                      Constraint::Percentage(50),
                                      Constraint::Percentage(50),
                                  ]
                                  .as_ref(),
                              )
                              .split(main_screen[1]);

    let fox_chart: BarChart = BarChart::default()
                                       .block(Block::default()
                                                    .title(Span::styled(r#"🦊 populacio ("R") szamanak valtozasa"#, Style::default()
                                                                                                                          .fg(Color::Cyan)
                                                                                                                          .add_modifier(Modifier::BOLD)))
                                                    .borders(Borders::ALL)
                                                    .border_type(BorderType::Rounded)
                                                    .border_style(Style::default()
                                                                        .fg(Color::LightGreen))
                                       )
                                       .data(fox_population_data)
                                       .bar_width(6)
                                       .bar_style(Style::default()
                                                        .fg(Color::Red))
                                       .value_style(Style::default()
                                                          .fg(Color::Cyan)
                                                          .add_modifier(Modifier::BOLD));
    frame.render_widget(fox_chart, chart_screen[0]);

    let rabbit_chart: BarChart = BarChart::default()
                                          .block(Block::default()
                                                       .title(Span::styled(r#"🐰 populacio ("N") szamanak valtozasa"#, Style::default()
                                                                                                                             .fg(Color::Cyan)
                                                                                                                             .add_modifier(Modifier::BOLD)))
                                                       .borders(Borders::ALL)
                                                       .border_type(BorderType::Rounded)
                                                       .border_style(Style::default()
                                                                           .fg(Color::LightGreen))
                                          )
                                          .data(rabbit_population_data)
                                          .bar_width(6)
                                          .bar_style(Style::default()
                                                           .fg(Color::DarkGray))
                                          .value_style(Style::default()
                                                             .fg(Color::Cyan)
                                                             .add_modifier(Modifier::BOLD));
    frame.render_widget(rabbit_chart, chart_screen[1]);


    let population_chart: BarChart = BarChart::default()
                                              .block(Block::default()
                                                           .title(Span::styled("🦊 + 🐰 populacio szamanak valtozasa", Style::default()
                                                                                                                             .fg(Color::Cyan)
                                                                                                                             .add_modifier(Modifier::BOLD)))
                                                           .borders(Borders::ALL)
                                                           .border_type(BorderType::Rounded)
                                                           .border_style(Style::default()
                                                                               .fg(Color::LightGreen))
                                              )
                                              .data(population_data)
                                              .bar_width(6)
                                              .bar_style(Style::default()
                                                               .fg(Color::LightYellow))
                                              .value_style(Style::default()
                                                                 .fg(Color::Cyan)
                                                                 .add_modifier(Modifier::BOLD));
    frame.render_widget(population_chart, main_screen[2]);

    let info_screen = Layout::default()
                             .direction(Direction::Vertical)
                             .constraints(
                                 [
                                     Constraint::Length(2),
                                 ]
                                 .as_ref(),
                             )
                             .split(main_screen[3]);
    
    let info_block = Block::default()
                           .title(Span::styled("🚦 Muveletek 🚦", Style::default()
                                                                        .fg(Color::Cyan)
                                                                        .add_modifier(Modifier::BOLD)))
                           .title_alignment(Alignment::Center)
                           .border_style(Style::default()
                                               .fg(Color::LightGreen))
                           .border_type(BorderType::Rounded)
                           .borders(Borders::ALL);

    let info_paragraph = Paragraph::new(Span::from("Kilepes: <ESC> | Szimulacio megallitasa / inditasa: <S>"))
                                   .style(Style::default()
                                                .fg(Color::White))
                                   .block(info_block)
                                   .alignment(Alignment::Center);

    frame.render_widget(info_paragraph, info_screen[0]);
}   