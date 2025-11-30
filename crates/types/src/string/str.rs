use crate::StrExt;
impl StrExt for &str {
    fn is_palindrome(&self) -> bool {
        let s = self.chars().filter(|c| c.is_alphanumeric()).collect::<String>().to_lowercase();
        s == s.chars().rev().collect::<String>()
    }

    fn capitalize(&self) -> String {
        let mut chars = self.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => String::new(),
        }
    }

    fn reverse(&self) -> String {
        self.chars().rev().collect()
    }

    fn reverse_words(&self) -> String {
        self.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
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

