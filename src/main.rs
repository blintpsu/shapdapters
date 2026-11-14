
use rand;

/// Shape consts
/// I'm using consts due to problems I ran into picking a random enum value.
/// TODO: Will return to try to get enum working if time.
const SHAPE_SQUARE: usize = 0;
const SHAPE_CIRCLE: usize = 1;
const SHAPE_VERTICAL_RECTANGLE: usize = 2;
const SHAPE_HORIZONTAL_RECTANGLE: usize = 3;


fn generate_shape() {
    let random_number = rand::random_range(0..=3);
    println!("Random Number = {}", random_number);
}

fn main() {
    generate_shape();
}
