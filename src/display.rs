use ratatui::text::Line;

use crate::app::App;

impl App {
    pub fn get_address_to_lines(
        &self,
        start_line_idx: usize,
        end_line_idx: usize,
    ) -> Vec<Line<'static>> {
        let cap = count_hexdigits(self.fileinfo.size);
        let mut text = vec![];

        for idx in start_line_idx..end_line_idx {
            let addr = idx * self.alignment;
            let hex = format!("{addr:x}");
            text.push(Line::from("0".repeat(cap - hex.len()) + &hex));
        }

        text
    }

    pub fn get_hexdump(&self, start_line_idx: usize, end_line_idx: usize) -> Vec<Line<'static>> {
        let mut text = vec![];
        let chunks_iter = self
            .fileinfo
            .content
            .chunks(self.alignment)
            .skip(start_line_idx)
            .take(end_line_idx - start_line_idx);

        for chunk in chunks_iter {
            text.push(Line::from(line_format_hex(chunk)));
        }
        text
    }

    pub fn get_asciidump(&self, start_line_idx: usize, end_line_idx: usize) -> Vec<Line<'static>> {
        let mut text = vec![];
        let chunks_iter = self
            .fileinfo
            .content
            .chunks(self.alignment)
            .skip(start_line_idx)
            .take(end_line_idx - start_line_idx);

        for chunk in chunks_iter {
            text.push(Line::from(line_format_ascii(chunk)));
        }
        text
    }
}

fn line_format_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(3 * bytes.len());

    for (i, b) in bytes.iter().enumerate() {
        if i != 0 && i % 4 == 0 {
            s.push(' ');
        }
        s.push_str(format!("{b:02X} ").as_str());
    }
    // Pop last space
    s.pop();

    s
}

fn line_format_ascii(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len());

    for b in bytes.iter() {
        if b.is_ascii_graphic() {
            s.push((*b).into());
        } else {
            s.push('.');
        }
    }

    s
}

fn count_hexdigits(val: usize) -> usize {
    let mut i = 0;
    while val >> (4 * i) != 0 {
        i += 1;
    }

    i
}
