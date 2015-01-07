#[cfg(test)]
extern crate test;

pub use blake2b::Blake2b;
pub use blake2s::Blake2s;

mod blake2b;
mod blake2s;
#[cfg(test)] mod kat;
