use crate::math::div::do_div;

mod div;

pub fn div(denominator: usize, molecular: usize) -> eyre::Result<()> {
    do_div(denominator, molecular)?;
    Ok(())
}

