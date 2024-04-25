fn main() {
    println!("Hello, world!");
}
fn abbrev_name(name: &str) -> String {
    let names = name.split(" ");
    let chars = names.map(|s| s.chars().next().unwrap().to_uppercase().to_string());
    chars.collect::<Vec<String>>().join(".")
}

// Rust test example:
#[test]
fn sample_tests() {
    assert_eq!(abbrev_name("Sam Harris"), "S.H");
    assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
    assert_eq!(abbrev_name("Evan Cole"), "E.C");
    assert_eq!(abbrev_name("P Favuzzi"), "P.F");
    assert_eq!(abbrev_name("David Mendieta"), "D.M");
}