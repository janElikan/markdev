use std::{path::PathBuf, str::FromStr};

use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    let vault = PathBuf::from_str("/home/elikan/vault")?;

    markdev::run(vault);

    Ok(())
}
