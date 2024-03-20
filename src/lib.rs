/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    for x in 'a'..'z' {
        if !sentence.contains(x) {
            return false;
        }
    }
    true
}
