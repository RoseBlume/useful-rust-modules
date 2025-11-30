use crate::StrExt;
// Works for String as well
impl StrExt for String {
    fn is_palindrome(&self) -> bool {
        self.as_str().is_palindrome()
    }

    fn capitalize(&self) -> String {
        self.as_str().capitalize()
    }

    fn reverse(&self) -> String {
        self.as_str().reverse()
    }
    fn reverse_words(&self) -> String {
        self.as_str().reverse_words()
    }
    fn count(&self, character: char) -> usize {
        let mut number: usize = 0;
        for i in self.chars() {
            if i == character {
                number += 1;
            } 
        }
        number
    }
}
