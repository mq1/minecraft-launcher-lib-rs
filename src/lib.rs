pub mod news;
pub mod utils;

#[macro_use]
extern crate anyhow;

const MINECRAFT_NET_URL: &str = "https://www.minecraft.net";

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::news::get_minecraft_news;

    #[test]
    fn news() -> Result<()> {
        let _ = get_minecraft_news(None)?;
        Ok(())
    }
}
