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
    widgets::{ Block, Borders, BorderType, List, ListItem, Paragraph, Wrap },
    text::{ Span, Spans },
    Terminal,
    Frame
};
use simple_commander_lib::list_type;
use walkdir::{ WalkDir, DirEntry };
use std::ffi::OsStr;
use std::path::Path;
use chrono::{ prelude::DateTime, Local };
use open;

struct FileNameList {
    items: list_type::StatefulList<DirEntry>
}

impl FileNameList {
    fn new(files: Vec<DirEntry>) -> FileNameList {
        FileNameList {
            items: list_type::StatefulList::with_items(files)
        }
    }
}

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

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

fn run_simple_commander_app<B>(terminal: &mut Terminal<B>) -> io::Result<()>
                           where B: Backend {
    
    let mut files: Vec<DirEntry> = Vec::new();
    for elem in WalkDir::new("/")
                        .max_depth(1)
                        .into_iter()
                        .filter_entry(|e| !is_hidden(e)) {
        let elem = elem.unwrap();
        files.push(elem.to_owned());
    }

    let mut file_names: FileNameList = FileNameList::new(files.to_owned());
    file_names.items.state.select(Some(0));

    loop {
        terminal.draw(|frame| file_browser_screen(frame, &mut file_names))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc   => return Ok(()),
                KeyCode::Down  => file_names.items.next(),
                KeyCode::Up    => file_names.items.previous(),
                KeyCode::Char('U') | KeyCode::Char('u') => {
                    let selected_item_id = file_names.items.state.selected().unwrap_or(0);
                    let path = file_names.items.items[selected_item_id].path().parent().unwrap_or(Path::new("/"));
                    open::that(path).unwrap();
                },
                KeyCode::Enter => {
                    let selected_item_id = file_names.items.state.selected().unwrap_or(0);
                    let path = file_names.items.items[selected_item_id].path();
                    if path.is_dir() {
                        files.clear();

                        for elem in  WalkDir::new(path)
                                             .max_depth(1)
                                             .into_iter()
                                             .filter_entry(|e| !is_hidden(e)) {
                            let elem = elem.unwrap();
                            files.push(elem.to_owned());
                       }

                       file_names = FileNameList::new(files.to_owned());
                       file_names.items.state.select(Some(0));
                    }
                },
                KeyCode::Left | KeyCode::Backspace => {
                    let selected_item_id = file_names.items.state.selected().unwrap_or(0);
                    let path = file_names.items.items[selected_item_id].path().parent().unwrap_or(Path::new("/"));
                    files.clear();

                    for elem in WalkDir::new(path)
                                        .max_depth(1)
                                        .into_iter()
                                        .filter_entry(|e| !is_hidden(e)) {
                        let elem = elem.unwrap();
                        files.push(elem.to_owned());
                    }

                    file_names = FileNameList::new(files.to_owned());
                    file_names.items.state.select(Some(0));
                }
                _ => {}
            }
        }
    }
}

fn file_browser_screen<B>(frame: &mut Frame<B>, file_names: &mut FileNameList) where B: Backend {
    let main_block = Block::default()
                           .title(Span::styled("File bongeszo", Style::default()
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
                                            
    let items: Vec<ListItem> = file_names.items
                                         .items
                                         .iter()
                                         .map(|text| { 
                                                         let value = match text.file_name().to_str() {
                                                             Some(t) => {
                                                                            if text.metadata().unwrap().is_dir() || text.metadata().unwrap().is_symlink() {
                                                                                String::from("📂 ") + t
                                                                            } else {
                                                                                String::from("📄 ") + t
                                                                            }
                                                                        },
                                                             None => String::from("")
                                                         };
                                                         ListItem::new(value).style(Style::default()
                                                                                          .fg(Color::White)
                                                                                          .bg(Color::DarkGray)) 
                                                     }
                                         )
                                         .collect();

    let left_file_window_block = List::new(items)
                                      .block(Block::default()
                                                   .title(Span::styled("Fajlok es mappak", Style::default()
                                                                                                   .fg(Color::Cyan)
                                                                                                   .add_modifier(Modifier::BOLD)))
                                                   .borders(Borders::ALL)
                                                   .border_type(BorderType::Rounded)
                                                   .style(Style::default()
                                                                .fg(Color::LightGreen)
                                                                .bg(Color::DarkGray)))
                                                   .highlight_style(Style::default()
                                                                          .bg(Color::Cyan)
                                                                          .fg(Color::Black)
                                                                          .add_modifier(Modifier::BOLD))
                                                   .highlight_symbol(">> ");
    frame.render_stateful_widget(left_file_window_block, file_browser_windows_layout[0], &mut file_names.items.state);

    let right_file_window_block = Block::default()
                                        .title(Span::styled("Info megjelenitese", Style::default()
                                                                                        .fg(Color::Cyan)
                                                                                        .add_modifier(Modifier::BOLD)))
                                        .borders(Borders::ALL)
                                        .border_type(BorderType::Rounded)
                                        .style(Style::default()
                                                     .fg(Color::LightGreen));
    
    let selected_item_id        = file_names.items.state.selected().unwrap_or(0);
    let selected_item_path      = file_names.items.items[selected_item_id].path().to_str().unwrap_or("-");
    let selected_item_path      = String::from("Hely: ") + &selected_item_path;

    let selected_item_extension = file_names.items.items[selected_item_id].path().extension().unwrap_or(OsStr::new("-")).to_str().unwrap_or("-");
    let selected_item_extension = String::from("Kiterjesztes: ") + &selected_item_extension;
    
    let selected_item_file_size = file_names.items.items[selected_item_id].metadata().unwrap().len().to_string();
    let selected_item_file_size = String::from("Meret: ") + &selected_item_file_size + &String::from(" bájt");

    let file_modified_system_time = file_names.items.items[selected_item_id].metadata().unwrap().modified().unwrap();
    let file_modified_date_time = DateTime::<Local>::from(file_modified_system_time);
    let file_modified_date_time = String::from("Modositva: ") + &file_modified_date_time.format("%Y-%m-%d, %A %H:%M").to_string();

    let file_created_system_time = file_names.items.items[selected_item_id].metadata().unwrap().created().unwrap();
    let file_created_date_time = DateTime::<Local>::from(file_created_system_time);
    let file_created_date_time = String::from("Letrehozva: ") + &file_created_date_time.format("%Y-%m-%d, %A %H:%M").to_string();

    let is_selected_item_file_str = match file_names.items.items[selected_item_id].metadata().unwrap().is_file() {
                                        true => String::from("Fajl?: Igen"),
                                        false => String::from("Fajl?: Nem")
                                    };
    let is_selected_item_dir_str = match file_names.items.items[selected_item_id].metadata().unwrap().is_dir() {
                                       true => String::from("Mappa?: Igen"),
                                       false => String::from("Mappa?: Nem")
                                   };
    let is_selected_item_symlink_str = match file_names.items.items[selected_item_id].metadata().unwrap().is_symlink() {
                                           true => String::from("Symlink?: Igen"),
                                           false => String::from("Symlink?: Nem")
                                       };

    
    
    let file_info = vec![
                            Spans::from(Span::styled(selected_item_extension, Style::default().fg(Color::Cyan))),
                            Spans::from(Span::styled(selected_item_file_size, Style::default().fg(Color::Cyan))),
                            Spans::from(Span::styled(selected_item_path, Style::default().fg(Color::Cyan))),
                            Spans::from(Span::styled(file_created_date_time, Style::default().fg(Color::Cyan))),
                            Spans::from(Span::styled(file_modified_date_time, Style::default().fg(Color::Cyan))),
                            Spans::from(Span::styled(is_selected_item_file_str, Style::default().fg(Color::Cyan))),
                            Spans::from(Span::styled(is_selected_item_dir_str, Style::default().fg(Color::Cyan))),
                            Spans::from(Span::styled(is_selected_item_symlink_str, Style::default().fg(Color::Cyan)))            
                        ];

    let file_info_paragraph = Paragraph::new(file_info)
                                        .block(right_file_window_block)
                                        .style(Style::default().fg(Color::White).bg(Color::Black))
                                        .wrap(Wrap { trim: true });

    frame.render_widget(file_info_paragraph, file_browser_windows_layout[1]);

    let operations_block = Block::default()
                                 .title(Span::styled("Muveletek", Style::default()
                                                                        .fg(Color::Cyan)
                                                                        .add_modifier(Modifier::BOLD)))
                                 .title_alignment(Alignment::Center)
                                 .borders(Borders::ALL)
                                 .border_type(BorderType::Rounded)
                                 .style(Style::default()
                                              .fg(Color::LightGreen));

    let info_paragraph = Paragraph::new(Span::from(r#"Listaban fel: <UP ARROW> | Listaban le: <DOWN ARROW> | Listaban vissza: <LEFT ARROW> vagy <BACKSPACE> | Belepes mappaba: <ENTER> | Ugras a mappahoz: <U> | Kilepes: <ESC>"#))
                                   .style(Style::default()
                                                .fg(Color::White))
                                   .block(operations_block)
                                   .alignment(Alignment::Center);
    frame.render_widget(info_paragraph, file_browser_layout[1]);
}
