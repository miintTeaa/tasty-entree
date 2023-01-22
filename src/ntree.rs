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
    fn push_ntree() {
        let def_data = NtreeData {
            tile_type: 0,
            depth: 0,
        };

        let mut ntree = Ntree::<8, _>::new(def_data);

        ntree.interface_mut().bounded_push(4, def_data);
        ntree
            .interface_mut()
            .bounded_push_mut(6, def_data)
            .expect("Array bounding not working properly")
            .bounded_push(3, def_data);

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

    #[test]
    fn pop_ntree() {
        let data1 = NtreeData {
            tile_type: 4,
            depth: 8,
        };

        let data2 = NtreeData {
            tile_type: -3,
            depth: 5,
        };

        let mut ntree = Ntree::<8, _>::new(data1);

        ntree
            .interface_mut()
            .bounded_push_mut(3, data1)
            .expect("Array bounding not working properly")
            .bounded_push_mut(5, data2);

        let out = ntree
            .interface_mut()
            .bounded_peek_mut(3)
            .expect("Array bounding not working properly")
            .bounded_pop(5);

        assert_eq!(out, Some(data2));
    }

    #[test]
    fn set_data_ntree() {
        let def_data = NtreeData {
            tile_type: -5,
            depth: 8,
        };

        let replaced_data = NtreeData {
            tile_type: 1,
            depth: 2,
        };

        let mut ntree = Ntree::<8, _>::new(def_data);

        let old_data = ntree.interface_mut().set_data(replaced_data);

        assert_eq!(old_data, def_data);

        let new_data = ntree.interface_mut().get_data();

        assert_eq!(new_data, &replaced_data);
    }

    #[test]
    fn send_test() {
        use std::thread;

        let mut tree = Ntree::<4, i32>::new(3);

        let thread = thread::spawn(move || {
            tree.interface_mut().bounded_push(0, 5);
        });

        thread.join().unwrap();
    }

    #[test]
    fn sync_test() {
        use std::sync::Arc;
        use std::thread;

        let tree = Arc::from(Ntree::<3, _>::new(5));

        let tree_ref = tree.clone();
        let tree_ref2 = tree.clone();

        let thread = thread::spawn(move || {
            tree_ref.interface();
        });

        let thread2 = thread::spawn(move || {
            tree_ref2.interface();
        });

        thread.join().unwrap();
        thread2.join().unwrap();
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

        ntree.interface_mut().bounded_push(4, def_data);
        ntree
            .interface_mut()
            .bounded_push_mut(6, def_data)
            .expect("Array bounding not working properly")
            .bounded_push(3, def_data);

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

        ntree.interface_mut().bounded_push(4, def_data);
        ntree
            .interface_mut()
            .bounded_push_mut(6, def_data)
            .expect("Array bounding not working properly")
            .bounded_push(3, def_data);

        println!("Debug output:");
        println!("{:?}", ntree);
        println!("");
    }
}

/// Safe n-tree interface.
///
/// You can create one with [Ntree::<size, Type>::new(default_value)][Self::new()],
///
/// and you can interact with it using [Self::interface()] and [Self::interface_mut()].
///
/// ```
/// use tasty_ntree::Ntree;
///
/// // Making an octree where each node holds an i32.
/// let i32_octree = Ntree::<8, i32>::new(5);
///
/// // As it was passed as the default value, the root node
/// // will hold a 5 as its data.
/// let root_data = i32_octree.interface().get_data();
/// println!("Found data: {:}", root_data);
/// assert_eq!(root_data, &5);
/// ```
///
///
/// For more info on what you can do with interfaces check [NtreeNodeInterface].
#[derive(PartialEq, Eq)]
pub struct Ntree<const N: usize, T: Sized> {
    root: NtreeNode<N, T>,
}

impl<const N: usize, T: Sized> Ntree<N, T> {
    /// Creates a new [Ntree], with the root node's data set to `default_data`.
    pub fn new(default_data: T) -> Self {
        let root;

        root = NtreeNode::new(default_data);

        Self { root }
    }

    /// Returns an **immutable** reference to an [interface][NtreeNodeInterface] to the root node.
    pub fn interface(&self) -> &dyn NtreeNodeInterface<N, T> {
        &self.root
    }

    /// Returns a **mutable** reference to an [interface][NtreeNodeInterface] to the root node.
    pub fn interface_mut(&mut self) -> &mut dyn NtreeNodeInterface<N, T> {
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
