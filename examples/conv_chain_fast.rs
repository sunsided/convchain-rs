use convchain::conv_chain;
use image::GrayImage;
use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

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
            output
                .save(format!(
                    "{} {} t={} i={} {}.png",
                    pass, row.name, row.temperature, row.iterations, k
                ))
                .expect("unable to save output image");
        }

        pass += 1;
    }
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
