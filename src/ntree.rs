#![allow(dead_code)]

#[cfg(feature = "debug")]
use std::fmt::Debug;
#[cfg(feature = "display")]
use std::fmt::Display;

use crate::ntree_node::*;

mod test {
    #[allow(unused_imports)]
    use super::*;
    #[allow(unused_imports)]
    use crate::macros::none_array;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct NtreeData {
        pub tile_type: i32,
        pub depth: i32,
    }

    #[test]
    fn make_ntree() {
        let def_data = NtreeData {
            tile_type: 0,
            depth: 8,
        };
        Ntree::<8, _>::new(def_data);
    }

    #[test]
    fn empty_ntree() {
        let def_data = NtreeData {
            tile_type: 0,
            depth: 8,
        };
        Ntree::<0, _>::new(def_data);
    }

    #[test]
    fn insert_ntree() {
        let def_data = NtreeData {
            tile_type: 0,
            depth: 0,
        };

        let mut ntree = Ntree::<8, _>::new(def_data);

        ntree.interface().insert(4, def_data);
        ntree
            .interface()
            .insert_mut(6, def_data)
            .insert(3, def_data);

        let correct_ntree = Ntree {
            root: NtreeNode {
                data: def_data,
                children: [
                    None,
                    None,
                    None,
                    None,
                    Some(Box::from(NtreeNode {
                        data: def_data,
                        children: none_array!(8, Box<NtreeNode<8, NtreeData>>),
                    })),
                    None,
                    Some(Box::from(NtreeNode {
                        data: def_data,
                        children: [
                            None,
                            None,
                            None,
                            Some(Box::from(NtreeNode {
                                data: def_data,
                                children: none_array!(8, Box<NtreeNode<8, NtreeData>>),
                            })),
                            None,
                            None,
                            None,
                            None,
                        ],
                    })),
                    None,
                ],
            },
        };

        #[cfg(not(feature = "debug"))]
        assert!(ntree == correct_ntree, "Trees are not equal");
        #[cfg(feature = "debug")]
        assert_eq!(ntree, correct_ntree);
    }

    /// Run with cargo test -- --nocapture
    #[cfg(feature = "display")]
    #[test]
    fn display_ntree() {
        #[derive(Clone, Copy, PartialEq, Eq)]
        struct NtreeDataNoDebug {
            pub tile_type: i32,
            pub depth: i32,
        }

        let def_data = NtreeDataNoDebug {
            tile_type: 0,
            depth: 0,
        };

        let mut ntree = Ntree::<8, _>::new(def_data);

        ntree.interface().insert(4, def_data);
        ntree
            .interface()
            .insert_mut(6, def_data)
            .insert(3, def_data);

        println!("Display output:");
        println!("{:}", ntree);
        println!("");
    }

    /// Run with cargo test -- --nocapture
    #[cfg(feature = "debug")]
    #[test]
    fn debug_ntree() {
        let def_data = NtreeData {
            tile_type: 0,
            depth: 0,
        };

        let mut ntree = Ntree::<8, _>::new(def_data);

        ntree.interface().insert(4, def_data);
        ntree
            .interface()
            .insert_mut(6, def_data)
            .insert(3, def_data);

        println!("Debug output:");
        println!("{:?}", ntree);
        println!("");
    }
}

/// Safe interface for NtreeNodes
#[derive(PartialEq, Eq)]
pub struct Ntree<const N: usize, T: Sized> {
    root: NtreeNode<N, T>,
}

impl<const N: usize, T: Sized> Ntree<N, T> {
    pub fn new(default_data: T) -> Self {
        let root;

        root = NtreeNode::new(default_data);

        Self { root }
    }

    pub fn interface(&mut self) -> &mut dyn NtreeNodeInterface<T> {
        &mut self.root
    }
}

#[cfg(feature = "debug")]
impl<const N: usize, T: Sized + Debug> Debug for Ntree<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[Ntree]")?;
        Debug::fmt(&self.root, f)
    }
}

#[cfg(feature = "display")]
impl<const N: usize, T: Sized> Display for Ntree<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("[Ntree]")?;
        Display::fmt(&self.root, f)
    }
}
