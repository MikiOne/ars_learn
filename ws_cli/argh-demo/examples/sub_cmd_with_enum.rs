// Copyright (c) 2022 Google LLC All rights reserved.
// Use of this source code is governed by a BSD-style
// license that can be found in the LICENSE file.

use {argh::FromArgs, std::fmt::Debug};

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command.
struct TopLevel {
    #[argh(subcommand)]
    nested: MySubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum MySubCommandEnum {
    One(SubCommandOne),
    Two(SubCommandTwo),
}

#[derive(FromArgs, PartialEq, Debug)]
/// First subcommand.
#[argh(subcommand, name = "one")]
struct SubCommandOne {
    #[argh(option)]
    /// how many x
    x: usize,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Second subcommand.
#[argh(subcommand, name = "two")]
struct SubCommandTwo {
    #[argh(switch)]
    /// whether to fooey
    fooey: bool,
}

/// - cargo run --example sub_cmd_with_enum -- one --x 33
/// ```
/// TopLevel {
///     nested: One(
///         SubCommandOne {
///             x: 33,
///         },
///     ),
/// }
/// ```
/// - cargo run --example sub_cmd_with_enum -- two
/// 
/// - cargo run --example sub_cmd_with_enum -- two --fooey
fn main() {
    let toplevel: TopLevel = argh::from_env();
    println!("{:#?}", toplevel);
}