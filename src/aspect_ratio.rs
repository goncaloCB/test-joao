use serde_derive::{Serialize, Deserialize};


#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AspectRatio {
    FourThree,
    SixteenNine,
    TwoThree,
    OneOne,
}

impl AspectRatio {
    pub fn ratio(&self) -> f64 {
        match *self {
            AspectRatio::FourThree => 4.0 / 3.0,
            AspectRatio::SixteenNine => 16.0 / 9.0,
            AspectRatio::TwoThree => 2.0 / 3.0,
            AspectRatio::OneOne => 1.0,
        }
    }
}


pub fn get_aspect_ratio(width: f64, height: f64) -> AspectRatio {
    use AspectRatio::*;
    let aspect_ratio = width / height;

    [FourThree, SixteenNine, TwoThree, OneOne]
        .iter()
        .min_by(|a, b| {
            (a.ratio() - aspect_ratio).abs().partial_cmp(&(b.ratio() - aspect_ratio).abs()).unwrap()
        })
        .copied()
        .unwrap()
}