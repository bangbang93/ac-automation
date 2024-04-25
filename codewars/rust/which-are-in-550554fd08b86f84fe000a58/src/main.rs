use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

fn in_array(arr_a: &[&str], arr_b: &[&str]) -> Vec<String> {
    let mut result = HashSet::new();

    for a in arr_a {
        for b in arr_b {
            if b.contains(a) {
                result.insert(a.to_string());
                break;
            }
        }
    }

    let mut arr = result.into_iter().collect::<Vec<String>>();
    arr.sort();
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(
            in_array(
                &["xyz", "live", "strong"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["live", "strong"]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );

        assert_eq!(
            in_array(
                &["tarp", "mice", "bull"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            [] as [&str; 0]
        );

        assert_eq!(
            in_array(
                &["live", "strong", "arp", "arp"],
                &["lively", "alive", "harp", "sharp", "armstrong"],
            ),
            ["arp", "live", "strong"]
        );
    }
}
