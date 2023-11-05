#![deny(warnings)]
use crate::common::{zero_digest, Digest, hash_one_thing, hash_two_things};
use crate::kv_trait::AuthenticatedKV;


/* 
* A leaf node is a node in the tree that has no children
* and contains a key-value pair. The path of a leaf node
 * is the hash of the key.
 */
#[derive(Debug, Clone)]
pub struct SparseMerkleTreeNodeLeaf {
    key: String,
    value: Option<String>,
    hash: Digest,
}

/* 
* A branch node is an internal node in the tree
* that has two children subtrees, left and right,
*  and a path that is the hash of the concatenation
* of the left and right child nodes.
*/
#[derive(Debug, Clone)]
pub struct SparseMerkleTreeNodeBranch {
    left: Option<Box<SparseMerkleTreeNodeBranch>>,
    right: Option<Box<SparseMerkleTreeNodeBranch>>,
    hash: Digest,
    leaf: Option<SparseMerkleTreeNodeLeaf>,
}

/**
 * A Sparse Merkle Tree is a binary tree where each leaf node 
 * is a (K, V) pair. Internal nodes and root hashes are computed 
 * as the hash of the concatenation of the left and right child nodes. 
 * The root hash of the tree is the commitment of the tree.
 */
#[derive(Debug, Clone)]
pub struct SparseMerkleTree {
    commitment: Digest,
    root: Option<Box<SparseMerkleTreeNodeBranch>>,
    height: u8,
}

#[derive(Debug, Clone)]
pub enum SparseMerkleTreeProof {
    NotPresent {
        next_leaf: SparseMerkleTreeNodeLeaf,
    },
    Present {
        leaf: SparseMerkleTreeNodeLeaf,
    },
}

impl AuthenticatedKV for SparseMerkleTree {
    type K = String;
    type V = String;
    type LookupProof = SparseMerkleTreeProof;
    type Commitment = Digest;

    fn new() -> Self {
        SparseMerkleTree {
            commitment: smk_util::empty_smt_hash(),
            root: smk_util::create_root(),
            // TODO: unhardcode this
            height: 255,
        }
    }

    fn commit(&self) -> Self::Commitment {
        self.commitment
    }

    fn check_proof(
        key: Self::K,
        res: Option<Self::V>,
        pf: &Self::LookupProof,
        comm: &Self::Commitment,
    ) -> Option<()> {
        
        // TODO: unhardcode this as I fix self.height
        let height: u8 = 255;

        match(res, pf) {
            (None, SparseMerkleTreeProof::NotPresent { next_leaf }) => {
                let mut node = SparseMerkleTreeNodeBranch {
                    left: None,
                    right: None,
                    hash: smk_util::empty_smt_hash(),
                    leaf: Some(next_leaf.clone()),
                };
                let mut path: u8 = smk_util::key_to_path(&key);
                // TODO: move the repeated code into a function
                for _ in 0..height + 1 {
                    if (path >> height) & 1 == 0 { 
                        node = SparseMerkleTreeNodeBranch {
                            left: Some(Box::new(node.clone())),
                            right: None,
                            hash: smk_util::hash_branch(&node.clone().hash, &smk_util::empty_smt_hash()),
                            leaf: None,
                        };
                    } else {
                        node = SparseMerkleTreeNodeBranch {
                            left: None,
                            right: Some(Box::new(node.clone())),
                            hash: smk_util::hash_branch(&smk_util::empty_smt_hash(), &node.hash),
                            leaf: None,
                        };
                    }
                    path <<= 1;
                }
                if node.hash != *comm {
                    return None;
                }
            },
            (Some(value), SparseMerkleTreeProof::Present { leaf }) => {
                let mut node = SparseMerkleTreeNodeBranch {
                    left: None,
                    right: None,
                    hash: smk_util::empty_smt_hash(),
                    leaf: Some(leaf.clone()),
                };
                
                // TODO: move the repeated code into a function
                if node.leaf.is_some() {
                    let leaf = node.leaf.clone().unwrap();
                    if leaf.hash == smk_util::hash_leaf(&key) && leaf.key == key {
                        if leaf.value.clone().unwrap() != value {
                            return None;
                        }
                    } 
                }

                let mut path: u8 = smk_util::key_to_path(&key);
                // TODO: move the repeated code into a function
                for _ in 0..height + 1 {
                    if (path >> height) & 1 == 0 { 
                        node = SparseMerkleTreeNodeBranch {
                            left: Some(Box::new(node.clone())),
                            right: None,
                            hash: smk_util::hash_branch(&node.hash, &smk_util::empty_smt_hash()),
                            leaf: None,
                        };
                    } else {
                        node = SparseMerkleTreeNodeBranch {
                            left: None,
                            right: Some(Box::new(node.clone())),
                            hash: smk_util::hash_branch(&smk_util::empty_smt_hash(), &node.hash),
                            leaf: None,
                        };
                    }
                    path <<= 1;
                }
                if node.hash != *comm {
                    return None;
                }
            },
            _ => {
                return None;
            }
        }

        Some(())
    }

    fn get(&self, key: Self::K) -> (Option<Self::V>, Self::LookupProof) {

        let next_leaf = SparseMerkleTreeNodeLeaf {
            key: key.clone(),
            value: None,
            hash: smk_util::hash_leaf(&key), 
        };

        let mut node = self.root.clone().unwrap();
        if node.hash == smk_util::empty_smt_hash() {
            return (None, SparseMerkleTreeProof::NotPresent {
                next_leaf: next_leaf,
            });
        }

        let mut path: u8 = smk_util::key_to_path(&key);
        for _ in 0..self.height + 1 {
            if (path >> self.height) & 1 == 0 { 
                node = node.left.clone().unwrap();
            } else {
                node = node.right.clone().unwrap();
            }
            path <<= 1;
        }

        if node.leaf.is_some() {
            let leaf = node.leaf.clone().unwrap();
            if leaf.hash == smk_util::hash_leaf(&key) && leaf.key == key {
                return (Some(leaf.value.clone().unwrap()), SparseMerkleTreeProof::Present {
                    leaf: leaf,
                });
            } 
        }

        (None, SparseMerkleTreeProof::NotPresent {
            next_leaf: next_leaf,
        })
    }

    fn insert(self, _key: Self::K, _value: Self::V) -> Self {
        todo!()
    }

    fn remove(self, _key: Self::K) -> Self {
        todo!()
    }
}

// Utility functions
mod smk_util {
    use super::*;
    
    pub fn empty_smt_hash() -> Digest {
        zero_digest()
    }
    // Create the empty tree root.
    // TODO: In theory we should generate the entire tree with all of the
    // 2^256 leaf nodes, but this is not feasible. We need to add the 
    // optimization of all the internal nodes being empty and their hashes
    // being the empty hash or hash of empty hashes. For now, we will just
    // create the empty root node.
    pub fn create_root() -> Option<Box<SparseMerkleTreeNodeBranch>> {
        Some(Box::new(SparseMerkleTreeNodeBranch {
            left: None,
            right: None,
            leaf: None,
            hash: hash_branch(&empty_smt_hash(), &empty_smt_hash()),
        }))
    }

    // TODO: Make sure I can use this function in the SparseMerkleTree 
    // for leaf paths, or if I need to modify it.
    pub fn hash_leaf<T: std::convert::AsRef<[u8]>>(thing: &T) -> Digest {
        hash_one_thing("leaf_hash", &thing)
    }

    // TODO: Make sure I can use this function in the SparseMerkleTree 
    // for internal nodes and root hashes, or if I need to modify it.
    pub fn hash_branch<T: std::convert::AsRef<[u8]>>(left: &T, right: &T) -> Digest {
        hash_two_things("branch_left", "branch_right", &left, &right)
    }

    // TODO: Verify/Test this function
    pub fn key_to_path(key: &String) -> u8 {
        let digest = hash_leaf(key);
        let path = digest.as_ref();  
        let mut path_u8: u8 = 0;
        for i in 0..8 {
            path_u8 = path_u8 | path[i];
            path_u8 = path_u8 << 1;
        }
        path_u8
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(1 + 1, 2);
    }
}
