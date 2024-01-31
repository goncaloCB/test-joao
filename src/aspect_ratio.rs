#[derive(Debug, PartialEq, Clone)]
pub enum AspectRatio {
    FourThree,
    SixteenNine,
    TwoThree,
    OneOne,
}


fn find_closest_ratio(aspect_ratio: f64) -> Option<(AspectRatio, f64)> {
    use AspectRatio::*;

    let supported_ratios = [
        (FourThree, 4.0 / 3.0), 
        (SixteenNine, 16.0 / 9.0),
        (TwoThree, 2.0 / 3.0),
        (OneOne, 1.0)    
    ];

    if let Some((name, closest_ratio)) = supported_ratios.iter().min_by(|&(_, a), &(_, b)| {
        (a - aspect_ratio).abs().partial_cmp(&(b - aspect_ratio).abs()).unwrap()
    }) {
        Some((name.clone(), *closest_ratio))
    } else {
        None
    }
}

pub fn get_aspect_ratio(width: f64, height: f64) -> AspectRatio {
    let aspect_ratio = width / height;
    
    match find_closest_ratio(aspect_ratio) {
        Some((name, closest)) => name,
        None => AspectRatio::OneOne,
    }
}