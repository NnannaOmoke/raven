#![allow(dead_code)]
///This contains the basic structs and methods and means that they interact with each other
pub mod base;
///This is the application code for the raven CLI
pub mod core;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
