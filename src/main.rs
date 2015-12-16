extern crate rand;
extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;
use rand::{Rng, thread_rng};

#[test]
fn test_same() {
    assert!(mix_string("a") == "a");
    assert!(mix_string("tt") == "tt");
    assert!(mix_string("tet") == "tet");
    assert!(mix_string("teet") == "teet");
}


pub fn mix_string(s: &str) -> String {    let mut g = UnicodeSegmentation::graphemes(s, true).collect::<Vec<&str>>();
    if g.len() > 3
    {
        let l = g.len() -1;
        let mut v = &mut g[1..l];
        let mut rng = thread_rng();
        rng.shuffle(v);
    }
    g.iter().map(|x| *x).collect::<String>()
}


fn main() {
    println!("{}", mix_string("graphemes"));
}
