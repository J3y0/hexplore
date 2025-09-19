use ratatui::{
    Frame,
    crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind},
    layout::{Constraint, Flex, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};
use std::fs;

use crate::popup::{Popup, centered_rect};

const ADDR_PANE_WIDTH_PERCENTAGE: u16 = 15;
const HEX_PANE_WIDTH_PERCENTAGE: u16 = 50;
const ASCII_PANE_WIDTH_PERCENTAGE: u16 = 35;

const HELP_BODY: &str = r#"
h:        Toggle this help dialog
q:        Quit the application
j:        Move one line down
h:        Move one line up
PageUp:   Move one page up
PageDown: Move one page down
ctrl+u:   Move half page up
ctrl+d:   Move half page down
g:        Go to start
G:        Go to end
"#;

pub struct App {
    pub line_idx: usize,
    pub vertical_margin: usize,
    pub frame_size: (u16, u16),
    pub show_help: bool,
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

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        // layout
        let screen = Layout::vertical([Constraint::Fill(0), Constraint::Length(1)]).split(area);

        let body = Layout::horizontal([
            Constraint::Percentage(ADDR_PANE_WIDTH_PERCENTAGE),
            Constraint::Percentage(HEX_PANE_WIDTH_PERCENTAGE),
            Constraint::Percentage(ASCII_PANE_WIDTH_PERCENTAGE),
        ])
        .split(screen[0]);

        self.update_frame_size(area.height, area.width);

        let start_line_idx = self.line_idx;
        // sub 2 because compared to frame size, only height-2 lines are rendered
        let mut end_line_idx = self.line_idx + area.height as usize - self.vertical_margin;
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

        frame.render_widget(address_view, body[0]);

        // --- Hex view
        let hex_block = Block::default().title("Hex").borders(Borders::ALL);

        let hexdump = self.get_hexdump(start_line_idx, end_line_idx);
        let hex_view = Paragraph::new(hexdump).block(hex_block);

        frame.render_widget(hex_view, body[1]);

        // --- Ascii view
        let ascii_block = Block::default()
            .title("Ascii")
            .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM);

        let asciidump = self.get_asciidump(start_line_idx, end_line_idx);
        let ascii_view = Paragraph::new(asciidump).block(ascii_block);
        frame.render_widget(ascii_view, body[2]);

        // --- Footer
        let footer_chunks = Layout::horizontal([Constraint::Length(25), Constraint::Length(35)])
            .horizontal_margin(4)
            .flex(Flex::SpaceBetween)
            .split(screen[1]);

        let left_footer = Text::from("Press (h) for help");
        frame.render_widget(left_footer, footer_chunks[0]);

        // --- Help popup
        if self.show_help {
            let popup_rect = centered_rect(area, 30, 40);
            let popup = Popup::default().title("Help").content(HELP_BODY);
            frame.render_widget(popup, popup_rect);
        }
    }

    pub fn handle_event(&mut self, event: Event) -> anyhow::Result<()> {
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match (key.code, key.modifiers) {
                    // Exit
                    (KeyCode::Char('q'), KeyModifiers::NONE) => self.quit = true,
                    // Navigation (vim style)
                    //   Down
                    (KeyCode::Char('j'), KeyModifiers::NONE) => {
                        if self.line_idx < self.size / self.alignment {
                            self.line_idx += 1;
                        }
                    }
                    //   Up
                    (KeyCode::Char('k'), KeyModifiers::NONE) => {
                        self.line_idx = self.line_idx.saturating_sub(1);
                    }
                    //   Mid page up
                    (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                        self.move_page_half_up();
                    }
                    //   Mid page down
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        self.move_page_half_down();
                    }
                    //   Mid page up
                    (KeyCode::PageUp, KeyModifiers::NONE) => {
                        self.move_page_up();
                    }
                    //   Mid page down
                    (KeyCode::PageDown, KeyModifiers::NONE) => {
                        self.move_page_down();
                    }
                    //   go to start
                    (KeyCode::Char('g'), KeyModifiers::NONE) => {
                        self.line_idx = 0;
                    }
                    //   SHIFT + G -- go to end
                    (KeyCode::Char('G'), KeyModifiers::SHIFT) => {
                        self.line_idx = self.size / self.alignment;
                    }
                    // Toggle help dialog
                    (KeyCode::Char('h'), KeyModifiers::NONE) => self.show_help = !self.show_help,
                    _ => {}
                }
            }
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => {
                    if self.line_idx < self.size / self.alignment {
                        self.line_idx += 1;
                    }
                }
                MouseEventKind::ScrollUp => {
                    self.line_idx = self.line_idx.saturating_sub(1);
                }
                _ => {}
            },
            _ => {}
        }

        Ok(())
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            line_idx: 0,
            vertical_margin: 3,
            frame_size: (0, 0),
            show_help: false,
            data: vec![],
            size: 0,
            quit: false,
            alignment: 16,
            filename: String::new(),
        }
    }
}
