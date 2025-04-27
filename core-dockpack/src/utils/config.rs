use semver::{BuildMetadata, Prerelease, Version};
use serde::{Deserialize, Serialize};
use serde_yaml;
use std::path::Path;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Config {
    version: Version,
    directory: String,
    bucket_name: String,
    docker_images: Vec<String>,
}

// TODO: implement remote derive and serialize for Version https://serde.rs/remote-derive.html

// This can work, but I don't really want to spec the derive like this. It should be with
// parse() https://docs.rs/semver/latest/semver/struct.Version.html#method.parse
// maybe do an initial ConfigFile -> Config?
struct VersionDef {
    pub major: u64,
    pub minor: u64,
    pub patch: u64,
    pub pre: Prerelease,
    pub build: BuildMetadata,
}

impl TryFrom<String> for Config {
    type Error = String;
    fn try_from(str: String) -> Result<Self, Self::Error> {
        let f = std::fs::File::open(str)?;
        let d: Config = serde_yaml::from_reader(f)?;
    }
}
