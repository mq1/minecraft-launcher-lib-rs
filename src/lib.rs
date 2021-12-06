pub mod assets;
pub mod instances;
pub mod launchermeta;
pub mod config;
pub mod util;
pub mod accounts;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
