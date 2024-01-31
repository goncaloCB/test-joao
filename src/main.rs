mod aspect_ratio;

fn main() {
    let result = aspect_ratio::get_aspect_ratio(4.0, 4.0);
    println!("Result: {:?}", result);
}
