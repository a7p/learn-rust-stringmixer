extern crate rand;
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use rand::{Rng, thread_rng};

#[test]
fn test_same() {
    assert!(mix_string("a") == "a");
    // assert!(mix_string("tt") == "tt");
    // assert!(mix_string("tet") == "tet");
    // assert!(mix_string("teet") == "teet");
    // let s: &'static str = "blaaa";
    // let mut g = s.graphemes(true).collect::<Vec<&str>>();
    // let () = g;
}


pub fn mix_string(s: &str) -> String {
    let mut g = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    {
        let l = g.len() -1;
        let mut v = &mut g[1..l];
        let mut rng = thread_rng();
        rng.shuffle(v);
    }
    g.iter().map(|x| *x).collect::<String>()
}
// pub fn remove_and_add_first_and_last(s: &str) -> String {
    // mix_string(s[1])
// }

fn main() {
    println!("{}", mix_string("graphemes"));
    // let h = "hallo";
    // let h2 = h.chars().next();
    // match h2 {
        // Some(ref h2) => println!("{}", h2),
        // None => "faaail",
    // }§
}
