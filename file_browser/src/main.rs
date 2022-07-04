use std::{ io::{ self, ErrorKind }, error::Error, thread, time };
use crossterm::{
    execute,
    terminal::{ enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen },
    event::{ self, Event, KeyCode }
};
use tui::{
    backend::{ CrosstermBackend, Backend },
    layout::{ Layout, Direction, Constraint, Alignment, Rect },
    style::{ Style, Color, Modifier },
    widgets::{ Block, Borders, BorderType, List, ListItem, Paragraph, Wrap, Clear },
    text::{ Span, Spans },
    Terminal,
    Frame
};
use walkdir::{ WalkDir, DirEntry };
use chrono::{ prelude::DateTime, Local };
use std::path::Path;
use open;
use std::ffi::OsStr;
use file_browser_lib::list_type;

struct DirList {
    items: list_type::StatefulList<DirEntry>
}

impl DirList {
    fn new(dir_entries: Vec<DirEntry>) -> DirList {
        DirList {
            items: list_type::StatefulList::with_items(dir_entries)
        }
    }

    pub fn fill_items(&mut self, dir_entries: Vec<DirEntry>) {
        for dir_entry in dir_entries {
            self.items.items.push(dir_entry);
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout(); 
    execute!(stdout, EnterAlternateScreen)?;

    let backend      = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
 
    let file_browser_app_result = run_file_browser_app(&mut terminal);
    
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(error_message) = file_browser_app_result {
        eprintln!("Hiba tortent az alkalmazas futtatasa soran: {}", error_message)        
    }

    Ok(())
}

fn run_file_browser_app<B>(terminal: &mut Terminal<B>) -> io::Result<()> where B: Backend {
    let mut dir_entries: Vec<DirEntry> = Vec::new();
    init_dir_entries(&mut dir_entries);

    let mut dir_list: DirList = DirList::new(dir_entries);
    dir_list.items.state.select(Some(0));

    loop {
        terminal.draw(|frame| file_browser_screen(frame, &mut dir_list))?;
        
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc   => return Ok(()),
                KeyCode::Down  => dir_list.items.next(),
                KeyCode::Up    => dir_list.items.previous(),
                KeyCode::Char('U') | KeyCode::Char('u') => open_finder(&dir_list),
                KeyCode::Enter => next_dir_list(&mut dir_list, terminal),
                KeyCode::Backspace => previous_dir_list(&mut dir_list),
                _ => {}
            }
        }
    }
}

fn is_hidden_file(dir_entry: &DirEntry) -> bool {
    dir_entry.file_name()
             .to_str()
             .map(|file_name| file_name.starts_with('.'))
             .unwrap_or(false)
}

fn init_dir_entries(dir_entries: &mut Vec<DirEntry>) {
    for dir_entry in WalkDir::new("/")
                             .max_depth(1)
                             .into_iter()
                             .filter_entry(|dir| !is_hidden_file(dir)) {
        let dir_entry = dir_entry.unwrap();
        dir_entries.push(dir_entry);
    }
}

fn open_finder(dir_list: &DirList) {
    let selected_item_id = dir_list.items
                                   .state
                                   .selected()
                                   .unwrap_or(0);
    
    let path = dir_list.items
                       .items[selected_item_id]
                       .path()
                       .parent()
                       .unwrap_or(Path::new("/"));
    open::that(path).unwrap();
}

fn next_dir_list<B>(dir_list: &mut DirList, terminal: &mut Terminal<B>) where B: Backend {
    let mut dir_entries: Vec<DirEntry> = Vec::new();

    let selected_item_id = dir_list.items
                                   .state
                                   .selected()
                                   .unwrap_or(0);
    let path = dir_list.items
                       .items[selected_item_id]
                       .path();

    if path.is_dir() {
        for dir_entry in WalkDir::new(path)
                                 .max_depth(1)
                                 .into_iter()
                                 .filter_entry(|dir| !is_hidden_file(dir)) {
            match dir_entry {
                Ok(dir) => {
                    dir_entries.push(dir);
                },
                Err(error) => match error.io_error().unwrap().kind() {
                    ErrorKind::PermissionDenied => {
                        terminal.draw(|frame| {
                            file_browser_screen(frame, dir_list);
                            show_popup(frame, "Hibauzenet", "Hozzaferes megtagadva! Az alkalmazasnak nincs jogosultsaga a mappa tartalmanak eleresehez!");
                        }).ok();
                        thread::sleep(time::Duration::from_millis(3000));
                    },
                    _ => {
                        terminal.draw(|frame| {
                            file_browser_screen(frame, dir_list);
                            show_popup(frame, "Hibauzenet", "Ismeretlen hiba tortent az alkalmazas futasa soran!");
                        }).ok();
                        thread::sleep(time::Duration::from_millis(3000));
                    }
                }
            }
       }
       
       dir_list.items.items.clear();
       dir_list.fill_items(dir_entries);
       dir_list.items.state.select(Some(0));
    }
}

fn previous_dir_list(dir_list: &mut DirList) {
    let mut dir_entries: Vec<DirEntry> = Vec::new();

    let selected_item_id = dir_list.items
                                   .state
                                   .selected()
                                   .unwrap_or(0);

    let path = dir_list.items
                       .items[selected_item_id]
                       .path()
                       .parent()
                       .unwrap_or(Path::new("/"))
                       .parent()
                       .unwrap_or(Path::new("/"));

    for dir_entry in WalkDir::new(path)
                             .max_depth(1)
                             .into_iter()
                             .filter_entry(|dir| !is_hidden_file(dir)) {
                        
        let dir_entry = dir_entry.unwrap();
        dir_entries.push(dir_entry);
    }

    dir_list.items.items.clear();
    dir_list.fill_items(dir_entries);
    dir_list.items.state.select(Some(0));
}

fn file_browser_screen<B>(frame: &mut Frame<B>, dir_list: &mut DirList) where B: Backend {
    let main_block = Block::default()
                           .title(Span::styled("File bongeszo", Style::default()
                                                                      .fg(Color::White)
                                                                      .bg(Color::LightBlue)
                                                                      .add_modifier(Modifier::BOLD)))
                           .title_alignment(Alignment::Center)
                           .borders(Borders::ALL)
                           .border_type(BorderType::Rounded)
                           .style(Style::default()
                                        .fg(Color::LightBlue)
                                        .bg(Color::Yellow));
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
                                            
    let dir_list_items: Vec<ListItem> = dir_list.items
                                                .items
                                                .iter()
                                                .map(|list_item| { 
                                                        let file_name = match list_item.file_name().to_str() {
                                                            Some(name) => {
                                                                if list_item.metadata().unwrap().is_dir() || list_item.metadata().unwrap().is_symlink() {
                                                                    String::from("📂 ") + name
                                                                } else {
                                                                    String::from("📄 ") + name
                                                                }
                                                            },
                                                            None => String::from("")
                                                        };

                                                        ListItem::new(file_name).style(Style::default()
                                                                                             .fg(Color::White)
                                                                                             .bg(Color::DarkGray)) 
                                                })
                                                .collect();

    let left_file_window_block = List::new(dir_list_items)
                                      .block(Block::default()
                                                   .title(Span::styled("Fajlok es mappak", Style::default()
                                                                                                 .fg(Color::White)
                                                                                                 .bg(Color::LightBlue)
                                                                                                 .add_modifier(Modifier::BOLD)))
                                                   .borders(Borders::ALL)
                                                   .border_type(BorderType::Rounded)
                                                   .style(Style::default()
                                                                .fg(Color::LightBlue)
                                                                .bg(Color::DarkGray)))
                                      .highlight_style(Style::default()
                                                             .bg(Color::LightYellow)
                                                             .fg(Color::Black)
                                                             .add_modifier(Modifier::BOLD))
                                      .highlight_symbol(">> ");
    frame.render_stateful_widget(left_file_window_block, file_browser_windows_layout[0], &mut dir_list.items.state);

    let right_file_window_block = Block::default()
                                        .title(Span::styled("Info megjelenitese", Style::default()
                                                                                        .fg(Color::White)
                                                                                        .bg(Color::LightBlue)
                                                                                        .add_modifier(Modifier::BOLD)))
                                        .borders(Borders::ALL)
                                        .border_type(BorderType::Rounded)
                                        .style(Style::default()
                                                     .fg(Color::LightBlue)
                                                     .bg(Color::DarkGray));
    
    let mut file_info: Vec<Spans> = Vec::new();
    init_file_info(&mut file_info, dir_list); 
    
    let file_info_paragraph = Paragraph::new(file_info)
                                        .block(right_file_window_block)
                                        .style(Style::default().fg(Color::LightYellow).bg(Color::DarkGray))
                                        .wrap(Wrap { trim: true });
    frame.render_widget(file_info_paragraph, file_browser_windows_layout[1]);

    let operations_block = Block::default()
                                 .title(Span::styled("Muveletek", Style::default()
                                                                        .fg(Color::White)
                                                                        .bg(Color::LightBlue)
                                                                        .add_modifier(Modifier::BOLD)))
                                 .title_alignment(Alignment::Center)
                                 .borders(Borders::ALL)
                                 .border_type(BorderType::Rounded)
                                 .style(Style::default()
                                              .fg(Color::LightBlue)
                                              .bg(Color::Gray));

    let operations_block_texts = vec![
                                         Spans::from(vec![
                                             Span::raw("Listaban fel: "),
                                             Span::styled("<UP ARROW>",Style::default().add_modifier(Modifier::BOLD)),
                                             Span::raw(" | "),

                                             Span::raw("Listaban le: "),
                                             Span::styled("<DOWN ARROW>",Style::default().add_modifier(Modifier::BOLD)),
                                             Span::raw(" | "),

                                             Span::raw("Listaban vissza: "),
                                             Span::styled("<BACKSPACE>",Style::default().add_modifier(Modifier::BOLD)),
                                             Span::raw(" | "),

                                             Span::raw("Belepes mappaba: "),
                                             Span::styled("<ENTER>",Style::default().add_modifier(Modifier::BOLD)),
                                             Span::raw(" | "),

                                             Span::raw("Ugras a mappahoz: "),
                                             Span::styled("<U>",Style::default().add_modifier(Modifier::BOLD)),
                                             Span::raw(" | "),

                                             Span::raw("Kilepes: "),
                                             Span::styled("<ESC>",Style::default().add_modifier(Modifier::BOLD))
                                         ])
                                 ];

    let operations_paragraph = Paragraph::new(operations_block_texts)
                                         .style(Style::default()
                                                      .fg(Color::Black))
                                         .block(operations_block)
                                         .alignment(Alignment::Center);
    frame.render_widget(operations_paragraph, file_browser_layout[1]);
}

fn init_file_info(spans: &mut Vec<Spans>, dir_list: &mut DirList) {
    let selected_item_id   = dir_list.items.state.selected().unwrap_or(0);
    let selected_item_path = dir_list.items.items[selected_item_id].path().to_str().unwrap_or("-");
    let selected_item_path = String::from("Hely: ") + &selected_item_path;

    let selected_item_extension = dir_list.items.items[selected_item_id].path().extension().unwrap_or(OsStr::new("-")).to_str().unwrap_or("-");
    let selected_item_extension = String::from("Kiterjesztes: ") + &selected_item_extension;
    
    let selected_item_file_size = dir_list.items.items[selected_item_id].metadata().unwrap().len().to_string();
    let selected_item_file_size = String::from("Meret: ") + &selected_item_file_size + &String::from(" bájt");

    let file_modified_system_time = dir_list.items.items[selected_item_id].metadata().unwrap().modified().unwrap();
    let file_modified_date_time   = DateTime::<Local>::from(file_modified_system_time);
    let file_modified_date_time   = String::from("Modositva: ") + &file_modified_date_time.format("%Y-%m-%d, %A %H:%M").to_string();

    let file_created_system_time = dir_list.items.items[selected_item_id].metadata().unwrap().created().unwrap();
    let file_created_date_time   = DateTime::<Local>::from(file_created_system_time);
    let file_created_date_time   = String::from("Letrehozva: ") + &file_created_date_time.format("%Y-%m-%d, %A %H:%M").to_string();

    let is_selected_item_file_str = match dir_list.items.items[selected_item_id].metadata().unwrap().is_file() {
                                        true => String::from("Fajl?: Igen"),
                                        false => String::from("Fajl?: Nem")
                                    };
    let is_selected_item_dir_str = match dir_list.items.items[selected_item_id].metadata().unwrap().is_dir() {
                                       true => String::from("Mappa?: Igen"),
                                       false => String::from("Mappa?: Nem")
                                   };
    let is_selected_item_symlink_str = match dir_list.items.items[selected_item_id].metadata().unwrap().is_symlink() {
                                           true => String::from("Symlink?: Igen"),
                                           false => String::from("Symlink?: Nem")
                                       };

    spans.push(Spans::from(Span::styled(selected_item_extension, Style::default())));
    spans.push(Spans::from(Span::styled(selected_item_file_size, Style::default())));
    spans.push(Spans::from(Span::styled(selected_item_path, Style::default())));
    spans.push(Spans::from(Span::styled(file_created_date_time, Style::default())));
    spans.push(Spans::from(Span::styled(file_modified_date_time, Style::default())));
    spans.push(Spans::from(Span::styled(is_selected_item_file_str, Style::default())));
    spans.push(Spans::from(Span::styled(is_selected_item_dir_str, Style::default())));
    spans.push(Spans::from(Span::styled(is_selected_item_symlink_str, Style::default())));  
}

fn show_popup<B>(frame: &mut Frame<B>, title: &str, content: &str) where B: Backend {
    let popup_block = Block::default()
                            .title(title)
                            .title_alignment(Alignment::Center)
                            .borders(Borders::ALL)
                            .border_type(BorderType::Double)
                            .style(Style::default()
                                         .bg(Color::Red));

    let popup_area = centered_rect(45, 10, frame.size());
    frame.render_widget(Clear, popup_area);

    let content_paragraph = Paragraph::new(Span::from(content))
                                      .style(Style::default()
                                                   .fg(Color::White))
                                      .block(popup_block)
                                      .alignment(Alignment::Center)
                                      .wrap(Wrap { trim: true });

    frame.render_widget(content_paragraph, popup_area);
}

fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    let popup_layout = Layout::default()
                              .direction(Direction::Vertical)
                              .constraints(
                                              [
                                                  Constraint::Percentage((100 - percent_y) / 2),
                                                  Constraint::Percentage(percent_y),
                                                  Constraint::Percentage((100 - percent_y) / 2),
                                              ]
                                              .as_ref()
                                          )
                              .split(rect);

    Layout::default()
           .direction(Direction::Horizontal)
           .constraints(
                           [
                               Constraint::Percentage((100 - percent_x) / 2),
                               Constraint::Percentage(percent_x),
                               Constraint::Percentage((100 - percent_x) / 2),
                           ]
                           .as_ref()
                       )
           .split(popup_layout[1])[1]
}