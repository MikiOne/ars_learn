use argh::FromArgs;

fn default_height() -> usize {
    5
}

#[derive(FromArgs, Debug)]
/// Reach new heights.
struct GoUp {
    /// an optional nickname for the pilot
    #[argh(option)]
    pilot_nickname: Option<String>,

    /// an optional height
    #[argh(option, default = "default_height()")]
    height: usize,

    /// an optional direction which is "up" by default
    #[argh(option, default = "String::from(\"only up\")")]
    direction: String,
}

/// - cargo run --example with_default_value -- --direction hello
/// GoUp { pilot_nickname: None, height: 5, direction: "hello" }
fn main() {
    let up: GoUp = argh::from_env();
    println!("{:?}", up);
}