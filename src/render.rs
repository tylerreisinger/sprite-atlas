use std::path::Path;

use image;
use crate::spatial_tree::{Region, SpatialTree};

#[repr(C)]
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Rgba {
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba {
        Rgba { r, g, b, a }
    }
}

static COLORS: &'static [Rgba] = &[
    Rgba::new(255, 0, 0, 255),
    Rgba::new(0, 255, 0, 255),
    Rgba::new(0, 0, 255, 255),
    Rgba::new(255, 0, 255, 255),
    Rgba::new(0, 255, 255, 255),
    Rgba::new(0, 0, 0, 255),
];

fn draw_rectangle(color: Rgba, region: &Region, width: u32, pixels: &mut [Rgba]) {
    for y in (region.top..region.top+region.height) {
        for x in (region.left..region.left+region.width) {
            pixels[(x + y*width) as usize] = color;
        }
    }
}

pub fn draw_spatial_tree_nodes<T>(tree: &mut SpatialTree<T>, path: &Path) {
    let region = tree.region().clone();
    let mut pixels = vec![Rgba::new(255, 255, 255, 255); region.area() as usize];
    let mut i = 0;
    for node in tree.iter_nodes() {
        if let Some(v) = &node.value {
            draw_rectangle(COLORS[i % COLORS.len()], &node.region, region.width, &mut pixels);
            i += 1;
        }
    }

    let u8_pixels = unsafe { Vec::from_raw_parts(std::mem::transmute::<_, *mut u8>(pixels.as_mut_ptr()), (region.area()*4) as usize, (region.area()*4) as usize) };
    let img = image::RgbaImage::from_raw(tree.region().width, tree.region().height, u8_pixels).unwrap();
    img.save(path).unwrap();
}