use std::collections::HashMap;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut hashmap = HashMap::new();
    for i in 0..10 {
        hashmap.insert(('A' as u8 + i) as char, i);
    }
    let mut w = csv::Writer::from_writer(vec![]);

    w.serialize(hashmap).unwrap();
    w.flush().unwrap();
    let s = String::from_utf8(w.into_inner().unwrap()).unwrap();
    println!("{s}");
}
