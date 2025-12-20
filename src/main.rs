use range_overlap::{RangeOverlap, excl_classify};
use std::default::Default;

#[derive(Debug, Default)]
struct Tree {
    data: Vec<u8>,
    nodes: Vec<TreeNode>,
    root_idx: usize,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            nodes: vec![Default::default()],
            ..Default::default()
        }
    }

    fn write(&mut self, pos: usize, len: usize) {
        self.insert(pos, len);
    }

    fn read(&self, pos: usize, buf: &mut [u8]) {}

    fn insert(&mut self, pos: usize, len: usize) {
        let mut current_node = self.root_idx;

        loop {
            let node = &self.nodes[current_node];
            match node {
                TreeNode::Empty => {
                    // create new node
                    self.nodes.push(TreeNode::Leaf {
                        pos,
                        len,
                        offset: self.data.len(),
                    });
                    self.root_idx = self.nodes.len() - 1;
                    return;
                }
                TreeNode::Branch(branch_node) => {}
                TreeNode::Leaf {
                    pos: leaf_pos,
                    len: leaf_len,
                    offset,
                } => match excl_classify(pos, pos + len, *leaf_pos, leaf_pos + leaf_len) {
                    RangeOverlap::None => todo!(),
                    _ => todo!(),
                },
            }
        }
    }
}

#[derive(Debug, Default, Copy, Clone)]
enum TreeNode {
    #[default]
    Empty,
    Leaf {
        pos: usize,
        len: usize,
        offset: usize,
    },
    Branch(BranchTreeNode),
}

#[derive(Debug, Default, Copy, Clone)]
struct BranchTreeNode {
    pos: usize,
    len: usize,
    children: [usize; 2],
}

#[derive(Debug)]
enum InsertResult {
    NewLeaf,
    Done,
}

// empty
//
// +w 0 100
//
// n 0
// root {
// [
//   0 100 -> n0_pos
// ]
// }
//
// +w 0 50
//
// n 0
// root {
// [
//   0 100 -> n0_pos
// ]
// }
// n 1
// root {
//   [
//     0 50 -> n1_pos
//     50 100 -> n0_pos + 50
//   ]
//   }

fn main() {
    let mut tree = Tree::new();

    tree.write(0, 100);
    println!("{:?}", tree);
    tree.write(100, 110);
    println!("{:?}", tree);
    tree.write(100, 110);
}
