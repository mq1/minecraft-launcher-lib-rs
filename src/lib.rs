pub mod utils;

#[macro_use]
extern crate anyhow;

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::utils::get_minecraft_news;

    #[test]
    fn news() -> Result<()> {
        get_minecraft_news(None)?;
        Ok(())
    }
}
