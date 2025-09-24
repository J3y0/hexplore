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
            let addr = idx * self.bytes_per_row;
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
            .chunks(self.bytes_per_row)
            .skip(start_line_idx)
            .take(end_line_idx - start_line_idx);

        for chunk in chunks_iter {
            text.push(Line::from(line_format_hex(chunk, self.blocksize)));
        }
        text
    }

    pub fn get_asciidump(&self, start_line_idx: usize, end_line_idx: usize) -> Vec<Line<'static>> {
        let mut text = vec![];
        let chunks_iter = self
            .fileinfo
            .content
            .chunks(self.bytes_per_row)
            .skip(start_line_idx)
            .take(end_line_idx - start_line_idx);

        for chunk in chunks_iter {
            text.push(Line::from(line_format_ascii(chunk)));
        }
        text
    }
}

fn line_format_hex(bytes: &[u8], blocksize: u16) -> String {
    let mut s = String::with_capacity(3 * bytes.len());

    for (i, b) in bytes.iter().enumerate() {
        if i != 0 && i % blocksize as usize == 0 {
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

pub fn count_hexdigits(val: usize) -> usize {
    let mut i = 0;
    while val >> (4 * i) != 0 {
        i += 1;
    }

    i
}

/// Explanation of the calculation
///
/// |-Addr-----------|-Hex-------------------------------|-Ascii---------------|
/// |                |                                   |                     |
/// |                |                                   |                     |
/// | PAD + ADDR_LEN |  3*nb_bytes+nb_bytes/blocksize-1  |       nb_bytes      |
/// | <------------> | <-------------------------------> | <-----------------> |
/// |                |                                   |                     |
/// |                |                                   |                     |
/// |                |                                   |                     |
/// |----------------|-----------------------------------|---------------------|
///
///
/// width = (ADDR_WIDTH) + (3*nb_bytes + nb_bytes/blocksize - 1) + nb_bytes + 3
///
/// ADDR_WIDTH = PAD + ADDR_LEN
/// 3 is the number of bytes allocated to ratatui interface
///
/// Reverse the formula in order to get `nb_bytes`:
///
///  nb_bytes = floor((blocksize)/(4*blocksize+1) * (width - 3 + 1 - PAD - ADDR_LEN))
pub fn get_bytes_per_row(width: u16, addr_width: u16, blocksize: u16) -> usize {
    (blocksize * ((width - addr_width - 3 /* ratatui needed bytes */ + 1) / (4 * blocksize + 1)))
        as usize
}
