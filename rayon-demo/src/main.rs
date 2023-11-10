fn main() -> reqwest::Result<()> {
    //     let ranges = &[
    //         (0, 249MB),
    //         (250MB, 499MB),
    //         (500MB, 749MB),
    //         (750MB, 999MB),
    //         (1000MB, 1249MB),
    //         (1250MB, 1499MB),
    //         (1500MB, 1749MB),
    //         (1750MB, 2000MB),
    //     ];
    //     ranges.iter().for_each(|(start, end)| {
    //         let client = reqwest::blocking::Client::new();
    //         let response = client
    //             .get("random_video_url.mp4")
    //             .header(RANGE, RangeHeader((start, end)))
    //             .send()
    //         std::fs::write(format!("file{}", a), response.bytes().unwrap())
    //     });
    //     // finally, join all files

    let names = vec!["egal"];
    // let is_present = names.par_iter().any(|name| name == "Alice");

    Ok(())
}

use rayon::prelude::*;

// sequential
fn sum_of_char_len(list: &[&str]) -> usize {
    list.iter().map(|word| word.len()).sum()
}

// parallel
//             .unwrap();
fn par_sum_of_char_len(list: &[&str]) -> usize {
    list.par_iter().map(|word| word.len()).sum()
}
