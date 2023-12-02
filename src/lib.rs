use rust_embed::RustEmbed;

pub trait Solution {
    fn solve(&self) -> String;
}

#[derive(RustEmbed)]
#[folder = "inputs/"]
pub(crate) struct Input;


pub mod ex1a;
