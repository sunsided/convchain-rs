use rand::prelude::ThreadRng;
use rand::Rng;

pub fn conv_chain(
    sample: &[bool],
    sample_width: u32,
    sample_height: u32,
    receptor_size: u32,
    temperature: f64,
    output_size: u32,
    iterations: usize,
) -> Vec<bool> {
    assert!(receptor_size > 0);

    let num_field_elements = output_size * output_size;
    let mut field = vec![false; num_field_elements as usize];
    let mut weights = vec![0.0; 1 << (receptor_size * receptor_size)];
    let mut rng = rand::thread_rng();

    let one_over_temp = 1.0 / temperature;

    initialize_weights(
        sample,
        sample_width,
        sample_height,
        receptor_size,
        &mut weights,
    );

    initialize_field(&mut field, &mut rng);

    let num_loops = iterations * output_size as usize * output_size as usize;
    for _ in 0..num_loops {
        let r = rng.gen_range(0..num_field_elements);

        let out_y = (r / output_size) as i64;
        let out_x = (r % output_size) as i64;

        let sy_min = out_y - receptor_size as i64 + 1;
        let sy_max = out_y + receptor_size as i64 - 1;
        let sx_min = out_x - receptor_size as i64 + 1;
        let sx_max = out_x + receptor_size as i64 - 1;

        let mut q: f64 = 1.0;
        for sy in sy_min..=sy_max {
            for sx in sx_min..=sx_max {
                let mut ind = 0;
                let mut difference: i32 = 0;
                for dy in 0..receptor_size {
                    let local_y = get_local_coordinate(output_size, sy, dy);
                    let local_row = local_y * output_size as i64;

                    for dx in 0..receptor_size {
                        let local_x = get_local_coordinate(output_size, sx, dx);

                        let index = local_row + local_x;
                        let value = field[index as usize];
                        let power = 1 << (dy * receptor_size + dx);
                        ind += if value { power } else { 0 };

                        if out_y == local_y && out_x == local_x {
                            difference = if value { power } else { -power };
                        }
                    }
                }

                // Metropolis algorithm: Determine energy difference before and after change.
                q *= weights[(ind - difference) as usize] / weights[ind as usize];
            }
        }

        // Metropolis algorithm: If q is greater than or equal to 1, always accept.
        if q >= 1. {
            field[r as usize] = !field[r as usize];
            continue;
        }

        // Metropolis algorithm: If q is less than 1, accept with a probability.
        if temperature != 1. {
            q = q.powf(one_over_temp);
        }
        if q > rng.gen() {
            field[r as usize] = !field[r as usize];
        }
    }

    field
}

fn get_local_coordinate(output_size: u32, s: i64, d: u32) -> i64 {
    let local = s + d as i64;
    if local < 0 {
        local + output_size as i64
    } else if local >= output_size as i64 {
        local - output_size as i64
    } else {
        local
    }
}

fn initialize_field(field: &mut Vec<bool>, rng: &mut ThreadRng) {
    for i in 0..field.len() {
        field[i as usize] = rng.gen();
    }
}

fn initialize_weights(
    sample: &[bool],
    sample_width: u32,
    sample_height: u32,
    receptor_size: u32,
    weights: &mut Vec<f64>,
) {
    for y in 0..sample_height {
        for x in 0..sample_width {
            let mut ps = Vec::with_capacity(8);
            ps.push(pattern(
                |dx, dy| {
                    let index =
                        ((x + dx) % sample_width) + ((y + dy) % sample_height) * sample_width;
                    sample[index as usize]
                },
                receptor_size,
            ));
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
