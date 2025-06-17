//! Week-01/Day-02 solution.

/// Returns the index of the word "Nemo" in the words of the input string, or `None` if "Nemo" is not found.
pub fn get_index(inp: &str) -> Option<usize> {
    inp.split_whitespace().position(|x| x == "Nemo")
}

/// Returns a message with the position of "Nemo" if it's found.
pub fn index_to_string(index: Option<usize>) -> String {
    index.map_or(
        // choose
        "I can't find Nemo :(".to_string(),
        |i| "I found Nemo at ".to_string() + &(i + 1).to_string() + "!",
    )
}

/// Finds "Nemo" in a given string and returns a message about whether and where it was found.
pub fn find_nemo(inp: &str) -> String {
    index_to_string(get_index(inp))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_nemo() {
        assert_eq!(
            find_nemo("We lost Nemo!"),
            String::from("I can't find Nemo :(")
        );
    }

    #[test]
    fn just_nemo() {
        assert_eq!(find_nemo("Nemo"), String::from("I found Nemo at 1!"));
    }

    #[test]
    fn nemo_middle() {
        assert_eq!(
            find_nemo("In the Nemo is the middle."),
            String::from("I found Nemo at 3!")
        );
    }

    #[test]
    fn nemo_last() {
        assert_eq!(
            find_nemo("The last one is Nemo"),
            String::from("I found Nemo at 5!")
        );
    }

    #[test]
    fn nemo_s() {
        assert_eq!(find_nemo("Nemo's"), String::from("I can't find Nemo :("));
    }

    #[test]
    fn multiple_nemos() {
        assert_eq!(
            find_nemo("Nemo Nemo Nemo"),
            String::from("I found Nemo at 1!")
        );
    }

    #[test]
    fn small_nemo() {
        assert_eq!(
            find_nemo("Where is nemo ?"),
            String::from("I can't find Nemo :(")
        );
    }
}
