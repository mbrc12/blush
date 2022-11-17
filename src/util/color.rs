/* A tristimulus color space; uses lch for perceptual uniformity, but normalized to 0 - 1 */

use std::{path::Path, fs, io};

use egui::Color32;

use super::vptree::{VPTree, MetricPoint};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub lightness: f64,
    pub chroma: f64,
    pub hue: f64
}

fn hex_digit_to_int(x: u8) -> u8 {
    if x >= 97 {
        x - 97 + 10
    } else {
        x - 48
    }
}

fn int_to_hex_digit(x: u8) -> u8 {
    if x < 10 {
        x + 48
    } else {
        x - 10 + 97
    }
}

impl Color {
    pub fn from_hex(hex: &str) -> Self {
        let b = hex[1..]
            .to_lowercase()
            .as_bytes()
            .iter()
            .map(|x| {hex_digit_to_int(*x) as f64})
            .collect::<Vec<f64>>();

        assert!(b.len() == 6);

        let (r, g, b) = (b[0] * 16.0 + b[1], b[2] * 16.0 + b[3], b[4] * 16.0 + b[5]);

        let (hue, saturation, lightness) = hsluv::rgb_to_hsluv(r/256.0, g/256.0, b/256.0);

        Color {
            hue: hue/360.,
            saturation: saturation/100.,
            lightness: lightness/100.
        }
    }

    pub fn to_hex(self) -> String {
        let Color { hue, saturation, lightness } = self;
        let (r, g, b) = hsluv_to_rgb(hue * 360., saturation * 100., lightness * 100.);
        let r = (r * 256.).round() as u8;
        let g = (g * 256.).round() as u8;
        let b = (b * 256.).round() as u8;

        let bytes = [(r as u8)/16, (r as u8) % 16, (g as u8) / 16, (g as u8) % 16,
        (b as u8)/16, (b as u8) % 16]
            .map(int_to_hex_digit);

        let mut s = String::from_utf8(bytes.into()).unwrap();
        s.insert(0, '#');
        s
    }

    pub fn to_color32(self) -> Color32 {
        let Color { hue, saturation, lightness } = self;
        let (r, g, b) = hsluv_to_rgb(hue * 360., saturation * 100., lightness * 100.);
        let r = (r * 256.).round() as u8;
        let g = (g * 256.).round() as u8;
        let b = (b * 256.).round() as u8;

        Color32::from_rgb(r as u8, g as u8, b as u8)
    }
}

#[test]
fn test_inverse_color() {
    let hex = "#de5d83";
    assert_eq!(hex, Color::from_hex(hex).to_hex());
}


/* Bad implementation for color distance, ideally one should convert to Luv and then compute
 * distances */
impl MetricPoint for Color {
    type Dist = f64;

    fn dist(from: &Self, to: &Self) -> Self::Dist {
        let Color{lightness: l1, chroma: c1, hue: h1} = from;
        let Color{lightness: l2, chroma: c2, hue: h2} = to;
        
        let hdist = (h1 - h2).abs().min(1. - (h1 - h2).abs());
        hdist + (c2 - c1).abs() + (l2 - l1).abs()
    }
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
pub type Lerp = Box<dyn Fn(Color, f64) -> Color>;

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

pub fn lightness_lerp(start: f64, end: f64) -> Lerp {
    Box::new(move |color: Color, t| Color{lightness: start + (end - start) * t, ..color})
}

pub fn chroma_lerp(start: f64, end: f64) -> Lerp {
    Box::new(move |color: Color, t| Color{chroma: start + (end - start) * t, ..color})
}

pub fn hue_lerp(start: f64, end: f64) -> Lerp {
    Box::new(move |color: Color, t| Color{hue: start + (end - start) * t, ..color})
}

pub fn shades(base_color: Color, max_shades: usize, lerp: Lerp) -> Vec<Color> {
    (0 .. max_shades)
        .map(|x| (x as f64) / (max_shades as f64))
        .map(|t| lerp(base_color, t))
        .collect()
}

pub fn shades_quantized(db: &ColorDB, base_color: Color, max_shades: usize, lerp: Lerp) -> Vec<&NamedColor> {
    let mut shades = (0 .. max_shades)
        .map(|x| (x as f64) / (max_shades as f64))
        .map(|t| quantize_color(db, lerp(base_color, t)))
        .collect::<Vec<&NamedColor>>();

    shades.dedup();

    shades
}
