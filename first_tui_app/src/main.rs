use crossterm::{
    event::{
        self,
        Event,
        KeyCode
    },
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    cursor,
    style::{
        PrintStyledContent,
        Stylize
    }
};

use std::{
    error::Error,
    io,
    time::Duration,
    vec::Vec
};

use tui::{
    backend::{
        Backend,
        CrosstermBackend
    },
    layout::{
        Alignment,
        Constraint,
        Direction,
        Layout
    },
    widgets::{
        Block,
        Borders,
        BarChart,
        BorderType,
        Paragraph
    },
    style::{
        Color,
        Style,
        Modifier
    },
    text::Span,
    Frame,
    Terminal,
};

use rand::Rng;

const MAX_COLUMN_NUM: usize = 155;
const MAX_ROW_NUM: usize    = 8;   

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    
    let run_app_result = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(error_message) = run_app_result {
        println!("{:?}", error_message)
    }

    Ok(())
}

fn generate_population(population: &mut Vec<Vec<&str>>) {
    for simulation_space_row in population.iter_mut() {
        for simulation_space_cell in simulation_space_row.iter_mut() {
            let birth_probability: f64 = rand::thread_rng()
                                              .gen_range(0.0..1.0);
            if birth_probability >= 0.5 {
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
        } else if y_position < 1 && x_position < MAX_COLUMN_NUM - 1 && x_position >= 1 {
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
        } else if x_position < 1 && y_position >= 1 && y_position < MAX_ROW_NUM - 1 {
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
        } else if y_position + 1 >= MAX_ROW_NUM && x_position + 1 >= MAX_COLUMN_NUM {
            if population[y_position][x_position - 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position - 1] = "R";
            }
            else if population[y_position - 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position - 1][x_position] = "R";
            }
        } else if y_position == MAX_ROW_NUM - 1 && x_position >= 1 && x_position < MAX_COLUMN_NUM - 1 {
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
        } else if x_position == MAX_COLUMN_NUM - 1 && y_position >= 1 && y_position < MAX_ROW_NUM - 1 {
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
        } else if y_position < 1 && x_position + 1 >= MAX_COLUMN_NUM {
            if population[y_position][x_position - 1] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position][x_position - 1] = "R";
            }
            else if population[y_position + 1][x_position] == "N" {
                population[y_position][x_position]     = " ";
                population[y_position + 1][x_position] = "R";
            }
        } else if x_position < 1 && y_position + 1 >= MAX_ROW_NUM {
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut population: Vec<Vec<&str>> = vec![vec![" ";MAX_COLUMN_NUM];MAX_ROW_NUM];    
    generate_population(&mut population);

    loop {
        fox_eating(&mut population);
        terminal.draw(|frame| ui(frame, &population))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if (key.code == KeyCode::Char('q')) || (key.code == KeyCode::Char('Q')) {
                    return Ok(());
                }
                if (key.code == KeyCode::Char('s')) || (key.code == KeyCode::Char('S')) {
                    event::read()?; 
                }
            }
        }
    }
}

fn get_symbol_count(population: &Vec<Vec<&str>>, symbol: &str) -> u64 {
    let mut symbol_count: u64 = 0;

    for row in 0..MAX_ROW_NUM {
        for cell in 0..MAX_COLUMN_NUM {
            let symbol_value = population[row][cell];
            if symbol_value == symbol {
                symbol_count += 1;
            } 
        }
    }

    symbol_count
}

fn print_animal(simulation_space_cell: usize, simulation_space_row: usize, value: &str) {
    if value == "R" {
        execute!(io::stdout(),
                 cursor::MoveTo((simulation_space_cell + 1) as u16, (simulation_space_row + 1) as u16),
                 PrintStyledContent(value.with(crossterm::style::Color::DarkRed))).ok();
    }
    else if value == "N" {
        execute!(io::stdout(),
                 cursor::MoveTo((simulation_space_cell + 1) as u16, (simulation_space_row + 1) as u16),
                 PrintStyledContent(value.with(crossterm::style::Color::Grey))).ok();
    } else if value == " " {
        execute!(io::stdout(),
                 cursor::MoveTo((simulation_space_cell + 1) as u16, (simulation_space_row + 1) as u16),
                 PrintStyledContent(value.with(crossterm::style::Color::Black))).ok();
    }
}

fn show_population(population: &Vec<Vec<&str>>) {
    let mut value: &str;
    for simulation_space_row in 0..MAX_ROW_NUM {
        for simulation_space_cell in 0..MAX_COLUMN_NUM {
            value = population[simulation_space_row][simulation_space_cell];
            print_animal(simulation_space_cell, simulation_space_row, value);     
        }
    }
}

fn clear_simulation_space() {
    let clear_symbol = " ";
    for simulation_space_row in 0..MAX_ROW_NUM {
        for simulation_space_cell in 0..MAX_COLUMN_NUM {
            print_animal(simulation_space_cell, simulation_space_row, clear_symbol);     
        }
    }    
}

fn ui<B: Backend>(frame: &mut Frame<B>, population: &Vec<Vec<&str>>) {
    let main_screen = Layout::default()
                             .direction(Direction::Vertical)
                             .constraints(
                                             [
                                                 Constraint::Length(10),
                                                 Constraint::Length(20),
                                                 Constraint::Length(2),
                                             ]
                                             .as_ref(),
                             )
                             .split(frame.size());
    
    let simulation_space = Block::default()
                                 .title(Span::styled("Populaciobiologiai Szimulacios Modell", Style::default()
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


    let count_fox: u64    = get_symbol_count(&population, "R");
    let count_rabbit: u64 = get_symbol_count(&population, "N");
    let fox_rabbit_data: &[(& str, u64)] = &[
                                                (r#"Rokak ("R") szama"#, count_fox),
                                                (r#"Nyulak ("N") szama"#, count_rabbit)
                                            ];
                        
    let fox_rabbit_chart: BarChart = BarChart::default()
                                              .block(Block::default()
                                                           .title(Span::styled(r#"Rokak ("R") es Nyulak ("N") szamanak alakulasa"#, Style::default()
                                                                                                                                          .fg(Color::Cyan)
                                                                                                                                          .add_modifier(Modifier::BOLD)))
                                                           .borders(Borders::ALL)
                                                           .border_type(BorderType::Rounded)
                                                           .border_style(Style::default()
                                                                               .fg(Color::LightGreen))
                                              )
                                              .data(fox_rabbit_data)
                                              .bar_width(18)
                                              .bar_gap(16)
                                              .bar_style(Style::default()
                                                               .fg(Color::LightMagenta))
                                              .value_style(Style::default()
                                                                 .fg(Color::White)
                                                                 .bg(Color::LightMagenta)
                                                                 .add_modifier(Modifier::BOLD));

    frame.render_widget(fox_rabbit_chart, chart_screen[0]);

    let population_count_diff: u64 = count_fox + count_rabbit;
    let population_count: u64      = population_count_diff + get_symbol_count(&population, " ");
    let population_data: &[(& str, u64)] = &[
                                                ("Populacio indulo szama", population_count),
                                                ("Populacio akt. szama", population_count_diff)
                                            ];

    let population_chart: BarChart = BarChart::default()
                                              .block(Block::default()
                                                           .title(Span::styled("Populacio szamanak valtozasa", Style::default()
                                                                                                                     .fg(Color::Cyan)
                                                                                                                     .add_modifier(Modifier::BOLD)))
                                                           .borders(Borders::ALL)
                                                           .border_type(BorderType::Rounded)
                                                           .border_style(Style::default()
                                                                               .fg(Color::LightGreen))
                                              )
                                              .data(population_data)
                                              .bar_width(22)
                                              .bar_gap(16)
                                              .bar_style(Style::default()
                                                               .fg(Color::LightBlue))
                                              .value_style(Style::default()
                                                                 .fg(Color::White)
                                                                 .bg(Color::Blue)
                                                                 .add_modifier(Modifier::BOLD));

    frame.render_widget(population_chart, chart_screen[1]);

    let info_screen = Layout::default()
                             .direction(Direction::Vertical)
                             .constraints(
                                 [
                                     Constraint::Length(2),
                                 ]
                                 .as_ref(),
                             )
                             .split(main_screen[2]);
    
    let info_block = Block::default()
                           .title(Span::styled("Muveletek", Style::default()
                                                                  .fg(Color::Cyan)
                                                                  .add_modifier(Modifier::BOLD)))
                           .title_alignment(Alignment::Center)
                           .border_style(Style::default()
                                               .fg(Color::LightGreen))
                           .border_type(BorderType::Rounded)
                           .borders(Borders::ALL);

    let info_paragraph = Paragraph::new(Span::from(r#"Kilepes: "q"/"Q" | Stop es Start: "s"/"S""#))
                                   .style(Style::default()
                                                .fg(Color::LightYellow))
                                   .block(info_block)
                                   .alignment(Alignment::Center);

    frame.render_widget(info_paragraph, info_screen[0]);
}   