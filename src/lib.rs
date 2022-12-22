use once_cell::sync::Lazy;

use cli::ConfigArgs;

pub mod cli;
pub mod yso;

pub static EMO_ARGS: Lazy<ConfigArgs> = Lazy::new(|| -> ConfigArgs { ConfigArgs::new() });
