use rust_concepts::utils::{weave, weave_iter, IteratorExt};

fn main() {
    let v1: Vec<_> = (0..16).map(|v| (v * 5) as u8).collect();
    let v2 = [0x80u8, 0x84, 0x89];
    let v3: Vec<_> = (0..9).map(|v| 0xB0 + v as u8).rev().collect();
    // utils::say_hello();

    let s1: Vec<String> = ["hello", "general", "!¿"]
        .map(|s| String::from(s))
        .into_iter()
        .collect();
    let s2: Vec<_> = ["there", "Kenobi"]
        .map(|s| String::from(s))
        .into_iter()
        .collect();

    let coll = [&v1[..], &v2, &v3];
    for v in weave(&coll[..]) {
        print!("{v:02x} ");
    }

    print!("\nweave_iter\n");

    fn to_str<'a,'b>(s: &'a String) -> &'b str 
    where 'a: 'b {

        s.as_str()
    }
    let s1_iter = s1.iter().map(to_str);
    let s2_iter = s2.iter().map(to_str);
    let s1_s2 = [s1_iter, s2_iter].into_iter();
    for v in weave_iter( s1_s2) {
        print!("{v} ");
    }
    print!("\nweave");
    for v in weave(&[&s1, &s2]) {
        print!("{v} ");
    }
    println!("\nInterleave with iterators\n");
    for v in v1.iter().interleave(&v2).interleave(&v3) {
        print!("{v:02x} ");
    }

    println!("\nWeave with iterators\n");
    for v in v1.iter().weave(&v2).weave(&v3) {
        print!("{v:02x} ");
    }
}
