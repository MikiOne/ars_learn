mod filem;
mod math;

use eyre::Result;
use crate::math::div;

fn main() -> Result<()> {
    div(10, 0)?;
    filem::load_file()?;
    Ok(())
}

