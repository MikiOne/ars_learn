use eyre::eyre;

pub fn do_div(denominator: usize, molecular: usize) -> eyre::Result<()> {
    if molecular == 0 {
        return Err(eyre!("分子不能等于0, got {:?}", molecular));
    }
    let _ = denominator / molecular;
    Ok(())
}