//use std::collections::HashMap;

use std::fmt::Display;
use crate::defs::{REVERSE, BOLD, RESET};

pub struct Media<M> {
    title: M,
    url: M,
    // TODO: author: M,
    //                  hh  mm  ss
    // TODO: duration: (u8, u8, u8),
}

impl<X: Clone> Media<X> {
    pub fn new(title: X, url: X) -> Self {
        Self { title, url }
    }
    pub fn title(&self) -> X {
        self.title.clone()
    }
    pub fn url(&self) -> X {
        self.url.clone()
    }
}

impl<X: Display> Display for Media<X> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, " {REVERSE}{BOLD} {} {RESET} {}", self.title, self.url )
    }
}
