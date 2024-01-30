mod aspect_ratio;

fn main() {
    let result = aspect_ratio::get_aspect_ratio(2.0, 3.0);
    println!("Result: {:?}", result);
}
