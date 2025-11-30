mod str;
mod string;
pub trait StrExt {
    fn is_palindrome(&self) -> bool;
    fn capitalize(&self) -> String;
    fn reverse(&self) -> String;
    fn reverse_words(&self) -> String;
    fn count(&self, character: char) -> usize;
}