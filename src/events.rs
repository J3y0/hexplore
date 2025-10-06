use crate::app::App;
use ratatui::crossterm::event::{Event, KeyCode, KeyEventKind, KeyModifiers, MouseEventKind};

impl App {
    pub fn handle_event(&mut self, event: Event) {
        let filesize = self.fileinfo.size;
        match event {
            Event::Key(key) if key.kind == KeyEventKind::Press => {
                match (key.code, key.modifiers) {
                    // Exit
                    (KeyCode::Char('q'), KeyModifiers::NONE) => self.quit = true,
                    // Navigation (vim style)
                    //   Down
                    (KeyCode::Char('j'), KeyModifiers::NONE) => {
                        if (self.scroll_pos + 1) * self.bytes_per_row < filesize {
                            self.scroll_pos += 1;
                        }
                    }
                    //   Up
                    (KeyCode::Char('k'), KeyModifiers::NONE) => {
                        self.scroll_pos = self.scroll_pos.saturating_sub(1);
                    }
                    //   Mid-page up
                    (KeyCode::Char('u'), KeyModifiers::CONTROL) => {
                        self.move_page_half_up();
                    }
                    //   Mid-page down
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        self.move_page_half_down();
                    }
                    //   Mid-page up
                    (KeyCode::PageUp, KeyModifiers::NONE) => {
                        self.move_page_up();
                    }
                    //   Mid-page down
                    (KeyCode::PageDown, KeyModifiers::NONE) => {
                        self.move_page_down();
                    }
                    //   go to start
                    (KeyCode::Char('g'), KeyModifiers::NONE) => {
                        self.scroll_pos = 0;
                    }
                    //   SHIFT + G -- go to end
                    (KeyCode::Char('G'), KeyModifiers::SHIFT) => {
                        self.scroll_pos = (filesize - 1) / self.bytes_per_row;
                    }
                    // Toggle help dialog
                    (KeyCode::Char('h'), KeyModifiers::NONE) => self.show_help = !self.show_help,
                    // Toggle file details dialog
                    (KeyCode::Char('i'), KeyModifiers::NONE) => {
                        self.show_fileinfo = !self.show_fileinfo;
                    }
                    _ => {}
                }
            }
            Event::Mouse(mouse) => match mouse.kind {
                MouseEventKind::ScrollDown => {
                    if (self.scroll_pos + 1) * self.bytes_per_row < filesize {
                        self.scroll_pos += 1;
                    }
                }
                MouseEventKind::ScrollUp => {
                    self.scroll_pos = self.scroll_pos.saturating_sub(1);
                }
                _ => {}
            },
            Event::Resize(width, height) => self.update_frame_size(width, height),
            _ => {}
        }
    }
}
