use std::str::CharIndices;

/// An iterator over all single chars of a String but returning `&str` instead of `char`.
pub struct CharSubs<'a> {
    ci: CharIndices<'a>,
    s: &'a str,
}

impl<'a> CharSubs<'a> {
    #[inline]
    fn new(s: &'a str) -> Self {
        let ci = s.char_indices();
        Self { ci, s }
    }
}

impl<'a> Iterator for CharSubs<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let (next, u) = self.ci.next()?;
        let len = u.len_utf8();
        Some(&self.s[next..next + len])
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.ci.size_hint()
    }
}

impl<'a> DoubleEndedIterator for CharSubs<'a> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        let (next, u) = self.ci.next_back()?;
        let len = u.len_utf8();
        Some(&self.s[next..next + len])
    }
}

pub trait CharSubsExt<'a> {
    fn char_substrings(&'a self) -> CharSubs<'a>;
}

impl<'a, T> CharSubsExt<'a> for T
where
    Self: AsRef<str>,
{
    #[inline]
    fn char_substrings(&'a self) -> CharSubs<'a> {
        CharSubs::new(self.as_ref())
    }
}

#[cfg(test)]
mod test {
    use super::CharSubsExt;
    use test_case::test_case;

    #[test_case(""; "empty")]
    #[test_case("someTexst"; "ascii")]
    #[test_case("ちょっとユー・ティー・エフ・はち"; "utf8")]
    fn char_substrings(inp: &str) {
        let subs = inp
            .char_substrings()
            .map(|i| i.to_string())
            .collect::<Vec<_>>();
        assert_eq!(subs.len(), inp.char_indices().count());
        let exp: Vec<_> = inp.char_indices().map(|i| i.1.to_string()).collect();
        assert_eq!(subs, exp);
    }

    #[test_case(""; "empty")]
    #[test_case("someTexst"; "ascii")]
    #[test_case("ちょっとユー・ティー・エフ・はち"; "utf8")]
    fn char_substrings_rev(inp: &str) {
        let subs = inp
            .char_substrings()
            .rev()
            .map(|i| i.to_string())
            .collect::<Vec<_>>();
        assert_eq!(subs.len(), inp.char_indices().count());
        let exp: Vec<_> = inp.char_indices().rev().map(|i| i.1.to_string()).collect();
        assert_eq!(subs, exp);
    }
}
