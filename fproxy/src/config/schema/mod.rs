mod parser;

use serde::Deserialize;

/// App name slug.
pub type App = String;

/// App port
pub type Port = u16;

/// Address for a target.
#[derive(Debug)]
pub struct TargetAddr {
    /// Target address.
    pub addr: String,
    /// Target port
    pub port: Port,
}

#[derive(Debug, Deserialize)]
pub struct Apps {
    #[serde(rename = "Apps")]
    pub apps: Vec<AppConfig>,
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    #[serde(rename = "Name")]
    pub name: App,

    #[serde(rename = "Ports")]
    pub ports: Vec<Port>,

    #[serde(rename = "Targets")]
    pub targets: Vec<TargetAddr>,
}
