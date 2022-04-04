pub mod news;
pub mod version_manifest;

const MINECRAFT_NET_URL: &str = "https://www.minecraft.net";

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use crate::{version_manifest::get_version_manifest, news::get_minecraft_news};

    #[test]
    fn version_manifest() -> Result<()> {
        let _ = get_version_manifest()?;
        Ok(())
    }

    #[test]
    fn news() -> Result<()> {
        let _ = get_minecraft_news(None)?;
        Ok(())
    }
}
