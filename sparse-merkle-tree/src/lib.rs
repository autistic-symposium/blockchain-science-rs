pub mod common;
pub mod kv_trait;
pub mod sorted_kv;
pub mod sparse_merkle_tree;
#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
