pub trait Solution {
    fn solve(&self) -> &str;
}

pub struct Ex1A {
}

impl Ex1A {
    pub fn new() -> Self {
        Self {}
    }
}

impl Solution for Ex1A {
    fn solve(&self) -> &str {
        ":-)"
    }
}

