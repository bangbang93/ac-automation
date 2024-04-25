fn are_you_playing_banjo(name: &str) -> String {
    if name.to_lowercase().chars().next().unwrap() == 'r' {
        format!("{} plays banjo", name)
    } else {
        format!("{} does not play banjo", name)
    }
}

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_you_playing_banjo() {
        assert_eq!(are_you_playing_banjo("Martin"), "Martin does not play banjo");
        assert_eq!(are_you_playing_banjo("Rikke"), "Rikke plays banjo");
        assert_eq!(are_you_playing_banjo("bravo"), "bravo does not play banjo");
        assert_eq!(are_you_playing_banjo("rolf"), "rolf plays banjo");
    }
}