pub trait StrExt {
    fn first_letter_upper(&self) -> String;
}

impl<T: AsRef<str>> StrExt for T {
    /// Makes the first character to uppercase and returns a newly owned string
    #[inline]
    fn first_letter_upper(&self) -> String {
        let mut c = self.as_ref().chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().chain(c).collect(),
        }
    }
}
