use std::hash::Hash;

pub trait Key : Hash + Eq + PartialEq + PartialOrd + Ord {}
