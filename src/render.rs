use std::path::{Path, PathBuf};

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
    Rgba::new(255, 127, 0, 255),
    Rgba::new(127, 0, 0, 255),
    Rgba::new(0, 127, 0, 255),
    Rgba::new(0, 0, 127, 255),
    Rgba::new(127, 0, 127, 255),
];

fn draw_border(color: Rgba, region: &Region, width: u32, pixels: &mut [Rgba]) {
    for x in (region.left..region.left+region.width) {
        unsafe { *pixels.get_unchecked_mut((x + width*region.top) as usize) = color; }
        unsafe { *pixels.get_unchecked_mut((x + width*region.bottom()) as usize) = color; }
    }
    for y in (region.top..region.top+region.height) {
        unsafe { *pixels.get_unchecked_mut((region.left + width*y) as usize) = color; }
        unsafe { *pixels.get_unchecked_mut((region.right() + width*y) as usize) = color; }
    }
}

fn draw_rectangle(color: Rgba, region: &Region, width: u32, pixels: &mut [Rgba]) {
    for y in (region.top..region.top+region.height) {
        for x in (region.left..region.left+region.width) {
            unsafe { *pixels.get_unchecked_mut((x + y*width) as usize) = color; }
        }
    }
}

fn blit_sprite(sprite: &image::RgbaImage, region: &Region, width: u32, pixels: &mut [Rgba]) {
    for (image_y, sprite_y) in (region.top..region.top+region.height).zip(0..region.height) {
        for (image_x, sprite_x) in (region.left..region.left + region.width).zip(0..region.width) {
            let pixel = sprite.get_pixel(sprite_x, sprite_y);
            let color = Rgba::new(pixel.data[0], pixel.data[1], pixel.data[2], pixel.data[3]);
            unsafe { *pixels.get_unchecked_mut((image_x + image_y*width) as usize) = color; }
        }
    }
}

pub fn draw_spatial_tree_nodes<T>(tree: &mut SpatialTree<T>, path: &Path) {
    let region = tree.region().clone();
    let mut pixels = vec![Rgba::new(255, 255, 255, 255); region.area() as usize];
    let mut i = 0;
    for node in tree.iter_nodes() {
        if let Some(v) = &node.value {
            let inner_region = Region::new(node.region.top, node.region.left, node.value_size.0, node.value_size.1);
            draw_rectangle(COLORS[i % COLORS.len()], &inner_region, region.width, &mut pixels);
            i += 1;
        }
    }

    let u8_pixels = unsafe { Vec::from_raw_parts(std::mem::transmute::<_, *mut u8>(pixels.as_mut_ptr()), (region.area()*4) as usize, (region.area()*4) as usize) };
    let img = image::RgbaImage::from_raw(tree.region().width, tree.region().height, u8_pixels).unwrap();
    std::mem::forget(pixels);
    img.save(path).unwrap();
}

pub fn draw_spatial_tree_sprites(tree: &mut SpatialTree<(PathBuf, image::RgbaImage)>, path: &Path) {
    let region = tree.region().clone();
    let mut pixels = vec![Rgba::new(255, 255, 255, 255); ((region.width+1)*(region.height+1)) as usize];
    for node in tree.iter_nodes() {
        if let Some(v) = &node.value {
            let inner_region = Region::new(node.region.top, node.region.left, node.value_size.0, node.value_size.1);
            blit_sprite(&v.1, &inner_region, region.width, &mut pixels);
        }
        draw_border(Rgba::new(255, 0, 255, 255), &node.region, region.width, &mut pixels);
    }
    let u8_pixels = unsafe { Vec::from_raw_parts(std::mem::transmute::<_, *mut u8>(pixels.as_mut_ptr()), (region.area()*4) as usize, (region.area()*4) as usize) };
    let img = image::RgbaImage::from_raw(tree.region().width, tree.region().height, u8_pixels).unwrap();
    std::mem::forget(pixels);
    img.save(path).unwrap();
}