pub fn load_file() -> eyre::Result<()> {
    let file_path = "non_existent_file.txt";
    let content = std::fs::read_to_string(file_path)?;
    println!("{}", content);
    Ok(())
}