use crate::database::Database;

mod database;

//
// 获取当前的工作目录ø
// let current_dir: PathBuf = env::current_dir()?;
// 打印当前的工作目录
// println!("当前的工作目录: {:?}", current_dir);
fn main() -> std::io::Result<()> {
    let db = Database::new("./ws-middleware/rs-rocksdb-a/demo_rocks_db");
    db.put("test_key", "test_value modify");

    for _i in 0..2 {
        let val = db.get("test_key");
        println!("value: {val:?}");

        db.delete("test_key");
    }

    db.delete("test_none_key");

    Ok(())
}