#![warn(clippy::all, rust_2018_idioms)]

mod app;

pub use app::TemplateApp;

use egui::Color32;
use log::warn;

type Color = [i64; 3];

fn _color32_to_color(c32: Color32) -> Color {
    [c32.r() as i64, c32.g() as i64, c32.b() as i64]
}

fn color_to_str(c: Color) -> String {
    format! {"R{} G{} B{}", c[0], c[1], c[2]}
}

fn euclidean_distance(vec_a: &Color, vec_b: &Color) -> f64 {
    let mut sum: i64 = 0;

    for (a, b) in vec_a.into_iter().zip(vec_b.into_iter()) {
        sum += (a - b).pow(2)
    }

    (sum as f64).sqrt()
}

#[derive(Debug, Clone)]
struct StandardColor {
    name: String,
    color: Color,
}

fn build_standard_color(name: String, color: Color) -> StandardColor {
    StandardColor {
        name,
        color,
    }
}

/*
#[deprecated]
#[derive(serde::Deserialize, Debug)]
struct _DeserialisedStandardColor {
    name: String,
    r: i64,
    g: i64,
    b: i64,
}

#[deprecated]
fn _to_standard_color(dsc: _DeserialisedStandardColor) -> StandardColor {
    StandardColor { name: String::from(&dsc.name), color: [dsc.r, dsc.g, dsc.b] }
}

#[deprecated]
fn _get_standard_colors() -> Vec<StandardColor> {
    let mut res: Vec<StandardColor> = Vec::with_capacity(865);
    let rdr = csv::Reader::from_path("C:/Users/Nestor/IdeaProjects/nearest_color/colors.csv");

    for result in rdr.unwrap().deserialize() {
        let std_color: _DeserialisedStandardColor = result.unwrap();
        res.push(_to_standard_color(std_color))
    }
    res
}
*/

// hallelujah https://rust-lang-nursery.github.io/rust-cookbook/encoding/csv.html
fn get_standard_colors() -> Vec<StandardColor> {
    let mut res: Vec<StandardColor> = Vec::with_capacity(865);
    let sth = include_str!("C:/Users/Nestor/IdeaProjects/nearest_color/colors.csv");
    let mut rdr = csv::Reader::from_reader(sth.as_bytes());

    for record in rdr.records() {
        let record = record.unwrap();
        let color = [
            record[1].parse::<i64>().unwrap(),
            record[2].parse::<i64>().unwrap(),
            record[3].parse::<i64>().unwrap()
        ];
        res.push(build_standard_color(String::from(&record[0]), color));
    }
    res
}

fn nearest_color_single(standard_colors: &Vec<StandardColor>, color: Color) -> StandardColor {
    standard_colors.into_iter()
        .map(|std| (std.clone(), euclidean_distance(&std.color, &color)))
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .unwrap()
        .0
}
