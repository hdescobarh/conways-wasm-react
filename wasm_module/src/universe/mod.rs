pub mod observer;
use crate::abort;
use std::mem::{swap, take};
pub struct Universe {
    height: usize,
    width: usize,
    size: usize,
    // An 1D array of fixed capacity to reduce allocating
    space: Vec<bool>,
    next_space: Vec<bool>,
    age: usize,
    observer: observer::Observer,
    max_col: usize,
    max_row: usize,
    col_buffer: Vec<usize>,
}

impl Universe {
    pub fn new(height: usize, width: usize, init_space: Vec<bool>) -> Self {
        // minimum vector's size is 9 and cannot exced usize::MAX
        let size = {
            if (height > 3) & (width > 3) {
                height.checked_mul(width).unwrap_or_else(|| abort())
            } else {
                abort()
            }
        };
        let space: Vec<bool>;
        let next_space: Vec<bool>;
        if init_space.len() != size {
            abort()
        } else {
            next_space = init_space.clone();
            space = init_space;
        }
        // the sum cannot be bigger than usize::MAX
        height.checked_add(width).unwrap_or_else(|| abort());
        let max_col = width.checked_add_signed(-1).unwrap_or_else(|| abort());
        let max_row = height.checked_add_signed(-1).unwrap_or_else(|| abort());
        Self {
            height,
            width,
            size,
            space,
            next_space,
            age: 0usize,
            observer: observer::Observer::new(&max_row, &max_col),
            max_col,
            max_row,
            col_buffer: vec![0; 3],
        }
    }

    pub fn get_space_raw(&self) -> &Vec<bool> {
        &self.space
    }

    pub fn get_single_value_by_raw_index(&self, index: usize) -> &bool {
        &self.space.get(index).unwrap_or_else(|| abort())
    }

    pub fn get_age(&self) -> &usize {
        &self.age
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

    // tested
    fn map_col_sum(&self, coordinate_j: &usize) -> usize {
        let mut counter: usize = 0;
        for coordinate_i in &self.observer.map_row {
            if *self.read_at_location(coordinate_i, coordinate_j) == true {
                counter = counter.checked_add(1).unwrap_or_else(|| abort());
            }
        }
        counter
    }

    // tested
    fn init_buffer(&mut self) {
        *self.col_buffer.get_mut(0).unwrap_or_else(|| abort()) = self.map_col_sum(&self.max_col);
        *self.col_buffer.get_mut(1).unwrap_or_else(|| abort()) = self.map_col_sum(&0);
        *self.col_buffer.get_mut(2).unwrap_or_else(|| abort()) = self.map_col_sum(&1);
    }

    // tested
    fn shift_buffer(&mut self, coordinate_j: &usize, jump: bool) {
        if jump == true {
            self.init_buffer()
        } else {
            *self.col_buffer.get_mut(0).unwrap_or_else(|| abort()) =
                take(self.col_buffer.get_mut(1).unwrap_or_else(|| abort()));
            *self.col_buffer.get_mut(1).unwrap_or_else(|| abort()) =
                take(self.col_buffer.get_mut(2).unwrap_or_else(|| abort()));
            *self.col_buffer.get_mut(2).unwrap_or_else(|| abort()) = self.map_col_sum(coordinate_j);
        }
    }

    //tested
    fn cell_step_check(&mut self, raw_index: usize) -> &bool {
        // get the current cell cell coordinates and check its state
        let current_i = self.observer.get_row(1);
        let current_j = self.observer.get_col(1);
        let current_cell_state = self.read_at_location(current_i, current_j);
        // counts alive in the nine cells block
        let mut sum = 0usize;
        for val in self.col_buffer.iter() {
            sum = sum.checked_add(*val).unwrap_or_else(|| abort());
        }
        // if the current cell is alive, remove the aditional count
        if *current_cell_state == true {
            sum = sum.checked_add_signed(-1).unwrap_or_else(|| abort());
        } else {
        }
        // define the cell fate based on the rules, then stores the value in the next_space
        let new_state = self.cell_fate(current_cell_state, &sum);
        // asing new state to next space by vector index (not i,j coordinates)
        *self
            .next_space
            .get_mut(raw_index)
            .unwrap_or_else(|| abort()) = new_state;
        // shifts the observer to next position, then shifts the buffer accordingly
        let jump = self.observer.forward_view(&self.width, &self.height);
        let coordinate_j = *self.observer.get_col(2);
        self.shift_buffer(&coordinate_j, jump);

        // ///////////// remember change the type to ()
        self.next_space.get(raw_index).unwrap_or_else(|| abort())
    }

    pub fn time_step(&mut self) {
        self.observer.refresh(&self.max_col, &self.max_row);
        self.init_buffer();
        for raw_index in 0..self.size {
            self.cell_step_check(raw_index);
        }
        // update number of generations
        self.age = self.age.saturating_add(1);
        // update the space
        swap(&mut self.space, &mut self.next_space);
    }
}
