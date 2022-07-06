pub mod instances;
pub mod news;
pub mod version_manifest;
pub mod version_meta;

const MINECRAFT_NET_URL: &str = "https://www.minecraft.net";

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use url::Url;

    use crate::{
        news::get_minecraft_news, version_manifest::get_version_manifest,
        version_meta::get_version_meta,
    };

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

    #[test]
    fn version_meta() -> Result<()> {
        let version_manifest = get_version_manifest()?;
        let _ = get_version_meta(&Url::parse(&version_manifest.versions[0].url)?)?;
        Ok(())
    }
}
