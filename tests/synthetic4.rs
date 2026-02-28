//

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        Matrix {
            data: vec![0.0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        if row < self.rows && col < self.cols {
            self.data[row * self.cols + col] = val;
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row * self.cols + col]
    }

    pub fn row_sum(&self, row: usize) -> f64 {
        let start = row * self.cols;
        self.data[start..start + self.cols]
            .iter()
            .sum()
    }

    pub fn col_max(&self, col: usize) -> f64 {
        (0..self.rows)
            .map(|r| self.data[r * self.cols + col])
            .fold(f64::NEG_INFINITY, f64::max)
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn to_map(&self) -> HashMap<(usize, usize), f64> {
        let mut map = HashMap::new();
        for r in 0..self.rows {
            for c in 0..self.cols {
                map.insert((r, c), self.get(r, c));
            }
        }
        map
    }
}
