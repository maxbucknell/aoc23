use rust_embed::RustEmbed;

pub trait Solution {
    fn solve(&self) -> String;
}

#[derive(RustEmbed)]
#[folder = "inputs/"]
pub(crate) struct Input;


pub mod ex1a;
pub mod ex1b;
pub mod ex2;
pub mod ex2a;
pub mod ex2b;
