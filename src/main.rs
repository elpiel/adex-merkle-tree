fn main () {
    use example::ExampleAlgorithm;
    use merkletree::merkle::{MerkleTree,VecStore};
    use std::iter::FromIterator;

    let mut h1 = [0u8; 32];
    let mut h2 = [0u8; 32];
    let mut h3 = [0u8; 32];
    h1[0] = 0x11;
    h2[0] = 0x22;
    h3[0] = 0x33;

    let t: MerkleTree<[u8; 32], ExampleAlgorithm, VecStore<_>> = MerkleTree::from_iter(vec![h1, h2, h3]);
    println!("{:?}", t.root());
}

mod example {
    use std::hash::Hasher;
    use sha3::{Digest, Keccak256};
    use merkletree::hash::Algorithm;

    pub struct ExampleAlgorithm(Keccak256);

    impl ExampleAlgorithm {
        pub fn new() -> ExampleAlgorithm {
            ExampleAlgorithm(Keccak256::new())
        }
    }

    impl Default for ExampleAlgorithm {
        fn default() -> ExampleAlgorithm {
            ExampleAlgorithm::new()
        }
    }

    impl Hasher for ExampleAlgorithm {
        #[inline]
        fn finish(&self) -> u64 {
            unimplemented!()
        }

        #[inline]
        fn write(&mut self, msg: &[u8]) {
            self.0.input(msg)
        }
    }

    impl Algorithm<[u8; 32]> for ExampleAlgorithm {
        #[inline]
        fn hash(&mut self) -> [u8; 32] {
            let mut h = [0u8; 32];
            let hash_result = self.0.clone().result();
            h.copy_from_slice(&hash_result[..32]);
            h
        }

        #[inline]
        fn reset(&mut self) {
            self.0.reset();
        }
    }
}