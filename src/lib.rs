//@ This library shows tested implementations of common data structures and algorithms
//@ ## Graphs
//@ - [Kosaraju's Algorithm](./graphs/kosaraju.html)
//@ - [Topological Sort](./graphs/rooted_topological_sort.html)
#![feature(test)]
#![allow(unused)]

use peak_alloc::PeakAlloc;

#[global_allocator]
static PEAK_ALLOC: PeakAlloc = PeakAlloc;

#[cfg(test)]
extern crate test;

pub mod arrays;
pub mod distances;
pub mod graphs;
pub mod images;
pub mod primitive;
pub mod sorts;
pub mod strings;
pub mod trees;
