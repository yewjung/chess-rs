mod cell;
mod game;
mod pieces;

use crate::cell::{CellSelect, CellSelectHistory};
use crate::game::Game;
use crate::pieces::{Coord, Piece};
use color_eyre::Result;
use color_eyre::owo_colors::OwoColorize;
use crossterm::event::{
    self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
    KeyModifiers, MouseButton, MouseEvent, MouseEventKind,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout, Position, Rect};
use ratatui::prelude::Widget;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::Borders;
use ratatui::{
    DefaultTerminal, Frame, Terminal,
    style::Stylize,
    widgets::{Block, Paragraph},
};
use std::io;
use std::time::Instant;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut stdout = io::stdout();
    enable_raw_mode()?;
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let result = App::new().run(&mut terminal);
    // restore terminal
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    disable_raw_mode()?;
    terminal.show_cursor()?;
    result
}

/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    cursor_pos: Option<CursorPosition>,
    cell_select_history: CellSelectHistory,
    game: Game,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.running = true;
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            self.handle_crossterm_events()?;
        }
        Ok(())
    }

    /// Renders the user interface.
    ///
    /// This is where you add new widgets. See the following resources for more information:
    ///
    /// - <https://docs.rs/ratatui/latest/ratatui/widgets/index.html>
    /// - <https://github.com/ratatui/ratatui/tree/main/ratatui-widgets/examples>
    fn render(&mut self, frame: &mut Frame) {
        let [_, right] =
            Layout::horizontal(Constraint::from_percentages([70, 30])).areas(frame.area());
        // multi-line paragraph
        let text = Text::from(vec![
            Line::from(format!(
                "Cursor: {}",
                self.cursor_pos
                    .as_ref()
                    .map(|p| format!("({}, {})", p.pos.x, p.pos.y))
                    .unwrap_or("None".to_owned())
            )),
            Line::from(format!(
                "Selected cell: {}",
                self.cell_select_history
                    .last()
                    .map(|p| format!("({}, {})", p.coord.row, p.coord.col))
                    .unwrap_or("None".to_owned())
            )),
        ]);
        let info = Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Info"));
        frame.render_widget(info, right);
        let board_area = Self::board_area(frame);
        self.render_board(board_area, frame);
    }

    fn board_area(frame: &mut Frame) -> Rect {
        let [_, middle, _] = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Ratio(1, 18),
                    Constraint::Ratio(16, 18),
                    Constraint::Ratio(1, 18),
                ]
                .as_ref(),
            )
            .areas(frame.area());

        let [_, board_area, _, _] = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(
                [
                    Constraint::Ratio(2, 17),
                    Constraint::Ratio(9, 17),
                    Constraint::Ratio(1, 17),
                    Constraint::Ratio(5, 17),
                ]
                .as_ref(),
            )
            .areas(middle);
        board_area
    }

    fn render_board(&mut self, area: Rect, frame: &mut Frame) {
        let width = area.width / 8;
        let height = area.height / 8;
        let border_height = area.height / 2 - (4 * height);
        let border_width = area.width / 2 - (4 * width);

        if self
            .cursor_pos
            .as_ref()
            .is_some_and(|pos| !area.contains(pos.pos))
        {
            self.cell_select_history.push(None);
        }
        let horizontals =
            Layout::vertical(Self::length_constraints(height, border_height)).split(area);
        for i in 0..8 {
            let verticals = Layout::horizontal(Self::length_constraints(width, border_width))
                .split(horizontals[i + 1]);
            for j in 0..8 {
                let cell_area = verticals[j + 1];
                // check if a piece is selected
                if let Some(cursor_pos) = &self.cursor_pos {
                    if cell_area.contains(cursor_pos.pos) {
                        match (
                            self.cell_select_history.last(),
                            &self.game.get_cell((i, j).into()),
                        ) {
                            (None, _) => {
                                // selecting a cell
                                self.cell_select_history
                                    .push(Some(CellSelect::new((i, j).into(), cursor_pos.time)));
                            }
                            (Some(cell), None) => {
                                match &self.game.get_cell(cell.coord) {
                                    // [_] -> [_]
                                    None => self.cell_select_history.push(Some(CellSelect::new(
                                        (i, j).into(),
                                        cursor_pos.time,
                                    ))),
                                    // [*] -> [_]
                                    Some(_) => {
                                        self.game.make_move(cell.coord, (i, j).into());
                                        self.cell_select_history.push(None);
                                        self.cursor_pos = None;
                                    }
                                }
                            }
                            (Some(cell), Some(piece)) => {
                                match self.game.get_cell(cell.coord) {
                                    // [_] -> [*]
                                    None => self.cell_select_history.push(Some(CellSelect::new(
                                        (i, j).into(),
                                        cursor_pos.time,
                                    ))),
                                    // [*] -> [*] but same color, reselecting
                                    Some(prev_piece)
                                        if prev_piece.piece_color() == piece.piece_color() =>
                                    {
                                        self.cell_select_history.push(Some(CellSelect::new(
                                            (i, j).into(),
                                            cursor_pos.time,
                                        )));
                                    }
                                    // [*] -> [*] different color, making the move
                                    Some(_) => {
                                        self.game.make_move(cell.coord, (i, j).into());
                                        self.cell_select_history.push(None);
                                        self.cursor_pos = None;
                                    }
                                }
                            }
                        }
                    }
                }

                // rendering each cell
                let bg_color =
                    if self.cell_select_history.last().map(|c| c.coord) == Some((i, j).into()) {
                        Color::LightGreen
                    } else if (i + j) % 2 == 0 {
                        Color::Gray
                    } else {
                        Color::DarkGray
                    };
                match &self.game.get_cell((i, j).into()) {
                    Some(piece) => {
                        let piece_paragraph = Paragraph::new(piece.to_string())
                            .fg(piece.ratatui_color())
                            .alignment(Alignment::Center)
                            .block(Block::new().style(Style::default().bg(bg_color)));
                        frame.render_widget(piece_paragraph, cell_area);
                    }
                    None => {
                        let block = Block::default().style(Style::default().bg(bg_color));
                        frame.render_widget(block, cell_area);
                    }
                }
            }
        }
    }

    fn length_constraints(length: u16, border_length: u16) -> [Constraint; 10] {
        [
            Constraint::Length(border_length),
            Constraint::Length(length),
            Constraint::Length(length),
            Constraint::Length(length),
            Constraint::Length(length),
            Constraint::Length(length),
            Constraint::Length(length),
            Constraint::Length(length),
            Constraint::Length(length),
            Constraint::Length(border_length),
        ]
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        match event::read()? {
            // it's important to check KeyEventKind::Press to avoid handling key release events
            Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
            Event::Mouse(mouse_event) => self.on_mouse_event(mouse_event),
            Event::Resize(_, _) => {}
            _ => {}
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            // Add other key handlers here.
            _ => {}
        }
    }

    fn on_mouse_event(&mut self, mouse_event: MouseEvent) {
        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                self.cursor_pos = Some(CursorPosition {
                    pos: Position::new(mouse_event.column, mouse_event.row),
                    time: Instant::now(),
                });
            }
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}

#[derive(Debug)]
pub struct CursorPosition {
    pub pos: Position,
    pub time: Instant,
}
