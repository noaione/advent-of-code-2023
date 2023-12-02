/// Simple utility function to split a string into lines, ignoring empty lines.
pub fn split_into_lines(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|line| !line.is_empty())
}
