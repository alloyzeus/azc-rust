//

#[macro_use]
extern crate mopa;

pub mod azml;
pub mod azcore;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
