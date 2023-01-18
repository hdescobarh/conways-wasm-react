use crate::abort;
use std::mem::take;
pub struct Observer {
    pub map_row: Vec<usize>,
    pub map_col: Vec<usize>,
}

impl Observer {
    pub fn new(max_row: &usize, max_col: &usize) -> Self {
        Self {
            map_row: vec![*max_row, 0, 1],
            map_col: vec![*max_col, 0, 1],
        }
    }

    pub fn get_row(&self, index: usize) -> &usize {
        self.map_row.get(index).unwrap_or_else(|| abort())
    }

    pub fn get_col(&self, index: usize) -> &usize {
        self.map_col.get(index).unwrap_or_else(|| abort())
    }

    fn set_row(&mut self, index: usize, value: usize) {
        *self.map_row.get_mut(index).unwrap_or_else(|| abort()) = value;
    }

    fn set_col(&mut self, index: usize, value: usize) {
        *self.map_col.get_mut(index).unwrap_or_else(|| abort()) = value;
    }

    fn reset_col(&mut self, max_col: &usize) {
        self.set_col(0, *max_col);
        self.set_col(1, 0);
        self.set_col(2, 1);
    }

    fn reset_row(&mut self, max_row: &usize) {
        self.set_row(0, *max_row);
        self.set_row(1, 0);
        self.set_row(2, 1);
    }

    fn shift_row(&mut self, value: usize) {
        *self.map_row.get_mut(0).unwrap_or_else(|| abort()) =
            take(self.map_row.get_mut(1).unwrap_or_else(|| abort()));
        *self.map_row.get_mut(1).unwrap_or_else(|| abort()) =
            take(self.map_row.get_mut(2).unwrap_or_else(|| abort()));
        *self.map_row.get_mut(2).unwrap_or_else(|| abort()) = value;
    }
    fn shift_col(&mut self, value: usize) {
        *self.map_col.get_mut(0).unwrap_or_else(|| abort()) =
            take(self.map_col.get_mut(1).unwrap_or_else(|| abort()));
        *self.map_col.get_mut(1).unwrap_or_else(|| abort()) =
            take(self.map_col.get_mut(2).unwrap_or_else(|| abort()));
        *self.map_col.get_mut(2).unwrap_or_else(|| abort()) = value;
    }

    pub fn refresh(&mut self, max_col: &usize, max_row: &usize) {
        self.reset_col(max_col);
        self.reset_row(max_row);
    }

    fn forward_row(&mut self, height: &usize) {
        let i_shift = self
            .get_row(2)
            .checked_add_signed(1)
            .unwrap_or_else(|| abort());

        if i_shift < *height {
            self.shift_row(i_shift);
        } else {
            self.shift_row(0);
        }
    }

    // universe will use it width*height times. if false will sum A + B + C - O_b. If true, resets.
    pub fn forward_view(&mut self, width: &usize, height: &usize) -> bool {
        let j_shift = self
            .get_col(2)
            .checked_add_signed(1)
            .unwrap_or_else(|| abort());
        if j_shift == 1 {
            self.shift_col(j_shift);
            self.forward_row(height);
            return true;
        } else if j_shift < *width {
            self.shift_col(j_shift);
            return false;
        } else {
            self.shift_col(0);
            return false;
        }
    }
}
