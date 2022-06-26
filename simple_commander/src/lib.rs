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
