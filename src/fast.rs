use crate::ConvChainSample;
use rand::prelude::*;

pub struct ConvChain {
    receptor_size: u32,
    one_over_temperature: f64,
    output_size: i64,
    field: Vec<bool>,
    weights: Vec<f64>,
}

impl ConvChain {
    pub fn new(
        sample: &ConvChainSample,
        output_size: u32,
        receptor_size: u32,
        temperature: f64,
    ) -> Self {
        let weights = Self::initialize_weights(&sample, receptor_size);
        let field = Self::initialize_field(output_size);

        let one_over_temperature = if temperature != 1.0 {
            1.0 / temperature
        } else {
            1.0
        };

        Self {
            receptor_size,
            one_over_temperature,
            output_size: output_size as i64,
            field,
            weights,
        }
    }

    pub fn process(&mut self, iterations: usize) -> &[bool] {
        let mut rng = rand::thread_rng();
        let num_field_elements = self.output_size * self.output_size;
        let num_loops = iterations * num_field_elements as usize;
        for _ in 0..num_loops {
            let r = rng.gen_range(0..num_field_elements);

            let mut q = self.single_iteration(r);

            // Metropolis algorithm: If q is greater than or equal to 1, always accept.
            if q >= 1. {
                self.field[r as usize] = !self.field[r as usize];
                continue;
            }

            // Metropolis algorithm: If q is less than 1, accept with a probability.
            if self.one_over_temperature != 1. {
                q = q.powf(self.one_over_temperature);
            }
            if q > rng.gen() {
                self.field[r as usize] = !self.field[r as usize];
            }
        }

        &self.field
    }

    fn initialize_weights(sample: &ConvChainSample, receptor_size: u32) -> Vec<f64> {
        let mut weights = vec![0.0; 1 << (receptor_size * receptor_size)];
        for y in 0..sample.height {
            for x in 0..sample.width {
                let mut ps = Vec::with_capacity(8);
                let pattern = pattern(|dx, dy| sample[(x + dx, y + dy)], receptor_size);

                ps.push(pattern);
                ps.push(rotate(&ps[0], receptor_size));
                ps.push(rotate(&ps[1], receptor_size));
                ps.push(rotate(&ps[2], receptor_size));
                ps.push(reflect(&ps[0], receptor_size));
                ps.push(reflect(&ps[1], receptor_size));
                ps.push(reflect(&ps[2], receptor_size));
                ps.push(reflect(&ps[3], receptor_size));

                for k in 0..8 {
                    weights[index(&ps[k])] += 1.0;
                }
            }
        }

        for k in 0..weights.len() {
            if weights[k] <= 0. {
                weights[k] = 0.1;
            }
        }

        weights
    }

    fn initialize_field(output_size: u32) -> Vec<bool> {
        let mut rng = rand::thread_rng();
        let mut field = vec![false; output_size as usize * output_size as usize];
        for i in 0..field.len() {
            field[i as usize] = rng.gen();
        }
        field
    }

    fn single_iteration(&mut self, r: i64) -> f64 {
        let out_y = r / self.output_size;
        let out_x = r % self.output_size;

        let sy_min = out_y - self.receptor_size as i64 + 1;
        let sy_max = out_y + self.receptor_size as i64 - 1;
        let sx_min = out_x - self.receptor_size as i64 + 1;
        let sx_max = out_x + self.receptor_size as i64 - 1;

        let mut q: f64 = 1.0;

        for sy in sy_min..=sy_max {
            for sx in sx_min..=sx_max {
                let weight = self.iteration_inner_loop(out_x, out_y, sx, sy);
                q *= weight;
            }
        }

        q
    }

    fn iteration_inner_loop(&mut self, out_x: i64, out_y: i64, sx: i64, sy: i64) -> f64 {
        let mut ind = 0;
        let mut difference: i64 = 0;

        for dy in 0..self.receptor_size {
            let local_y = self.get_local_coordinate(sy, dy);
            let local_row = local_y * self.output_size as i64;
            let is_relevant_row = out_y == local_y;

            for dx in 0..self.receptor_size {
                let power = 1i64 << (dy * self.receptor_size + dx);

                let local_x = self.get_local_coordinate(sx, dx);
                let is_relevant_column = out_x == local_x;

                let index = local_row + local_x;
                let value = self.field[index as usize];
                if value {
                    ind += power;
                }

                if is_relevant_row && is_relevant_column {
                    difference = if value { power } else { -power };
                }
            }
        }

        // Metropolis algorithm: Determine energy difference before and after change.
        self.weights[(ind - difference) as usize] / self.weights[ind as usize]
    }

    fn get_local_coordinate(&self, s: i64, d: u32) -> i64 {
        let mut local = s + d as i64;
        if local < 0 {
            local += self.output_size;
        } else if local >= self.output_size {
            local -= self.output_size;
        }
        local
    }
}

fn pattern<F>(f: F, receptor_size: u32) -> Vec<bool>
where
    F: Fn(u32, u32) -> bool,
{
    let mut result = vec![false; receptor_size as usize * receptor_size as usize];
    for y in 0..receptor_size {
        let row_offset = (y * receptor_size) as usize;
        for x in 0..receptor_size {
            result[row_offset + x as usize] = f(x, y);
        }
    }
    result
}

fn rotate(p: &[bool], receptor_size: u32) -> Vec<bool> {
    pattern(
        |x, y| {
            let index = receptor_size - 1 - y + x * receptor_size;
            p[index as usize]
        },
        receptor_size,
    )
}

fn reflect(p: &[bool], receptor_size: u32) -> Vec<bool> {
    pattern(
        |x, y| {
            let index = receptor_size - 1 - x + y * receptor_size;
            p[index as usize]
        },
        receptor_size,
    )
}

fn index(p: &[bool]) -> usize {
    let mut result = 0;
    let mut power = 1;
    let length = p.len();
    for i in 0..length {
        result += if p[length - 1 - i] { power } else { 0 };
        power *= 2;
    }
    result
}
