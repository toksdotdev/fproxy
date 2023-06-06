use crate::config::schema::TargetAddr;
use serde::de::Error;
use serde::Deserialize;
use serde::Deserializer;
use thiserror::Error;

#[derive(Error, Debug)]
enum ParseTargetError {
    #[error("invalid address format (expected {{address}}:{{port}})")]
    InvalidFormat,
    #[error("invalid port number")]
    InvalidPort,
}

impl<'de> Deserialize<'de> for TargetAddr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: &str = Deserialize::deserialize(deserializer)?;
        let (addr, port) = value
            .rsplit_once(':')
            .ok_or_else(|| Error::custom(ParseTargetError::InvalidFormat))?;

        let port = port
            .parse::<u16>()
            .map_err(|_| Error::custom(ParseTargetError::InvalidPort))?;

        Ok(TargetAddr {
            addr: addr.to_owned(),
            port,
        })
    }
}
