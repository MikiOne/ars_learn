use tokio::io::AsyncBufReadExt;

use std::io::Cursor;

#[tokio::main]
async fn main() {
    let mut cursor = Cursor::new(b"foo\nbar");
    let mut buf = String::new();

    // cursor is at 'f'
    let num_bytes = cursor.read_line(&mut buf).await.expect("reading from cursor won't fail");

    assert_eq!(num_bytes, 4);
    assert_eq!(buf, "foo\n");
    buf.clear();

    // cursor is at 'b'
    let num_bytes = cursor.read_line(&mut buf).await.expect("reading from cursor won't fail");

    assert_eq!(num_bytes, 3);
    assert_eq!(buf, "bar");
    buf.clear();

    // cursor is at EOF
    let num_bytes = cursor.read_line(&mut buf).await.expect("reading from cursor won't fail");

    assert_eq!(num_bytes, 0);
    assert_eq!(buf, "");
}
