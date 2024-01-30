#[derive(Debug, PartialEq)]
pub enum AspectRatio {
    FourThree,
    SixteenNine,
    TwoThree,
    OneOne,
}


fn find_closest_ratio(aspect_ratio: f64) -> f64 {
    let supported_ratios = vec![4.0 / 3.0, 16.0 / 9.0, 2.0 / 3.0, 1.0];

    let mut closest_ratio = supported_ratios[0];

    for &ratio in &supported_ratios[1..] {
        if (ratio - aspect_ratio).abs() < (closest_ratio - aspect_ratio).abs() {
            closest_ratio = ratio;
        }
    }

    closest_ratio
}

pub fn get_aspect_ratio(width: f64, height: f64) -> AspectRatio {
    let aspect_ratio = width / height;
    
    let closest_ratio = find_closest_ratio(aspect_ratio);

    match closest_ratio {
        r if r == 4.0 / 3.0 => AspectRatio::FourThree,
        r if r == 16.0 / 9.0 => AspectRatio::SixteenNine,
        r if r == 2.0 / 3.0 => AspectRatio::TwoThree,
        r if r == 1.0 => AspectRatio::OneOne,
        _ => panic!("Unsupported aspect ratio"),
    }
}