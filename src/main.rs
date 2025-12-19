use std::default::{self, Default};

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
        self.find_entry(self.root_idx, pos, len);
    }

    fn find_entry(&mut self, node_idx: usize, pos: usize, len: usize) {
        match &self.nodes[node_idx] {
            TreeNode::Empty => {
                // create new node
                self.nodes.push(TreeNode::Leaf {
                    pos,
                    len,
                    offset: self.data.len(),
                });
                self.root_idx = self.nodes.len() - 1;
            }
            TreeNode::Branch(branch_node) => {
                if pos + len < branch_node.pos {
                    // no overlap
                } else if pos + len > branch_node.pos + branch_node.len {
                    // no overlap
                } else {
                    // there is overlap
                    if pos <= branch_node.pos && pos + len >= branch_node.pos + branch_node.len {
                        // full overlap
                        // branch_node can be dropped
                    } else if pos == branch_node.pos {
                        // new node with data pos+len .. branch_node.pos + branch_node.len
                    } else if pos + len == branch_node.pos + branch_node.len {
                        // new node with data branch_node.pos .. pos
                    } else {
                        unreachable!()
                    }
                }
            }
            TreeNode::Leaf { pos, len, offset } => todo!(),
        }
    }
}

#[derive(Debug, Default)]
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

#[derive(Debug, Default)]
struct BranchTreeNode {
    pos: usize,
    len: usize,
    children: Vec<TreeNode>,
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
}
