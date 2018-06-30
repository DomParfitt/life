use rand::prelude::*;

pub struct Model {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<bool>>,
}

impl Model {
    pub fn new(width: usize, height: usize) -> Self {
        let mut cells = Vec::with_capacity(height);
        for _ in 0..height {
            let row = vec![false; width];
            cells.push(row);
        }

        Model {
            width: width,
            height: height,
            cells: cells,
        }
    }

    pub fn new_random(width: usize, height: usize) -> Self {
        let mut model = Model::new(width, height);
        model.populate_randomly();
        model
    }

    pub fn populate(&mut self, cells: Vec<Vec<bool>>) {
        self.cells = cells;
    }

    pub fn populate_randomly(&mut self) {
        let mut cells = Vec::with_capacity(self.height);
        for _ in 0..self.height {
            let mut row = Vec::with_capacity(self.width);
            for _ in 0..self.width {
                let cell: bool = random();
                row.push(cell);
            }
            cells.push(row);
        }
        self.cells = cells;
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        self.cells[y][x]
    }

    fn flip_cell(&mut self, x: usize, y: usize) {
        self.cells[y][x] = !self.cells[y][x];
    }

    pub fn run_step(&mut self) {
        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                let live = self.count_live_neighbours(x, y);
                
                if self.is_alive(x, y) {
                    if live < 2 || live > 3{
                        self.flip_cell(x, y);
                    } 
                } else {
                    if live == 3 {
                        self.flip_cell(x, y);
                    }
                }
            }
        }
    }

    fn count_live_neighbours(&self, x: usize, y: usize) -> u8 {
        let mut count: u8 = 0;
        for (x, y) in self.get_neighbours(x, y).iter() {
            if self.is_alive(*x, *y) {
                count += 1;
            }
        }
        count
    }

    // pub fn count_dead_neighbours(&self, x: usize, y: usize) -> u8 {
    //     let total_count = self.get_neighbours(x, y).len();
    //     let live_count = self.count_live_neighbours(x, y);
    //     total_count as u8 - live_count
    // }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbours = Vec::new();

        if x > 0 {
            neighbours.push((x - 1, y));
        }

        if x < self.width {
            neighbours.push((x + 1, y));
        }

        if y > 0 {
            neighbours.push((x, y - 1))
        }

        if y < self.height {
            neighbours.push((x, y + 1));
        }

        if x > 0 && y > 0 {
            neighbours.push((x - 1, y - 1));
        }

        if x < self.width && y < self.height {
            neighbours.push((x + 1, y + 1));
        }

        if x > 0 && y < self.height {
            neighbours.push((x - 1, y + 1));
        }

        if y > 0 && x < self.width {
            neighbours.push((x + 1, y - 1));
        }

        neighbours
    }
}
