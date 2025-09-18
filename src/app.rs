use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode, MouseEventKind},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
};
use std::fs;

const ADDR_PANE_WIDTH_PERCENTAGE: u16 = 15;
const HEX_PANE_WIDTH_PERCENTAGE: u16 = 50;
const ASCII_PANE_WIDTH_PERCENTAGE: u16 = 35;

pub struct App {
    line_idx: usize,
    // content of the opened file
    pub data: Vec<u8>,
    pub size: usize,
    // exit state
    pub quit: bool,

    // inherited from flags
    pub alignment: usize,
    #[allow(dead_code)]
    filename: String,
}

impl App {
    pub fn new(filename: String, alignment: usize) -> anyhow::Result<App> {
        let content = fs::read(&filename)?;
        let size = content.len();

        Ok(App {
            data: content,
            size,
            alignment,
            filename,
            ..App::default()
        })
    }

    pub fn draw(&self, frame: &mut Frame) {
        // layout
        let screen = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(ADDR_PANE_WIDTH_PERCENTAGE),
                Constraint::Percentage(HEX_PANE_WIDTH_PERCENTAGE),
                Constraint::Percentage(ASCII_PANE_WIDTH_PERCENTAGE),
            ])
            .split(frame.area());

        let f_height: usize = frame.area().height as usize;
        let start_line_idx = self.line_idx;
        let mut end_line_idx = self.line_idx + f_height;
        if end_line_idx * self.alignment > self.size {
            end_line_idx = self.size / self.alignment + 1;
        }

        // --- Address view
        let address_block = Block::default()
            .title("Address")
            .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM);

        let address = self.get_address_to_lines(start_line_idx, end_line_idx);
        let address_view = Paragraph::new(address)
            .block(address_block)
            .alignment(ratatui::layout::Alignment::Right);

        frame.render_widget(address_view, screen[0]);

        // --- Hex view
        let hex_block = Block::default().title("Hex").borders(Borders::ALL);

        let hexdump = self.get_hexdump(start_line_idx, end_line_idx);
        let hex_view = Paragraph::new(hexdump).block(hex_block);

        frame.render_widget(hex_view, screen[1]);

        // --- Ascii view
        let ascii_block = Block::default()
            .title("Ascii")
            .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM);

        let asciidump = self.get_asciidump(start_line_idx, end_line_idx);
        let ascii_view = Paragraph::new(asciidump).block(ascii_block);
        frame.render_widget(ascii_view, screen[2]);
    }

    pub fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        if let Event::Key(key) = event {
            match key.code {
                KeyCode::Char('q') => self.quit = true,
                KeyCode::Char('j') => {
                    if self.line_idx < self.size / self.alignment {
                        self.line_idx += 1;
                    }
                }
                KeyCode::Char('k') => {
                    self.line_idx = self.line_idx.saturating_sub(1);
                }
                _ => {}
            }
        }

        if let Event::Mouse(mouse) = event {
            match mouse.kind {
                MouseEventKind::ScrollDown => {
                    if self.line_idx < self.size / self.alignment {
                        self.line_idx += 1;
                    }
                }
                MouseEventKind::ScrollUp => {
                    self.line_idx = self.line_idx.saturating_sub(1);
                }
                _ => {}
            }
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            line_idx: 0,
            data: vec![],
            size: 0,
            quit: false,
            alignment: 16,
            filename: String::new(),
        }
    }
}
