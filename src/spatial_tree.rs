use std::collections::HashSet;
use std::iter::{Iterator, ExactSizeIterator, FusedIterator};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Region {
    pub top: u32,
    pub left: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, Default)]
pub struct SpatialNode<T> {
    pub region: Region,
    pub value: Option<T>,
    pub value_size: (u32, u32),
    pub right: Option<Box<SpatialNode<T>>>,
    pub bottom: Option<Box<SpatialNode<T>>>,
    pub parent: Option<*mut SpatialNode<T>>,
}

#[derive(Debug)]
pub struct SpatialTree<T> {
    num_items: usize,
    region: Region,
    root: Option<Box<SpatialNode<T>>>,
    free_regions: HashSet<*mut SpatialNode<T>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TraverseDirection {
    Right,
    Down,
    None,
}

#[derive(Debug)]
pub struct SpatialTreeNodeIterMut<'a, T> {
    tree: &'a SpatialTree<T>,
    node: Option<*mut SpatialNode<T>>,
    stack: Vec<TraverseDirection>,
}
#[derive(Debug)]
pub struct SpatialTreeIter<'a, T> {
    node_iter: SpatialTreeNodeIterMut<'a, T>,
}
#[derive(Debug)]
pub struct SpatialTreeIterMut<'a, T> {
    node_iter: SpatialTreeNodeIterMut<'a, T>,
}

impl Region {
    pub fn new(top: u32, left: u32, width: u32, height: u32) -> Region {
        Region {
            top,
            left,
            width,
            height,
        }
    }

    pub fn right(&self) -> u32 {
        self.left + self.width
    }

    pub fn bottom(&self) -> u32 {
        self.top + self.height
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn max_size(&self) -> u32 {
        self.width.max(self.height)
    }
}

impl<T> SpatialNode<T> {
    pub fn empty(region: Region) -> SpatialNode<T> {
        SpatialNode {
            region,
            value: None,
            right: None,
            bottom: None,
            parent: None,
            value_size: (0, 0),
        }
    }
}

impl<'a, T> SpatialTree<T> {
    pub fn iter_nodes(&'a mut self) -> SpatialTreeNodeIterMut<'a, T> {
        SpatialTreeNodeIterMut::new(self)
    }
    /*
    pub fn iter(&'a self) -> SpatialTreeIter<'a, T> {
        SpatialTreeIter { node_iter: SpatialTreeNodeIter::new(unsafe { std::mem::transmute::<_, &'a mut Self>(self) }) }
    }*/
    pub fn iter_mut(&'a mut self) -> SpatialTreeIter<'a, T> {
        SpatialTreeIter { node_iter: SpatialTreeNodeIterMut::new(self) }
    }
}

impl<T> SpatialTree<T> {
    pub fn new() -> SpatialTree<T> {
        SpatialTree {
            num_items: 0,
            region: Region::default(),
            root: None,
            free_regions: HashSet::new(),
        }
    }

    pub fn with_initial_size(width: u32, height: u32) -> SpatialTree<T> {
        SpatialTree {
            num_items: 0,
            region: Region::new(0, 0, width, height),
            root: None,
            free_regions: HashSet::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.num_items
    }
    pub fn region(&self) -> &Region {
        &self.region
    }

    pub fn insert(&mut self, item: T, width: u32, height: u32) {
        let region = Region {
            top: 0,
            left: 0,
            width,
            height
        };
        if let Some(root) = self.root.as_mut() {
            let mut target_node = None;
            for node in self.iter_nodes() {
                if node.value.is_none() && node.region.width >= width && node.region.height >= height && node.right.is_none() {
                    target_node = Some(unsafe { &mut *std::mem::transmute::<_, *mut SpatialNode<T>>(node) });
                    break;
                }
            }
            if let Some(node) = target_node {
                self.split_node(node, &region);
                node.value = Some(item);
                node.value_size = (width, height);
            } else {
                let node = unsafe{&mut *std::mem::transmute::<_, *mut SpatialNode<T>>(self.resize(&region))};
                self.split_node(node, &region);
                node.value = Some(item);
                node.value_size = (width, height);
            }
        } else {
            if self.region.width == 0 {
                self.region = region.clone();
            }
            let mut new_root = Box::new(SpatialNode {
                region: self.region.clone(),
                value: Some(item),
                right: None,
                bottom: None,
                parent: None,
                value_size: (width, height),
            });
            self.split_node(new_root.as_mut(), &region.clone());
            self.root = Some(new_root);
        }
        self.num_items += 1;
    }

    fn split_node<'a>(&mut self, node: &'a mut SpatialNode<T>, new_region: &Region)
        -> (&'a mut SpatialNode<T>, &'a mut SpatialNode<T>)
    {
        assert!(node.right.is_none() && node.bottom.is_none());
        let right_region = Region {
            left: node.region.left + new_region.width,
            top: node.region.top,
            width: node.region.width - new_region.width,
            height: new_region.height,
        };
        let bottom_region = Region {
            left: node.region.left,
            top: node.region.top + new_region.height,
            width: node.region.width,
            height: node.region.height - new_region.height,
        };
        let mut right_node = Box::new(SpatialNode::empty(right_region));
        let mut bottom_node = Box::new(SpatialNode::empty(bottom_region));
        right_node.as_mut().parent = Some(node);
        bottom_node.as_mut().parent = Some(node);
        self.free_regions.insert(right_node.as_mut() as *mut SpatialNode<T>);
        self.free_regions.insert(bottom_node.as_mut() as *mut SpatialNode<T>);
        node.right = Some(right_node);
        node.bottom = Some(bottom_node);
        (node.right.as_mut().unwrap().as_mut(), node.bottom.as_mut().unwrap().as_mut())

    }

    fn resize(&mut self, new_region: &Region) -> &mut SpatialNode<T> {
        let new_width = self.region.width + new_region.width;
        let new_height = self.region.height + new_region.height;

        if new_width < new_height {
            if new_region.height > self.region.height {
                //TODO
                //panic!("Height overflow")
                self.resize_down(self.region.width, new_region.height);
            }
            self.resize_right(new_width, self.region.height)
        } else {
            if new_region.width > self.region.width {
                //TODO
                //panic!("Width overflow")
                self.resize_right(new_region.width, self.region.height);
            }
            self.resize_down(self.region.width, new_height)
        }
    }

    fn resize_right(&mut self, new_width: u32, height: u32) -> &mut SpatialNode<T> {
        let full_region = Region {
            left: 0,
            top: 0,
            width: new_width,
            height,
        };
        let right_region = Region {
            left: self.region.width,
            top: 0,
            width: new_width - self.region.width,
            height,
        };
        let right_node = SpatialNode::empty(right_region);
        let mut new_root= Box::new(SpatialNode {
            region: full_region,
            value: None,
            bottom: self.root.take(),
            right: Some(Box::new(right_node)),
            parent: None,
            value_size: (new_width, height),
        });
        new_root.bottom.as_mut().unwrap().parent = Some(new_root.as_mut());
        new_root.right.as_mut().unwrap().parent = Some(new_root.as_mut());
        self.region = new_root.region.clone();
        self.root = Some(new_root);
        let new_node = self.root.as_mut().unwrap().right.as_mut().unwrap();
        self.free_regions.insert(new_node.as_mut() as *mut SpatialNode<T>);
        new_node
    }

    fn resize_down(&mut self, width: u32, new_height: u32) -> &mut SpatialNode<T> {
        let full_region = Region {
            left: 0,
            top : 0,
            width,
            height: new_height,
        };
        let bottom_region = Region {
            left: 0,
            top: self.region.height,
            width,
            height: new_height - self.region.height,
        };
        let bottom_node = SpatialNode::empty(bottom_region);
        let mut new_root = Box::new(SpatialNode {
            region: full_region,
            value: None,
            bottom: Some(Box::new(bottom_node)),
            right: self.root.take(),
            parent: None,
            value_size: (width, new_height),
        });
        new_root.bottom.as_mut().unwrap().parent = Some(new_root.as_mut());
        new_root.right.as_mut().unwrap().parent = Some(new_root.as_mut());
        self.region = new_root.region.clone();
        self.root = Some(new_root);
        let new_node = self.root.as_mut().unwrap().bottom.as_mut().unwrap();
        self.free_regions.insert(new_node.as_mut() as *mut SpatialNode<T>);
        new_node
    }

}

impl<'a, T: std::fmt::Display> SpatialTree<T> {
    pub fn display_recursive(&self, node: Option<&SpatialNode<T>>, depth: usize) {
        if let Some(n) = node {
            let padding = std::iter::repeat(" ").take(depth * 2).collect::<String>();
            if let Some(value) = &n.value {
                println!("{}{} ({}, {}) {}x{}", padding, value, n.region.top, n.region.left, n.region.width, n.region.height);
            } else {
                println!("{}EMPTY ({}, {}) {}x{}", padding, n.region.top, n.region.left, n.region.width, n.region.height);
            }
            self.display_recursive(n.right.as_ref().map(|x| x.as_ref()), depth+1);
            self.display_recursive(n.bottom.as_ref().map(|x| x.as_ref()), depth+1);
        }
    }
}

impl<'a, T: std::fmt::Display> std::fmt::Display for SpatialTree<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.display_recursive(self.root.as_ref().map(|x| x.as_ref()), 0);
        Ok(())
    }
}


impl<'a, T> SpatialTreeNodeIterMut<'a, T> {
    fn new(tree: &'a mut SpatialTree<T>) -> SpatialTreeNodeIterMut<T> {
        SpatialTreeNodeIterMut {
            node: tree.root.as_mut().map(|x| x.as_mut() as *mut SpatialNode<T>),
            tree,
            stack: vec![TraverseDirection::None],
        }
    }

    fn traverse_right(&mut self) -> bool {
        let node = unsafe { &mut *self.node.unwrap() };
        if let Some(right) = node.right.as_mut().map(|x| x.as_mut()) {
            self.node = Some(right as *mut SpatialNode<T>);
            self.stack.pop();
            self.stack.push(TraverseDirection::Right);
            self.stack.push(TraverseDirection::None);
            true
        } else {
            false
        }
    }

    fn traverse_bottom(&mut self) -> bool {
        let node = unsafe { &mut *self.node.unwrap() };
        if let Some(down) = node.bottom.as_mut().map(|x| x.as_mut()) {
            self.node = Some(down as *mut SpatialNode<T>);
            self.stack.pop();
            self.stack.push(TraverseDirection::Down);
            self.stack.push(TraverseDirection::None);
            true
        } else {
            false
        }
    }

    fn traverse_to_parent(&mut self) {
        self.stack.pop();
        self.node = self.node.as_mut().map(|x| unsafe { &**x }).unwrap().parent.map(|x| x as *mut SpatialNode<T>);
    }
}

impl<'a, T> Iterator for SpatialTreeNodeIterMut<'a, T> {
    type Item = &'a mut SpatialNode<T>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(node) = self.node.map(|x| unsafe { &mut *x }) {
            let orig_dir = self.stack.last().unwrap().clone();
            match self.stack.last().unwrap() {
                TraverseDirection::None => {
                    if !self.traverse_right() && !self.traverse_bottom() {
                        self.traverse_to_parent()
                    }
                },
                TraverseDirection::Right => {
                    if !self.traverse_bottom() {
                        self.traverse_to_parent()
                    }
                },
                TraverseDirection::Down => {
                    self.traverse_to_parent()
                },
            };
            if orig_dir == TraverseDirection::None {
                Some(node)
            } else {
                self.next()
            }
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.tree.num_items, Some(self.tree.num_items))
    }
}

impl<'a, T> ExactSizeIterator for SpatialTreeNodeIterMut<'a, T> {}
impl<'a, T> FusedIterator for SpatialTreeNodeIterMut<'a, T> {}

impl<'a, T> Iterator for SpatialTreeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(node) = self.node_iter.next() {
                if let Some(value) = node.value.as_ref() {
                    return Some(value)
                }
            } else {
                return None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.node_iter.size_hint()
    }
}
impl<'a, T> ExactSizeIterator for SpatialTreeIter<'a, T> {}
impl<'a, T> FusedIterator for SpatialTreeIter<'a, T> {}

impl<'a, T> Iterator for SpatialTreeIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(node) = self.node_iter.next() {
                if let Some(value) = node.value.as_mut() {
                    return Some(value)
                }
            } else {
                return None
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.node_iter.size_hint()
    }
}
impl<'a, T> ExactSizeIterator for SpatialTreeIterMut<'a, T> {}
impl<'a, T> FusedIterator for SpatialTreeIterMut<'a, T> {}
