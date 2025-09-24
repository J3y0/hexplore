use crate::App;

impl App {
    pub fn update_frame_size(&mut self, new_width: u16, new_height: u16) {
        self.frame_size = (new_width, new_height);
    }

    pub fn move_page_half_up(&mut self) {
        let height = self.frame_size.1 as usize - self.vertical_margin;
        self.scroll_pos = self.scroll_pos.saturating_sub(height / 2);
    }

    pub fn move_page_half_down(&mut self) {
        let height = self.frame_size.1 as usize;
        let shift = (height - self.vertical_margin) / 2;
        if (self.scroll_pos + shift) * self.bytes_per_row < self.fileinfo.size {
            self.scroll_pos += shift;
        }
    }

    pub fn move_page_up(&mut self) {
        let height = self.frame_size.1 as usize - self.vertical_margin;
        self.scroll_pos = self.scroll_pos.saturating_sub(height);
    }

    pub fn move_page_down(&mut self) {
        let height = self.frame_size.1 as usize - self.vertical_margin;
        if (self.scroll_pos + height) * self.bytes_per_row < self.fileinfo.size {
            self.scroll_pos += height;
        }
    }
}
