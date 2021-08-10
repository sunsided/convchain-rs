use rand::{random, Rng};

pub fn conv_chain(
    sample: &Vec<bool>,
    sample_width: u32,
    sample_height: u32,
    receptor_size: u32,
    temperature: f64,
    output_size: u32,
    iterations: usize,
) -> Vec<bool> {
    let mut field = vec![false; (output_size * output_size) as usize];
    let mut weights = vec![0.; 1 << (receptor_size * receptor_size)];

    for y in 0..sample_height {
        for x in 0..sample_width {
            let mut p = Vec::with_capacity(8);
            p.push(Pattern::new_from_pattern(
                sample_width,
                sample_height,
                sample,
                x as i64,
                y as i64,
                receptor_size,
            ));
            p.push(p[0].rotated());
            p.push(p[1].rotated());
            p.push(p[2].rotated());
            p.push(p[0].reflected());
            p.push(p[1].reflected());
            p.push(p[2].reflected());
            p.push(p[3].reflected());

            for k in 0..8 {
                let index = p[k].index();
                weights[index] += 1.0;
            }
        }
    }

    for k in 0..weights.len() {
        if weights[k] <= 0. {
            weights[k] = 0.1;
        }
    }

    for y in 0..output_size {
        for x in 0..output_size {
            let index = y * output_size + x;
            field[index as usize] = random();
        }
    }

    let mut rng = rand::thread_rng();
    for _ in 0..(iterations * output_size as usize * output_size as usize) {
        let x = rng.gen_range(0..output_size);
        let y = rng.gen_range(0..output_size);
        metropolis(
            x,
            y,
            output_size,
            temperature,
            receptor_size,
            &mut field,
            output_size,
            output_size,
            &weights,
        );
    }

    field
}

fn metropolis(
    i: u32,
    j: u32,
    output_width: u32,
    temperature: f64,
    receptor_size: u32,
    field: &mut Vec<bool>,
    field_width: u32,
    field_height: u32,
    weights: &Vec<f64>,
) {
    let index = (j * output_width + i) as usize;

    let p = energy_exp(
        i,
        j,
        receptor_size,
        field,
        field_width,
        field_height,
        weights,
    );
    field[index] = !field[index];
    let q = energy_exp(
        i,
        j,
        receptor_size,
        field,
        field_width,
        field_height,
        weights,
    );

    let q_over_p: f64 = q / p;
    let one_over_temp: f64 = 1. / temperature;
    if q_over_p.powf(one_over_temp) < random() {
        field[index] = !field[index];
    }
}

fn energy_exp(
    i: u32,
    j: u32,
    receptor_size: u32,
    field: &Vec<bool>,
    field_width: u32,
    field_height: u32,
    weights: &Vec<f64>,
) -> f64 {
    let mut value = 1.;

    let y_min = (j as i64) - (receptor_size as i64) + 1;
    let y_max = (j as i64) + (receptor_size as i64) - 1;

    let x_min = (i as i64) - (receptor_size as i64) + 1;
    let x_max = (i as i64) + (receptor_size as i64) - 1;

    debug_assert!(y_min <= y_max);
    debug_assert!(x_min <= x_max);

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let pattern =
                Pattern::new_from_pattern(field_width, field_height, field, x, y, receptor_size);
            let index = pattern.index();
            let weight = weights[index];
            value *= weight;
        }
    }
    value
}

struct Pattern {
    pub size: u32,
    data: Vec<bool>,
}

impl Pattern {
    pub fn new_from_function<F>(size: u32, f: F) -> Self
    where
        F: Fn(u32, u32) -> bool,
    {
        let data = vec![false; (size * size) as usize];
        let mut value = Self { size, data };
        value.set(f);
        value
    }

    pub fn new_from_pattern(
        field_width: u32,
        field_height: u32,
        field: &[bool],
        x: i64,
        y: i64,
        size: u32,
    ) -> Self {
        let value = Self::new_from_function(size, |i, j| {
            let fx = (x + i as i64 + field_width as i64) % (field_width as i64);
            let fy = (y + j as i64 + field_height as i64) % (field_height as i64);
            debug_assert!((fx >= 0) && (fy >= 0));
            let index = fy * (field_width as i64) + fx;
            field[index as usize]
        });
        value
    }

    pub fn rotated(&self) -> Self {
        Self::new_from_function(self.size, |x, y| {
            let index = self.array_index(self.size - 1 - y, x);
            self.data[index]
        })
    }

    pub fn reflected(&self) -> Self {
        Self::new_from_function(self.size, |x, y| {
            let index = self.array_index(self.size - 1 - x, y);
            self.data[index]
        })
    }

    pub fn index(&self) -> usize {
        let mut result = 0;
        let size = self.size;
        for y in 0..size {
            for x in 0..size {
                let index = self.array_index(x, y);
                result += if self.data[index] { 1 << index } else { 0 };
            }
        }
        result
    }

    fn set<F>(&mut self, f: F)
    where
        F: Fn(u32, u32) -> bool,
    {
        let size = self.size;
        for y in 0..size {
            for x in 0..size {
                let index = self.array_index(x, y);
                self.data[index] = f(x, y);
            }
        }
    }

    #[inline]
    fn array_index(&self, x: u32, y: u32) -> usize {
        (y * self.size + x) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod pattern {
        use super::*;

        #[test]
        fn new_from_function_works() {
            let pattern = Pattern::new_from_function(4, |x, y| (x + y) % 4 == 0);
            assert_eq!(
                pattern.data,
                [
                    true, false, false, false, //
                    false, false, false, true, //
                    false, false, true, false, //
                    false, true, false, false, //
                ]
            );
        }

        #[test]
        fn rotated_works() {
            let pattern = Pattern::new_from_function(4, |x, y| (x + y) % 4 == 0).rotated();
            assert_eq!(
                pattern.data,
                [
                    false, true, false, false, //
                    false, false, true, false, //
                    false, false, false, true, //
                    true, false, false, false, //
                ]
            );
        }

        #[test]
        fn reflected_works() {
            let pattern = Pattern::new_from_function(4, |x, y| (x + y) % 4 == 0).reflected();
            assert_eq!(
                pattern.data,
                [
                    false, false, false, true, //
                    true, false, false, false, //
                    false, true, false, false, //
                    false, false, true, false, //
                ]
            );
        }
    }
}
