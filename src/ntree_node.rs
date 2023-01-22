use std::{fmt::Debug, mem::MaybeUninit};

use crate::macros::new_array;

type NodeChildren<const N: usize, T> = [Option<Box<NtreeNode<N, T>>>;N];

pub trait NtreeNodeInterface<T: Sized + Debug>
{
    fn get_data(&self) -> &T;
    fn get_data_mut(&mut self) -> &mut T;
    fn peek_mut(&mut self, i:usize) -> Option<&mut dyn NtreeNodeInterface<T>>;
    fn insert_mut(&mut self, i: usize, default_data: T) -> &mut dyn NtreeNodeInterface<T>;
    fn peek(&self, i: usize) -> Option<&dyn NtreeNodeInterface<T>>;
    fn insert(&mut self, i: usize, default_data: T) -> &dyn NtreeNodeInterface<T>;
}


#[derive(PartialEq, Eq)]
pub struct NtreeNode<const N: usize, T: Sized + Debug> {
    pub data: T,
    pub children: NodeChildren<N, T>,
}

impl<const N: usize, T: Sized + Debug> NtreeNode<N, T>{

    pub fn new(data: T) -> Self
    {
        let children = new_array!(N, Option<Box<NtreeNode<N, T>>>, None);

        Self {
            data,
            children
        }
        // unsafe {
        //     let mut children: NodeChildren<N, T> = MaybeUninit::uninit().assume_init();

        //     if N == 0 {
        //         return Self {
        //             data,
        //             children
        //         }
        //     }

        //     let first_child = std::ptr::addr_of_mut!(*children.get_mut(0).unwrap());

        //     for i in 0..N {
        //         std::ptr::write(first_child.add(i), None)
        //     }

        //     Self {
        //         data,
        //         children,
        //     }
        // }
    }

    fn new_node(&mut self, i: usize, data: T)
    {
        self.children[i] = Some(Box::new(NtreeNode::new(data)));
    }

    pub fn fmt_indent(&self, index: usize, indentation: u32, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("\n")?;
        for _ in 0..indentation { f.write_str(" |")?; };
        f.write_fmt(format_args!(" {} NtreeNode ( {:?} )", index, self.data))?;
        for i in 0..8 {
            let child_opt = &self.children[i];
            match child_opt {
                Some(child) => child.fmt_indent(i, indentation + 1, f)?,
                None => ()
            }
        }
        Ok(())
    }
}

impl<const N: usize, T: Sized + Debug> NtreeNodeInterface<T> for NtreeNode<N, T>
{
    fn get_data(&self) -> &T {
        &self.data
    }

    fn get_data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    fn peek_mut(&mut self, i:usize) -> Option<&mut dyn NtreeNodeInterface<T>> {
        let node_opt = &mut self.children[i];
        match node_opt {
            Some(node) => Some(node.as_mut()),
            None => None
        }
    }

    fn insert_mut(&mut self, i: usize, default_data: T) -> &mut dyn NtreeNodeInterface<T> {
        if self.children[i].is_none()
        {
            self.new_node(i, default_data);
        }
        self.peek_mut(i).unwrap()
    }

    fn peek(&self, i:usize) -> Option<&dyn NtreeNodeInterface<T>> {
        let node_opt = &self.children[i];
        match node_opt {
            Some(node) => Some(node.as_ref()),
            None => None
        }
    }

    fn insert(&mut self, i: usize, default_data: T) -> &dyn NtreeNodeInterface<T> {
        if self.children[i].is_none()
        {
            self.new_node(i, default_data);
        }
        self.peek(i).unwrap()
    }
}