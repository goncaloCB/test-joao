#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AspectRatio {
    FourThree,
    SixteenNine,
    TwoThree,
    OneOne,
}


pub fn get_aspect_ratio(width: f64, height: f64) -> AspectRatio {
    use AspectRatio::*;
    let aspect_ratio = width / height;

    let supported_ratios = [
        (FourThree, 4.0 / 3.0), 
        (SixteenNine, 16.0 / 9.0),
        (TwoThree, 2.0 / 3.0),
        (OneOne, 1.0)    
    ];

    supported_ratios
        .iter()
        .min_by(|&(_, a), &(_, b)| (a - aspect_ratio).abs().partial_cmp(&(b - aspect_ratio).abs()).unwrap())
        .map(|(name, _)| *name)
        .expect("Unsupported aspect ratio")

}