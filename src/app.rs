use crate::{
    display,
    file::FileInfo,
    popup::{Popup, centered_rect_length, centered_rect_percent},
};
use ratatui::widgets::{Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{
    Frame,
    layout::{Constraint, Flex, Layout},
    text::Text,
    widgets::{Block, Borders, Paragraph},
};

const ADDR_PANE_PADDING: u16 = 4;
const MIN_BYTES_PER_ROW: u16 = 8;

const WIDTH_ERROR_TEXT: &str = "Terminal is not wide enough..";
const HELP_FOOTER: &str = "Press (h) for help";
const DETAILS_FOOTER: &str = "Press (i) for file details";

const HELP_BODY: &str = r#"
h:        Toggle this help dialog
q:        Quit the application
j:        Move one line down
k:        Move one line up
PageUp:   Move one page up
PageDown: Move one page down
ctrl+u:   Move half page up
ctrl+d:   Move half page down
g:        Go to start
G:        Go to end
i:        Get file details
"#;

pub struct App {
    pub scroll_pos: usize,
    pub vertical_margin: usize,
    pub frame_size: (u16, u16),
    pub show_help: bool,
    pub show_fileinfo: bool,
    pub fileinfo: FileInfo,
    pub quit: bool, // exit state
    pub bytes_per_row: usize,
    pub blocksize: u16, // inherited from cli flags
}

impl App {
    pub fn new(filename: String, blocksize: Option<u16>) -> anyhow::Result<App> {
        let mut app = App {
            fileinfo: FileInfo::new(&filename)?,
            ..App::default()
        };

        // if specified in CLI, change default value
        if let Some(blocksize) = blocksize {
            app.blocksize = blocksize;
        }

        Ok(app)
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();
        let filesize = self.fileinfo.size;
        let nb_digits_addr = display::count_hexdigits(filesize) as u16;

        // --- Compute min width
        let min_width_body = (MIN_BYTES_PER_ROW /* min bytes per line */ * 3 + /* width per byte in hex view */
            MIN_BYTES_PER_ROW/self.blocksize-1 + /* additional space every `blocksize` bytes in hex view */
            MIN_BYTES_PER_ROW +                  /* nb ascii bytes */
            3) +                                 /* ratatui padding bytes (borders,...) */
            ADDR_PANE_PADDING +                  /* address width padding */
            nb_digits_addr                       /* address width */;
        let min_width_footer = HELP_FOOTER.len() + DETAILS_FOOTER.len() + 6 /* padding */;
        let min_width = std::cmp::max(min_width_body, min_width_footer as u16);

        if self.frame_size.0 < min_width {
            // return early if terminal is not wide enough
            let rect = centered_rect_length(area, WIDTH_ERROR_TEXT.len() as u16 + 4, 3);
            let err_paragraph = Paragraph::new(Text::from(WIDTH_ERROR_TEXT))
                .centered()
                .block(Block::default().borders(Borders::ALL));
            frame.render_widget(err_paragraph, rect);
            return;
        }

        // update bytes_per_row according to terminal width
        self.bytes_per_row = display::get_bytes_per_row(
            self.frame_size.0,
            ADDR_PANE_PADDING + nb_digits_addr,
            self.blocksize,
        );

        // --- layout
        let screen =
            Layout::vertical([Constraint::Fill(0), Constraint::Length(1) /* Footer */]).split(area);
        let body = Layout::horizontal([
            Constraint::Length(nb_digits_addr + ADDR_PANE_PADDING),
            Constraint::Fill(1),
            Constraint::Length(self.bytes_per_row as u16 + 1 /* padding */),
            Constraint::Length(1), // Scrollbar
        ])
        .split(screen[0]);

        // --- range of displayed data
        let start_line_idx = self.scroll_pos;
        // sub margin because compared to frame size, only height-margin lines are rendered
        let mut end_line_idx = self.scroll_pos + self.frame_size.1 as usize - self.vertical_margin;
        if end_line_idx * self.bytes_per_row > filesize {
            end_line_idx = filesize.div_ceil(self.bytes_per_row);
        }

        // --- Address view
        let address = self.get_address_to_lines(start_line_idx, end_line_idx);
        let address_block = Block::default()
            .title("Address")
            .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM);

        let address_view = Paragraph::new(address)
            .block(address_block)
            .alignment(ratatui::layout::Alignment::Right);

        frame.render_widget(address_view, body[0]);

        // --- Hex view
        let hexdump = self.get_hexdump(start_line_idx, end_line_idx);
        let hex_block = Block::default().title("Hex").borders(Borders::ALL);
        let hex_view = Paragraph::new(hexdump).block(hex_block);

        frame.render_widget(hex_view, body[1]);

        // --- Ascii view
        let asciidump = self.get_asciidump(start_line_idx, end_line_idx);
        let ascii_block = Block::default()
            .title("Ascii")
            .borders(Borders::RIGHT | Borders::TOP | Borders::BOTTOM);
        let ascii_view = Paragraph::new(asciidump).block(ascii_block);
        frame.render_widget(ascii_view, body[2]);

        // --- Scrollbar
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight).track_symbol(None);
        let content_len = filesize.div_ceil(self.bytes_per_row);
        let mut scrollbar_state = ScrollbarState::default()
            .content_length(content_len)
            .viewport_content_length(4)
            .position(self.scroll_pos);
        frame.render_stateful_widget(scrollbar, body[3], &mut scrollbar_state);

        // --- Footer
        let footer_chunks = Layout::horizontal([
            Constraint::Length(HELP_FOOTER.len() as u16),
            Constraint::Length(DETAILS_FOOTER.len() as u16),
        ])
        .horizontal_margin(2)
        .flex(Flex::SpaceBetween)
        .split(screen[1]);

        let left_footer = Text::from(HELP_FOOTER);
        let right_footer = Text::from(DETAILS_FOOTER);
        frame.render_widget(left_footer, footer_chunks[0]);
        frame.render_widget(right_footer, footer_chunks[1]);

        // --- Help popup
        if self.show_help {
            let popup_rect = centered_rect_percent(area, 30, 40);
            let popup = Popup::default().title("Help").content(HELP_BODY);
            frame.render_widget(popup, popup_rect);
        }

        // --- Fileinfo popup
        if self.show_fileinfo {
            let popup_rect = centered_rect_percent(area, 50, 25);
            let popup = Popup::default()
                .title("File details")
                .content(self.fileinfo.to_text());
            frame.render_widget(popup, popup_rect);
        }
    }
}

impl Default for App {
    fn default() -> Self {
        App {
            scroll_pos: 0,
            vertical_margin: 3,
            frame_size: (0, 0),
            show_help: false,
            show_fileinfo: false,
            fileinfo: FileInfo::default(),
            quit: false,
            bytes_per_row: 16,
            blocksize: 8,
        }
    }
}
