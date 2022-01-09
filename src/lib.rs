pub mod accounts;
pub mod assets;
pub mod config;
pub mod instances;
pub mod launchermeta;
pub mod libraries;
pub mod util;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
