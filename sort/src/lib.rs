pub mod sort_algorithms {

    use std::{ time, thread };
    use tui::{ widgets::{ Block, BarChart, BorderType, Borders, Paragraph },
               backend::Backend,
               layout::{ Layout, Direction, Constraint, Alignment },
               style::{ Style, Color, Modifier },
               text::Span,
               Frame,
               Terminal };
    use rand::Rng;

    const MAX_VECTOR_SIZE: i32 = 39;

    fn fill_vector_with_random_numbers(numbers: &mut Vec<u64>) {
        let mut _random_number: u64 = 0;
    
        for _index in 0..MAX_VECTOR_SIZE {
            _random_number = rand::thread_rng()
                                  .gen_range(1..10000);
            numbers.push(_random_number);
        }
    }
    
    fn convert_vector_to_tuple_vector(numbers: &Vec<u64>) -> Vec<(&str, u64)> {
        let mut numbers_with_tuple: Vec<(&str, u64)> = Vec::new();
    
        for number in numbers {
            numbers_with_tuple.push(("", *number));
        }
    
        numbers_with_tuple
    }

    fn swap(numbers: &mut Vec<u64>, i_index: usize, j_index: usize) {
        let value        = numbers[i_index];
        numbers[i_index] = numbers[j_index];
        numbers[j_index] = value;
    }

    fn clear_chart_screen<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
        numbers.clear();
        terminal.draw(|frame| chart_screen(frame, numbers, "", String::from(""))).ok();
    }

    fn draw_elapsed_time<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>, title: &str, elapsed_time_millis: String) {
        let elapsed_time = String::from("A rendezési algoritmus végrehajtási ideje: ") + &elapsed_time_millis + &String::from(" ms");
    
        terminal.draw(|frame| chart_screen(frame, numbers, title, elapsed_time)).ok();
        thread::sleep(time::Duration::from_millis(5000));
    }
    
    pub fn simple_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
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
        
        let elapsed_time_millis = current_time.elapsed().as_millis().to_string();
        draw_elapsed_time(numbers, terminal, "Egyszeru cseres rendezes", elapsed_time_millis);
    
        clear_chart_screen(numbers, terminal);
    }
    
    pub fn min_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
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
    
        let elapsed_time_millis = current_time.elapsed().as_millis().to_string();
        draw_elapsed_time(numbers, terminal, "Minimum kivalasztasos rendezes", elapsed_time_millis);
    
        clear_chart_screen(numbers, terminal);
    }
    
    pub fn bubble_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
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
    
        let elapsed_time_millis = current_time.elapsed().as_millis().to_string();
        draw_elapsed_time(numbers, terminal, "Buborekos rendezes", elapsed_time_millis);
    
        clear_chart_screen(numbers, terminal);
    }
    
    pub fn opt_bubble_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) { 
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
    
        let elapsed_time_millis = current_time.elapsed().as_millis().to_string();
        draw_elapsed_time(numbers, terminal, "Javitott buborekos rendezes", elapsed_time_millis);
    
        clear_chart_screen(numbers, terminal);
    }
    
    pub fn insert_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
        fill_vector_with_random_numbers(numbers);

        let current_time = time::Instant::now();
    
        for i_index in 1..numbers.len() {
            let mut j_index = i_index - 1;
            while (j_index as i32) >= 0 && numbers[j_index] > numbers[j_index + 1] {
                swap(numbers, j_index, j_index + 1);
                terminal.draw(|frame| chart_screen(frame, numbers, "Beilleszteses rendezes", String::from(""))).ok();
                j_index -= 1;
            }
        }
    
        let elapsed_time_millis = current_time.elapsed().as_millis().to_string();
        draw_elapsed_time(numbers, terminal, "Beilleszteses rendezes", elapsed_time_millis);
    
        clear_chart_screen(numbers, terminal);
    }
    
    pub fn gnome_sort<B: Backend>(numbers: &mut Vec<u64>, terminal: &mut Terminal<B>) {
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
    
        let elapsed_time_millis = current_time.elapsed().as_millis().to_string();
        draw_elapsed_time(numbers, terminal, "Gnome rendezes", elapsed_time_millis);
    
        clear_chart_screen(numbers, terminal);
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

    pub mod list_type {
        use tui::widgets::ListState;

        pub struct StatefulList<T> {
            pub state: ListState,
            pub items: Vec<T>,
        }
        
        impl<T> StatefulList<T> {
            pub fn with_items(items: Vec<T>) -> StatefulList<T> {
                StatefulList {
                    state: ListState::default(),
                    items,
                }
            }
        
            pub fn next(&mut self) {
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
        
            pub fn previous(&mut self) {
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
    }
}