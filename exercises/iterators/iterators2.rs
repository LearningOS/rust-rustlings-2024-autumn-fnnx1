// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[ &str]) -> Vec<String> {
    let mut wordss =Vec::new();
    let mut y = wordss.iter();
    for x in words {
        let y = x.to_string();
        let mut y = y.chars();

        wordss.push(
             match y.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + y.as_str(),
            }
        );
    }
    wordss
    }

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut wordss =Vec::new();
    let mut y = wordss.iter();
    let mut wordsss=String::new();
    for x in words {
        let y = x.to_string();
        let mut y = y.chars();

        wordss.push(
             match y.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + y.as_str(),
            }
        );
    }
    for xx in wordss {
        wordsss += &xx;
    }
    wordsss
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
