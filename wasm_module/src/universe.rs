use crate::abort;
use std::mem::{swap, take};
pub struct Universe {
    height: usize,
    width: usize,
    semiperimeter: usize,
    size: usize,
    // An 1D array of fixed capacity to reduce allocating
    space: Vec<bool>,
    next_space: Vec<bool>,
    age: usize,
    observer: Observer,
    max_col: usize,
    max_row: usize,
    col_buffer: Vec<usize>,
}

impl Universe {
    pub fn new(height: usize, width: usize) -> Self {
        // minimum vector's size is 1 and cannot exced usize::MAX
        let size = {
            if (height > 3) & (width > 3) {
                height.checked_mul(width).unwrap_or_else(|| abort())
            } else {
                abort()
            }
        };
        // the sum cannot be bigger than usize::MAX
        let semiperimeter = height.checked_add(width).unwrap_or_else(|| abort());
        let max_col = width.checked_add_signed(-1).unwrap_or_else(|| abort());
        let max_row = height.checked_add_signed(-1).unwrap_or_else(|| abort());
        Self {
            height,
            width,
            semiperimeter,
            size,
            space: vec![false; size],
            next_space: vec![false; size],
            age: 0usize,
            observer: Observer::new(&max_row, &max_col),
            max_col,
            max_row,
            col_buffer: vec![0; 3],
        }
    }

    pub fn read_at_location(&self, coordinate_i: &usize, coordinate_j: &usize) -> &bool {
        // if matrix_i, matrix_j in valid range;
        // then, (matrix_i * width) + matrix_j <= (width * height) <= usize::MAX. Maybe can use uncheck operations
        let index = coordinate_i
            .checked_mul(self.width)
            .unwrap_or_else(|| abort())
            .checked_add(*coordinate_j)
            .unwrap_or_else(|| abort());
        self.space.get(index).unwrap_or_else(|| abort())
    }

    fn cell_fate(&self, current_cell_state: &bool, alive_neighbour: &usize) -> bool {
        if *current_cell_state == true {
            if *alive_neighbour < 2 {
                false
            } else if *alive_neighbour > 3 {
                false
            } else {
                true
            }
        } else {
            if *alive_neighbour == 3 {
                true
            } else {
                false
            }
        }
    }

    fn map_col_sum(&self, coordinate_j: &usize) -> usize {
        let mut counter: usize = 0;
        for coordinate_i in &self.observer.map_row {
            if *self.read_at_location(coordinate_i, coordinate_j) == true {
                counter = counter.checked_add(1).unwrap_or_else(|| abort());
            }
        }
        counter
    }

    fn init_buffer(&mut self) {
        for coordinate_j in 0..3 {
            *self
                .col_buffer
                .get_mut(coordinate_j)
                .unwrap_or_else(|| abort()) = self.map_col_sum(&coordinate_j);
        }
    }

    fn shift_buffer(&mut self, coordinate_j: &usize, jump: bool) {
        if jump == true {
            self.init_buffer()
        } else {
            *self.col_buffer.get_mut(0).unwrap_or_else(|| abort()) =
                take(self.col_buffer.get_mut(1).unwrap_or_else(|| abort()));
            *self.col_buffer.get_mut(1).unwrap_or_else(|| abort()) =
                take(self.col_buffer.get_mut(2).unwrap_or_else(|| abort()));
            *self.col_buffer.get_mut(3).unwrap_or_else(|| abort()) = self.map_col_sum(coordinate_j);
        }
    }

    fn time_step(&mut self) {
        self.observer.refresh(&self.max_col, &self.max_row);
        self.init_buffer();
        for cell in 0..self.size {
            // for each cell
            // get the current cell cell coordinates and check its state
            let current_i = self.observer.get_row(1);
            let current_j = self.observer.get_col(1);
            let current_cell_state = self.read_at_location(current_i, current_j);
            // initializes the counter for the alive neighbours
            let mut sum = 0usize;
            // sum the value in the buffer; then, if current is true rest -1 to the counter
            for val in self.col_buffer.iter() {
                sum = sum.checked_add(*val).unwrap_or_else(|| abort());
            }
            if *current_cell_state == true {
                sum = sum.checked_add_signed(-1).unwrap_or_else(|| abort());
            } else {
            }
            // define the fate based on the rules, then stores the value in the next_space
            let new_state = self.cell_fate(current_cell_state, &sum);
            // add to next space
            *self.next_space.get_mut(cell).unwrap_or_else(|| abort()) = new_state;
            // forward the observer to next cell and read if there was a row jump
            let jump = self.observer.forward_view(&self.width, &self.height);
            // Now updates the buffer based on the new state of the observer
            let coordinate_j = *self.observer.get_col(2);
            self.shift_buffer(&coordinate_j, jump);
        }
        // update number of generations
        self.age = self.age.saturating_add(1);
        // update the space
        swap(&mut self.space, &mut self.next_space);
    }
}

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
