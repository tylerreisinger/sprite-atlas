use std::path::Path;
use std::u32;

pub mod options;
pub mod input;
pub mod spatial_tree;
pub mod render;

use rand::distributions::Distribution;

fn main() {
    let config = options::parse_ops();
    sprite_sheet_main(config);
}

pub fn sprite_sheet_main(config: options::InputOptions) {
    let files = input::get_input_files(&config);
    for file in &files {
        println!("{}", file.display());
    }
    let mut images = files.iter()
        .map(|path| (path.clone(), input::load_image(&path)))
        .filter(|f| f.1.is_some())
        .map(|f| (f.0, f.1.unwrap().to_rgba()))
        .collect::<Vec<_>>();
    images.sort_unstable_by_key(|(_, img)| u32::MAX - img.width().max(img.height()));

    let mut tree = spatial_tree::SpatialTree::new();

    for image in images {
        let width = image.1.width();
        let height = image.1.height();
        tree.insert(image, width, height)
    }

    render::draw_spatial_tree_sprites(&mut tree, Path::new("sprites.png"))
}

fn generate_random_test_data(n: u32) -> Vec<(u32, u32, u32)> {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Normal::new(80.0, 20.0);

    let mut values = (0..n)
        .map(|x| (x, dist.sample(&mut rng).max(5.0) as u32, dist.sample(&mut rng).max(5.0) as u32)).collect::<Vec<_>>();

    values.sort_unstable_by_key(|(i, w, h)| u32::MAX - w.max(h));

    values
}

fn generate_discrete_test_data(n: u32, choices: &[u32]) -> Vec<(u32, u32, u32)> {
    let mut rng = rand::thread_rng();
    let dist = rand::distributions::Uniform::new(0, choices.len()-1);

    let mut values = (0..n)
        .map(|x| (x, choices[dist.sample(&mut rng)], choices[dist.sample(&mut rng)])).collect::<Vec<_>>();

    values.sort_unstable_by_key(|(i, w, h)| u32::MAX - w*h/*w.max(h)*/);

    values
}