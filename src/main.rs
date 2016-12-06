extern crate termion;
extern crate image;

use std::env;

use termion::color::{Fg, Bg, Rgb, Reset};
use image::GenericImage;

fn main() {
    if let Some(filename) = env::args().nth(1) {
        let image = image::open(&filename).unwrap();
        let (_, height) = image.dimensions();
        for y in 0..((height + 1) / 2) {
            for x in 0..image.width() {
                let data = image.get_pixel(x, y*2).data;
                let top_pixel = Fg(Rgb(data[0], data[1], data[2]));
                let bottom_pixel;
                if image.in_bounds(x, y*2 + 1) {
                    let data = image.get_pixel(x, y*2 + 1).data;
                    bottom_pixel = Bg(Rgb(data[0], data[1], data[2]));
                } else {
                    bottom_pixel = Bg(Rgb(0,0,0));
                }
                print!("{}{}▀", top_pixel, bottom_pixel);
            }
            println!("{}{}", Fg(Reset), Bg(Reset));
        }
    } else {
        println!("Please provide the path to an image as an argument.");
    }
}
