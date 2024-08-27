use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Reach new heights.
struct GoUp {
    /// whether or not to jump
    #[argh(switch, short = 'j')]
    jump: bool,

    /// how high to go
    #[argh(option)]
    height: usize,

    /// an optional nickname for the pilot
    #[argh(option)]
    pilot_nickname: Option<String>,
}


/// - cargo run --example first_from_args -- --help
///
/// - cargo run --example first_from_args -- -j --height 333
///
/// GoUp { jump: true, height: 333, pilot_nickname: None }
///
/// - cargo run --example first_from_args -- --height 333
///
/// GoUp { jump: false, height: 333, pilot_nickname: None }
fn main() {
    let up: GoUp = argh::from_env();
    println!("{:?}", up);
}