#[cfg(feature = "debug")]
use std::fmt::Debug;
#[cfg(feature = "display")]
use std::fmt::Display;

use super::macros::none_array;

type NodeChildren<const N: usize, T> = [Option<Box<NtreeNode<N, T>>>; N];

/// Safe interface to a specific node in an n-tree.
///
/// Usage example:
/// ```
/// use tasty_ntree::Ntree;
///
/// // Making a quadtree where each node holds a String.
/// let mut i32_octree = Ntree::<4, String>::new("Root node".to_string());
///
/// // As it was passed as the default value, the root node
/// // will hold a 5 as its data.
/// let root_data = i32_octree.interface().get_data();
/// println!("Found data: {:}", root_data);
/// assert_eq!(root_data, &"Root node".to_string());
///
/// let oct_interface = i32_octree.interface_mut();
/// //oct_interface.peek_or_push_mut(0, "This is node 0".to_string());
/// //oct_interface.peek_or_push_mut(3, "And this is node 3".to_string());
/// let node_2 = oct_interface.bounded_push_mut(2, "Hello from node 2".to_string()).expect("Array bounding error");
///
/// node_2
///     .bounded_push_mut(1, "And 2.1".to_string())
///     .expect("Array bounding error")
///     .bounded_push_mut(3, "And 2.1.3".to_string());
///
/// node_2.bounded_push_mut(2, "And 2.2!".to_string());
///
/// #[cfg(feature = "debug")]
/// println!("Tree:\n{:?}", i32_octree);
///
/// // With the debug feature enabled, this would print:
/// // [NTree]
/// // 0 NtreeNode ( "Root node" )
/// // | 0 NTreeNode ( "This is node 0" )
/// // | 2 NTreeNode ( "Hello from node 2" )
/// // | | 1 NtreeNode ( "And 2.1" )
/// // | | | 3 NtreeNode ( "And 2.1.3" )
/// // | | 2 NtreeNode ( "And 2.2!" )
/// // | 3 NtreeNode ( "And this is node 3" )
/// ```
pub trait NtreeNodeInterface<const N: usize, T: Sized> {
    /// Returns a reference to the data in this node.
    fn get_data(&self) -> &T;
    /// Returns a mutable reference to the data in this node.
    fn get_data_mut(&mut self) -> &mut T;
    /// Sets data inside this node to new value and returns old value.
    fn set_data(&mut self, data: T) -> T;
    /// Returns a mutable reference to an interface to child node `i` if it exists.
    ///
    /// Panics if i is bigger than the ammount of children the node has.
    unsafe fn peek_mut(&mut self, i: usize) -> Option<&mut dyn NtreeNodeInterface<N, T>>;
    /// Returns a mutable reference to an interface to child node `i` if it exists.
    ///
    /// Does bounds checking at runtime.
    fn bounded_peek_mut(&mut self, i: usize) -> Option<&mut dyn NtreeNodeInterface<N, T>> {
        if i > N {
            return None;
        }
        unsafe { self.peek_mut(i) }
    }
    /// Returns a reference to an interface to child node `i` if it exists.
    ///
    /// Panics if i is bigger than the ammount of children the node has.
    unsafe fn peek(&self, i: usize) -> Option<&dyn NtreeNodeInterface<N, T>>;
    /// Returns a reference to an interface to child node `i` if it exists.
    ///
    /// Does bounds checking at runtime.
    fn bounded_peek(&mut self, i: usize) -> Option<&dyn NtreeNodeInterface<N, T>> {
        if i > N {
            return None;
        }
        unsafe { self.peek(i) }
    }
    /// If child node 'i' exists, overwrites it.
    ///
    /// If it doesn't, creates it and returns a mutable reference to it.
    ///
    /// If you only want to create a child if one doesn't already exist,
    /// use [peek_or_push] or [peek_or_push_mut].
    ///
    /// Panics if i is bigger than the ammount of children the node has.
    unsafe fn push_mut(&mut self, i: usize, data: T) -> &mut dyn NtreeNodeInterface<N, T>;
    /// If child node 'i' exists, overwrites it.
    ///
    /// If it doesn't, creates it and returns a mutable reference to it (as long as `i` is within bounds).
    ///
    /// Does bounds checking at runtime. If `i` is out of bounds, returns `None`.
    fn bounded_push_mut(&mut self, i: usize, data: T) -> Option<&mut dyn NtreeNodeInterface<N, T>> {
        if i > N {
            return None;
        }
        unsafe { Some(self.push_mut(i, data)) }
    }
    /// If child node 'i' exists, overwrites it.
    ///
    /// If it doesn't, creates it and returns a reference to it.
    ///
    /// If you only want to create a child if one doesn't already exist,
    /// use [peek_or_push] or [peek_or_push_mut].
    ///
    /// Panics if i is bigger than the ammount of children the node has.
    unsafe fn push(&mut self, i: usize, data: T) -> &dyn NtreeNodeInterface<N, T> {
        self.push_mut(i, data)
    }
    /// If child node 'i' exists, overwrites it.
    ///
    /// If it doesn't, creates it and returns a reference to it  (as long as `i` is within bounds).
    ///
    /// Does bounds checking at runtime. If `i` is out of bounds, returns `None`.
    fn bounded_push(&mut self, i: usize, data: T) -> Option<&dyn NtreeNodeInterface<N, T>> {
        if i > N {
            return None;
        }
        unsafe { Some(self.push(i, data)) }
    }
    /// Returns a reference to an interface to child node `i` if it exists,
    /// if not, creates it and returns it.
    ///
    /// Panics if i is bigger than the ammount of children the node has.
    unsafe fn peek_or_push(&mut self, i: usize, default_data: T) -> &dyn NtreeNodeInterface<N, T> {
        self.peek_or_push_mut(i, default_data)
    }
    /// Returns a reference to an interface to child node `i` if it exists,
    /// if not, creates it and returns it (as long as `i` is within bounds).
    ///
    /// Does bounds checking at runtime. If `i` is out of bounds, returns `None`.
    fn bounded_peek_or_push(
        &mut self,
        i: usize,
        default_data: T,
    ) -> Option<&dyn NtreeNodeInterface<N, T>> {
        if i > N {
            return None;
        }
        unsafe { Some(self.peek_or_push(i, default_data)) }
    }
    /// Returns a mutable reference to an interface to child node `i` if it exists,
    /// if not, creates it and returns it.
    ///
    /// Panics if i is bigger than the ammount of children the node has.
    unsafe fn peek_or_push_mut(
        &mut self,
        i: usize,
        default_data: T,
    ) -> &mut dyn NtreeNodeInterface<N, T> {
        if self.peek(i).is_none() {
            self.push_mut(i, default_data);
        }
        self.peek_mut(i).unwrap()
    }
    /// Returns a mutable reference to an interface to child node `i` if it exists,
    /// if not, creates it and returns it (as long as `i` is within bounds).
    ///
    /// Does bounds checking at runtime.
    fn bounded_peek_or_push_mut(
        &mut self,
        i: usize,
        default_data: T,
    ) -> Option<&mut dyn NtreeNodeInterface<N, T>> {
        if i > N {
            return None;
        }
        unsafe { Some(self.peek_or_push_mut(i, default_data)) }
    }
    /// Pops child node 'i' and returns an optional value representing inner data.
    ///
    /// Returns `Some(data)` if it existed, `None` if it didn't.
    ///
    /// Panics if i is bigger than the ammount of children the node has.
    unsafe fn pop(&mut self, i: usize) -> Option<T>;
    /// Pops child node 'i' and returns an optional value representing inner data.
    ///
    /// Returns `Some(data)` if it existed, `None` if it didn't.
    ///
    /// Does bounds checking at runtime.
    fn bounded_pop(&mut self, i: usize) -> Option<T> {
        if i > N {
            return None;
        }
        unsafe { self.pop(i) }
    }
}

#[derive(PartialEq, Eq)]
pub struct NtreeNode<const N: usize, T: Sized> {
    pub data: T,
    pub children: NodeChildren<N, T>,
}

impl<const N: usize, T: Sized> NtreeNode<N, T> {
    pub fn new(data: T) -> Self {
        let children = none_array!(N, Box<NtreeNode<N, T>>);

        Self { data, children }
    }

    fn new_node(&mut self, i: usize, data: T) {
        self.children[i] = Some(Box::new(NtreeNode::new(data)));
    }

    fn take_data(self) -> T {
        self.data
    }
}

////////////////////////// Trait impl //////////////////////////

impl<const N: usize, T: Sized> NtreeNodeInterface<N, T> for NtreeNode<N, T> {
    fn get_data(&self) -> &T {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    fn set_data(&mut self, data: T) -> T {
        std::mem::replace(&mut self.data, data)
    }

    unsafe fn peek_mut(&mut self, i: usize) -> Option<&mut dyn NtreeNodeInterface<N, T>> {
        let node_opt = &mut self.children[i];
        match node_opt {
            Some(node) => Some(node.as_mut()),
            None => None,
        }
    }

    unsafe fn peek(&self, i: usize) -> Option<&dyn NtreeNodeInterface<N, T>> {
        let node_opt = &self.children[i];
        match node_opt {
            Some(node) => Some(node.as_ref()),
            None => None,
        }
    }

    unsafe fn pop(&mut self, i: usize) -> Option<T> {
        match self.children[i].take() {
            Some(node) => Some(node.take_data()),
            None => None,
        }
    }

    unsafe fn push_mut(&mut self, i: usize, data: T) -> &mut dyn NtreeNodeInterface<N, T> {
        self.children[i] = Some(Box::from(Self::new(data)));
        self.peek_mut(i).unwrap()
    }
}

//////////////////////////// Debug /////////////////////////////

#[cfg(feature = "debug")]
impl<const N: usize, T: Sized + Debug> NtreeNode<N, T> {
    pub fn dbg_indent(
        &self,
        index: usize,
        indentation: u32,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str("\n")?;
        for _ in 0..indentation {
            f.write_str(" |")?;
        }
        f.write_fmt(format_args!(" {} NtreeNode ( {:?} )", index, self.data))?;
        for i in 0..N {
            match &self.children[i] {
                Some(child) => child.dbg_indent(i, indentation + 1, f)?,
                None => (),
            }
        }
        Ok(())
    }
}

#[cfg(feature = "debug")]
impl<const N: usize, T: Sized + Debug> Debug for NtreeNode<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.dbg_indent(0, 0, f)
    }
}

/////////////////////////// Display ////////////////////////////

#[cfg(feature = "display")]
impl<const N: usize, T: Sized> NtreeNode<N, T> {
    pub fn fmt_indent(
        &self,
        index: usize,
        indentation: u32,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        f.write_str("\n")?;
        for _ in 0..indentation {
            f.write_str(" |")?;
        }
        f.write_fmt(format_args!(" {} NtreeNode", index))?;
        for i in 0..N {
            match &self.children[i] {
                Some(child) => child.fmt_indent(i, indentation + 1, f)?,
                None => (),
            }
        }
        Ok(())
    }
}

#[cfg(feature = "display")]
impl<const N: usize, T: Sized> Display for NtreeNode<N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_indent(0, 0, f)
    }
}
