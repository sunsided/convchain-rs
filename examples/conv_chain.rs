use convchain::conv_chain::Pattern;
use image::{DynamicImage, GrayImage};
use rand::{random, Rng};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

const RESOURCES_PATH: &str = "resources";

fn main() {
    let doc = read_samples();

    let mut pass = 1;
    for row in doc.samples {
        let file_path = get_file_path(&row.name, "png");
        assert!(file_path.exists());

        let gray = image::open(file_path.clone())
            .expect(format!("failed to open {:?}", file_path).as_str())
            .to_luma8();
        let sample = to_array(&gray);

        for k in 0..row.screenshots {
            println!("> {} {}", row.name, k);
            let result = conv_chain(
                &sample,
                gray.width(),
                gray.height(),
                row.receptor_size,
                row.temperature,
                row.output_size,
                row.iterations,
            );
            let output = to_image(row.output_size, row.output_size, result);
            output.save(format!(
                "{} {} t={} i={} {}.png",
                pass, row.name, row.temperature, row.iterations, k
            ));
        }

        pass += 1;
    }
}

fn conv_chain(
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
    // TODO: create random

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

fn to_array(img: &GrayImage) -> Vec<bool> {
    let vec: Vec<bool> = img.iter().map(|&x| x > 0).collect();
    assert_eq!(vec.len(), (img.width() * img.height()) as usize);
    vec
}

fn to_image(width: u32, height: u32, array: Vec<bool>) -> GrayImage {
    let bytes = array.iter().map(|&x| if x { 255 } else { 0 }).collect();
    GrayImage::from_raw(width, height, bytes).expect("unable to create image")
}

fn read_samples() -> Samples {
    let file_path = get_file_path("samples", "xml");
    let xml = fs::read_to_string(file_path.clone())
        .expect(format!("Could not read {:?}", file_path).as_str());
    quick_xml::de::from_str(xml.as_str()).unwrap()
}

fn get_file_path<S, E>(name: S, extension: E) -> PathBuf
where
    S: AsRef<str>,
    E: AsRef<str>,
{
    [
        RESOURCES_PATH,
        format!("{}.{}", name.as_ref(), extension.as_ref()).as_str(),
    ]
    .iter()
    .collect()
}

#[derive(Debug, Deserialize)]
struct Samples {
    #[serde(rename = "$value")]
    samples: Vec<Sample>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Sample {
    name: String,
    #[serde(default = "default_receptor_size")]
    receptor_size: u32,
    #[serde(default = "default_temperature")]
    temperature: f64,
    #[serde(default = "default_iterations")]
    iterations: usize,
    #[serde(default = "default_screenshot_count")]
    screenshots: usize,
    #[serde(default = "default_output_size")]
    output_size: u32,
}

fn default_receptor_size() -> u32 {
    2
}

fn default_temperature() -> f64 {
    1.
}

fn default_iterations() -> usize {
    2
}

fn default_screenshot_count() -> usize {
    1
}

fn default_output_size() -> u32 {
    32
}
