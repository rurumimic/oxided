use clap::ValueEnum;
use serde::{Deserialize, Deserializer, de};

#[derive(Debug, Copy, Clone, ValueEnum)]
pub enum Device {
    Cpu,
    Cuda,
}

impl<'de> Deserialize<'de> for Device {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "cpu" => Ok(Device::Cpu),
            "cuda" => Ok(Device::Cuda),
            _ => Err(de::Error::unknown_variant(&s, &["cpu", "cuda"])),
        }
    }
}
