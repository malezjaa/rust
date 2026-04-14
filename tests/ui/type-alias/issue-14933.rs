//@ check-pass
#![allow(trivial_type_alias)]

pub type BigRat<T = isize> = T;

fn main() {}
