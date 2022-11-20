/* A tristimulus color space; uses lch for perceptual uniformity, but normalized to 0 - 1 */

use std::{path::Path, fs, io};

use color_space::{Lch, Rgb};
use egui::Color32;

use super::vptree::{VPTree, MetricPoint};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub luminance: f64,
    pub chroma: f64,
    pub hue: f64
}

impl Color {
    pub fn from_hex(hex: &str) -> Self {
        let Lch{l, c, h} = 
            color_space::Rgb::from_hex(u32::from_str_radix(&hex[1..], 16).unwrap()).into();

        Color {
            hue: h/360.0,
            luminance: l/100.0,
            chroma: c/100.0,
        }
    }

    pub fn to_hex(self) -> String {
        let Color { luminance, chroma, hue } = self;
        let Rgb{r, g, b} =
            color_space::Lch::new(luminance * 100., chroma * 100., hue * 360.).into();

        let r = r.round() as u32;
        let g = g.round() as u32;
        let b = b.round() as u32;
        let hex = r * 256 * 256 + g * 256 + b; 

        let mut s = format!("{:06x}", hex);
        s.insert(0, '#');
        s
    }

    pub fn to_color32(self) -> Color32 {
        let Color { luminance, chroma, hue } = self;
        let Rgb{r, g, b} =
            color_space::Lch::new(luminance * 100., chroma * 100., hue * 360.).into();

        let r = r.round() as u8;
        let g = g.round() as u8;
        let b = b.round() as u8;

        Color32::from_rgb(r, g, b)
    }

    pub fn rotate(self, amount: f64) -> Color {
        Color{ luminance: self.luminance, chroma: self.chroma, hue: (self.hue + amount + 1.0) % 1.0 }
    }


    /* Return a very light or very dark color to be used to overlay on top of the current
     * background color, usually for dots or text */
    pub fn borw(self) -> Color32 {
        if self.luminance < 0.5 {
            Color32::WHITE
        } else {
            Color32::BLACK
        }
    }
}

#[test]
fn test_inverse_color() {
    let hex = "#0e5d83";
    assert_eq!(hex, Color::from_hex(hex).to_hex());
}


/* Bad implementation for color distance, ideally one should convert to Luv and then compute
 * distances */
impl MetricPoint for Color {
    type Dist = f64;

    fn dist(from: &Self, to: &Self) -> Self::Dist {
        let Color{luminance: l1, chroma: c1, hue: h1} = from;
        let Color{luminance: l2, chroma: c2, hue: h2} = to;
        
        let hdist = (h1 - h2).abs().min(1. - (h1 - h2).abs());
        hdist + (c2 - c1).abs() + (l2 - l1).abs()
    }
}

pub fn luminance_lerp(start: f64, end: f64) -> Lerp {
    Lerp{
        lerp: Box::new(move |color: Color, t| Color{luminance: start + (end - start) * t, ..color}),
        position: Box::new(move |color: Color| (color.luminance - start)/(end - start))
    }
}

pub fn chroma_lerp(start: f64, end: f64) -> Lerp {
    Lerp{
        lerp: Box::new(move |color: Color, t| Color{chroma: start + (end - start) * t, ..color}),
        position: Box::new(move |color: Color| (color.chroma - start)/(end - start))
    }
}

pub fn hue_lerp(start: f64, end: f64) -> Lerp {
    Lerp{
        lerp: Box::new(move |color: Color, t| Color{hue: start + (end - start) * t, ..color}),
        position: Box::new(move |color: Color| (color.hue - start)/(end - start))
    }
}


// list of shades, and position of current shade in that list
pub fn shades(base_color: Color, max_shades: usize, lerp: &Lerp) -> (Vec<Color>, usize) {
    let width = 1.0 / max_shades as f64;
    let index = ((lerp.position)(base_color) / width).floor() as usize;
    let modulus = (lerp.position)(base_color) % width;

    ((0 .. max_shades)
        .map(|x| width * (x as f64) + modulus)
        .map(|t| (lerp.lerp)(base_color, t))
        .collect(), index)
}


use serde_json::{Map, Value};

#[derive(Debug, Clone)]
pub struct NamedColor {
    color: Color,
    name: String
}   

impl NamedColor {
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn name(&self) -> &str {
        &self.name
    }   
}

impl MetricPoint for NamedColor {
    type Dist = f64;

    fn dist(from: &Self, to: &Self) -> Self::Dist {
        MetricPoint::dist(&from.color, &to.color)
    }
}

impl PartialEq for NamedColor {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub type ColorDB = VPTree<f64, NamedColor>;
pub struct Lerp {
    pub lerp: Box<dyn Fn(Color, f64) -> Color>,
    pub position: Box<dyn Fn(Color) -> f64>,
}

pub fn load_db(path: &Path) -> Result<ColorDB, io::Error> {
    let data = fs::read_to_string(path)?;
    let db: Map<String, Value> = serde_json::from_str(&data)?;
    let db: Vec<NamedColor> = db.into_iter()
        .map(|(entry, val)| {
            if let Value::String(val) = val {
                NamedColor{
                    color: Color::from_hex(&entry),
                    name: val
                }
            } else {
                panic!("String values expected");
            }
        })
        .collect();

    let vpt = VPTree::build(db);
    println!("Database constructed. Height = {}.", vpt.height);
    Ok(vpt)
}

pub fn quantize_color(db: &ColorDB, color: Color) -> &NamedColor {
    let named_color = NamedColor{ color, name: "".to_owned() };
    db.nearest(&named_color)
}


// pub fn shades_quantized(db: &ColorDB, base_color: Color, max_shades: usize, lerp: Lerp) -> Vec<&NamedColor> {
//     let mut shades = (0 .. max_shades)
//         .map(|x| (x as f64) / (max_shades as f64))
//         .map(|t| quantize_color(db, lerp(base_color, t)))
//         .collect::<Vec<&NamedColor>>();

//     shades.dedup();

//     shades
// }
