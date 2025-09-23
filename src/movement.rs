use crate::App;

impl App {
    pub fn update_frame_size(&mut self, new_width: u16, new_height: u16) {
        self.frame_size = (new_width, new_height);
    }

    pub fn move_page_half_up(&mut self) {
        let height = self.frame_size.1 as usize;
        self.line_idx = self.line_idx.saturating_sub(height / 2);
    }

    pub fn move_page_half_down(&mut self) {
        let height = self.frame_size.1 as usize;
        let shift = (height - self.vertical_margin) / 2;
        if self.line_idx + shift <= self.fileinfo.size / self.alignment {
            self.line_idx += shift;
        }
    }

    pub fn move_page_up(&mut self) {
        let height = self.frame_size.1 as usize;
        self.line_idx = self.line_idx.saturating_sub(height);
    }

    pub fn move_page_down(&mut self) {
        let height = self.frame_size.1 as usize - self.vertical_margin;
        if self.line_idx + height <= self.fileinfo.size / self.alignment {
            self.line_idx += height;
        }
    }
}
