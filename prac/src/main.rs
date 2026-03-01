use std::{error::Error, time::Duration};

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode,
        EnterAlternateScreen, LeaveAlternateScreen,
    },
};

use ratatui::{
    prelude::*,
    widgets::*,
};

struct App {
    items: Vec<String>,
    filtered: Vec<String>,
    input: String,
    selected: usize,
    selected_output: String,
}

impl App {
    fn new() -> Self {
        let items = vec![
            "Apple", "Banana", "Orange", "Grape", "Mango",
            "Pineapple", "Watermelon", "Strawberry", "Kiwi",
            "Blueberry", "Peach", "Cherry", "Papaya"
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();

        Self {
            filtered: items.clone(),
            items,
            input: String::new(),
            selected: 0,
            selected_output: "Press Enter to select".into(),
        }
    }

    fn filter(&mut self) {
        self.filtered = self
            .items
            .iter()
            .filter(|item| item.to_lowercase().contains(&self.input.to_lowercase()))
            .cloned()
            .collect();

        if self.selected >= self.filtered.len() {
            self.selected = 0;
        }
    }

    fn update(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char(c) => {
                self.input.push(c);
                self.filter();
            }
            KeyCode::Backspace => {
                self.input.pop();
                self.filter();
            }
            KeyCode::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyCode::Down => {
                if self.selected + 1 < self.filtered.len() {
                    self.selected += 1;
                }
            }
            KeyCode::Enter => {
                if let Some(item) = self.filtered.get(self.selected) {
                    self.selected_output = format!("Selected: {}", item);
                }
            }
            _ => {}
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        // Top / Middle / Bottom layout
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // search bar
                Constraint::Min(1),    // list
                Constraint::Length(1), // scrollbar
            ])
            .split(area);

        // 🔝 Search bar
        let search = Paragraph::new(self.input.as_str())
            .block(Block::default().title("Search").borders(Borders::ALL));
        frame.render_widget(search, chunks[0]);

        // 📋 List
        let items: Vec<ListItem> = self
            .filtered
            .iter()
            .map(|i| ListItem::new(i.clone()))
            .collect();

        let mut state = ListState::default();
        state.select(Some(self.selected));

        let list = List::new(items)
            .block(Block::default().title("Results").borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("> ");

        frame.render_stateful_widget(list, chunks[1], &mut state.clone());

        // 🔻 Scrollbar
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight);

        let mut scrollbar_state =
            ScrollbarState::new(self.filtered.len()).position(self.selected);

        frame.render_stateful_widget(scrollbar, chunks[1], &mut scrollbar_state);

        // Bottom status
        let status = Paragraph::new(self.selected_output.as_str())
            .alignment(Alignment::Center);

        frame.render_widget(status, chunks[2]);
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    loop {
        terminal.draw(|f| app.draw(f))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
                app.update(key.code);
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
