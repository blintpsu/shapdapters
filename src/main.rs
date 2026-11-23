
use rand;

/// Shape consts
/// I'm using consts due to problems I ran into picking a random enum value.
/// TODO: Will return to try to get enum working if time.
const SHAPE_SQUARE: usize = 0;
const SHAPE_CIRCLE: usize = 1;
const SHAPE_VERTICAL_RECTANGLE: usize = 2;
const SHAPE_HORIZONTAL_RECTANGLE: usize = 3;

/// An invalid grid size.
/// This enum was adpated from Bart Massey's Chomp library skeleton.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapdapterError {
    /// Shapes must not be 0, 0 and must not be generated on a square grid. 
    /// Any other size raises this image.
    InvalidSize {
        /// The requested width.
        width: usize,
        /// The requested number of columns.
        height: usize,
    },
}

impl std::error::Error for ShapdapterError {}

/// This formatter was adpated from Bart Massey's Chomp library skeleton.
impl std::fmt::Display for ShapdapterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSize { width, height } => {
                write!(f, "Tile size must be square and not have 0 width or height: {width} rows, {height} columns")
            }
        }
    }
}

/// Initializes a tile with transparent pixels.
/// Hex value 0xFFFFFF00 
/// (NOTE: Default AI response gave me the 0x format for hex values)
fn initialize_tile(width: usize, height: usize) -> Vec<usize> {
    let mut tile = Vec::<usize>::new();
    for i in 0..width*height {
        tile.push (0xFFFFFF00);
    }

    return tile;
}

/// Generate a vertical rectangle
/// NOTE:  width and height represents the tile size.  In the case of the 
/// vertical rectangle it fills in half the columns in the center of the tile 
/// top to bottom.
fn generate_vertical_rectangle (width: usize, height: usize) -> Vec<usize> {
    
    let mut rectangle = Vec::<usize>::new();

    for i in 0..height {
        for j in 0..width {
            if j >= width/4 && j < width-width/4 {
                rectangle.push(0x000000);
            }
            else {
                rectangle.push (1);
            }
        }
    }
    
    return rectangle;
}

/// Generate a horizontal rectangle
/// NOTE:  width and height represents the tile size.  In the case of the 
/// horizontal rectangle it fills in half the columns in the center of the tile 
/// left to right
fn generate_horizontal_rectangle (width: usize, height: usize) -> Vec<usize> {
    let mut rectangle = Vec::<usize>::new();

    for i in 0..height {
        for j in 0..width {
            if i >= height/4 && i < height-height/4 {
                rectangle.push(0x000000);
            }
            else {
                rectangle.push (1);
            }
        }
    }
    
    return rectangle;
}

/// Generate a square by filling the entire width and height with 0 (black)
/// NOTE:  width and height represents the tile size.  In the case of the 
/// square it fills the entire tile but that's not the case for all shapes.
fn generate_square (width: usize, height: usize) -> Vec<usize> {
    let mut square = Vec::<usize>::new();
    for i in 0..width*height-1 {
        square.push(0x000000);
    }
    
    return square;
}

/// Generate a circle.
/// Uses this algorithm: 
/// https://groups.csail.mit.edu/graphics/classes/6.837/F98/Lecture6/circle.html
fn generate_square (width: usize, height: usize) -> Vec<usize> {
}



fn generate_shape(width: usize, height: usize) -> Result<Vec<usize>, ShapdapterError> {
    // Error code adapted from Bart Massey's Chomp library skeleton.  tiles 
    // must not have a 0 dimension, must be square and must be a multiple of 2
    if width == 0 || height == 0 || width != height || width % 2 != 0 {
            return Err(ShapdapterError::InvalidSize { width, height });
    }
    
    let random_number = rand::random_range(0..=3);
    let mut shape = Vec::new();

    shape = initialize_tile(width, height);
    
    match random_number {
        SHAPE_SQUARE => shape = generate_square(width, height),
        SHAPE_CIRCLE => println! ("Circle generation needs to be implemented"),
        SHAPE_VERTICAL_RECTANGLE => shape = generate_vertical_rectangle(width, height),
        SHAPE_HORIZONTAL_RECTANGLE => shape = generate_horizontal_rectangle(width, height),
        _ => println! ("Default shape generation needs to be implemented"),
    }

    return Ok(shape);
}

fn print_shape (shape: Vec::<usize>, tile_width: usize, tile_height: usize) {
    if shape.len() > 0 {
        for i in 0..tile_width {
            for j in 0..tile_height {
                if shape[i*tile_height+j] != 0xFFFFFF00 {
                    print!("{}", shape[i*tile_height+j]);
                }
            }
            println!();
        }
    }
} 

fn main() {
    let tile_width: usize = 16;
    let tile_height: usize = 16;

    let shape = generate_shape(tile_width, tile_height);
    match shape {
        Ok(s) => print_shape (s, tile_width, tile_height),
        Err(ShapdapterError::InvalidSize { .. }) => println!("Shape was of invalid size"),
    }
}
