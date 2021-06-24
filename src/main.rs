#![allow(non_snake_case)]
mod src;

use src::render;

use render::BmpImageInfo;
use render::BmpImageInfoFuncs;

/*
0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 255, 255,
        0, 0, 255, 255,
        0, 0, 0, 0,
        0, 0, 255, 255,
        0, 0, 255, 255,
        0, 0, 0, 0,
        0, 0, 0, 0, 
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,

0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 255, 255,
        0, 0, 255, 255,
        0, 0, 255, 255,
        0, 0, 0, 0,
        0, 0, 0, 0,
        0, 0, 0, 0, 
        255, 255, 255,
*/

fn main() {
    let mut bmp_image = BmpImageInfo::new_bmp(3, 3);
    match bmp_image.configure_bmp(vec![
        0, 0, 0,
        0, 0, 0,
        0, 0,
        0, 0, 0,
        0, 0, 0,
        0, 0,
        255, 0, 0, 
        0, 0, 0,
        255, 255, 255,
        255, 255, 255,
        255, 255, 255,
        255, 255, 255,
        0, 0
    ]) {
        Ok(mut r) => {
            match r.write_bmp("img6.bmp".to_string()) {
                Ok(s) => println!("{:?}", s),
                Err(e) => panic!("{:?}", e)
            }
        },
        Err(e) => panic!("{:?}", e)
    }
}
