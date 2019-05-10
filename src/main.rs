use std::path::Path;

pub mod options;
pub mod input;
pub mod spatial_tree;
pub mod render;

use image::GenericImageView;

fn main() {
    let config = options::parse_ops();

    let files = input::get_input_files(&config);
    for file in &files {
        println!("{}", file.display());
    }
    let images = files.iter().map(|path| input::load_image(&path)).collect::<Vec<_>>();

    let mut tree = spatial_tree::SpatialTree::new();

    tree.insert(1, 100, 50);
    tree.insert(2, 50, 50);
    tree.insert(3, 25, 30);
    tree.insert(4, 25, 30);
    for item in tree.iter_nodes() {
        println!("Node {:?}", item.value);
    }
    println!("{}", tree);

    render::draw_spatial_tree_nodes(&mut tree, Path::new("out.png"));
}
