extern crate termion;
extern crate image;

use std::env;

use termion::color::{Fg, Bg, Rgb, Reset};
use image::GenericImage;

fn main() {
    if let Some(filename) = env::args().nth(1) {
        let image = image::open(&filename).unwrap();
        let (width, height) = image.dimensions();
        // Each row of characters in the terminal corresponds to two rows in the image.
        // A half-block character is used to represent two pixels, with the foreground
        // colour being that of the top pixel, and the background colour being the
        // bottom pixel.
        for y in 0..((height + 1) / 2) {
            for x in 0..width {
                let data = image.get_pixel(x, y*2).data;
                // Set top pixel colour
                print!("{}", Fg(Rgb(data[0], data[1], data[2])));
                // If the image has an odd height, the last row of pixels
                if image.in_bounds(x, y*2 + 1) {
                    let data = image.get_pixel(x, y*2 + 1).data;
                    // Set bottom pixel colour
                    print!("{}", Bg(Rgb(data[0], data[1], data[2])));
                } else {
                    // Leave the bottom at the default colour.
                    // In terminals with a transparent background, this will be transparent as well.
                    print!("{}", Bg(Reset));
                }
                print!("▀");
            }
            println!("{}{}", Fg(Reset), Bg(Reset));
        }
    } else {
        println!("Please provide the path to an image as an argument.");
    }
}
