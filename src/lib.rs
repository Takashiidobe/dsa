//@ This library shows tested implementations of common data structures and algorithms
//@ ## Primitive
//@ - [Parity of Bits](./primitive/parity.html)
//@ ## Arrays
//@ - [Shuffling](./arrays/shuffle.html)
//@ - [Sampling](./arrays/sample.html)
//@ ## Strings
//@ - [Copy on Write](./strings/copy_on_write.html)
//@ ## Graphs
//@ - [Kosaraju's Algorithm](./graphs/kosaraju.html)
//@ - [Topological Sort](./graphs/rooted_topological_sort.html)
//@ ## Trees
//@ - [Fenwick Trees](./trees/fenwick.html)
//@ - [BK Trees](./trees/bk_tree.html)
#![feature(test)]
#![allow(unused)]

use peak_alloc::PeakAlloc;

#[cfg_attr(test, global_allocator)]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

#[cfg(test)]
extern crate test;

pub mod arrays;
pub mod basic;
pub mod distances;
pub mod graphs;
pub mod images;
pub mod primitive;
pub mod sorts;
pub mod strings;
pub mod trees;
