use crate::ast_core::Span;

struct WhitespaceSplitter<'a> {
    s: &'a str,
    char_indices: std::str::CharIndices<'a>,
    span: Span,
}

impl<'a> Iterator for WhitespaceSplitter<'a> {
    type Item = (&'a str, Span);

    fn next(&mut self) -> Option<Self::Item> {
        let mut start;
        // we skip any whitespace characters, then take the first
        // non-whitespace sequence we get
        loop {
            if let Some((i, c)) = self.char_indices.next() {
                start = i;
                if !c.is_whitespace() {
                    break;
                }
            } else {
                return None;
            }
        }
        // now we take as many non-whitespace characters we find
        let mut end;
        loop {
            if let Some((i, c)) = self.char_indices.next() {
                end = i;
                if c.is_whitespace() {
                    break;
                }
            } else {
                end = self.s.len();
                break;
            }
        }

        let span = Span {
            start: self.span.start + start,
            end: self.span.start + end,
        };
        // next time, look at end
        Some((&self.s[start..end], span))
    }
}

pub(crate) fn split_whitespace_with_spans(
    s: &str,
    span: Span,
) -> impl Iterator<Item = (&str, Span)> {
    WhitespaceSplitter {
        s,
        char_indices: s.char_indices(),
        span,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_whitespace_with_spans_simple() {
        let s = "hello world";
        let splitted: Vec<_> = split_whitespace_with_spans(s, Span::new(0, 0)).collect();
        assert_eq!(splitted.len(), 2);
        assert_eq!(splitted[0].0, "hello");
        assert_eq!(splitted[0].1, Span::new(0, 5));
        assert_eq!(splitted[1].0, "world");
        assert_eq!(splitted[1].1, Span::new(6, 11));
    }

    #[test]
    fn test_split_whitespace_with_spans_long_whitespace() {
        let s = "hello   world";
        let splitted: Vec<_> = split_whitespace_with_spans(s, Span::new(0, 0)).collect();
        assert_eq!(splitted.len(), 2);
        assert_eq!(splitted[0].0, "hello");
        assert_eq!(splitted[0].1, Span::new(0, 5));
        assert_eq!(splitted[1].0, "world");
        assert_eq!(splitted[1].1, Span::new(8, 13));
    }

    #[test]
    fn test_split_whitespace_multiple() {
        let s = "alpha beta gamma";
        let splitted: Vec<_> = split_whitespace_with_spans(s, Span::new(0, 0)).collect();
        assert_eq!(splitted.len(), 3);
        assert_eq!(splitted[0].0, "alpha");
        assert_eq!(splitted[0].1, Span::new(0, 5));
        assert_eq!(splitted[1].0, "beta");
        assert_eq!(splitted[1].1, Span::new(6, 10));
        assert_eq!(splitted[2].0, "gamma");
        assert_eq!(splitted[2].1, Span::new(11, 16));
    }

    #[test]
    fn test_no_whitespace() {
        let s = "alpha";
        let splitted: Vec<_> = split_whitespace_with_spans(s, Span::new(0, 0)).collect();
        assert_eq!(splitted.len(), 1);
        assert_eq!(splitted[0].0, "alpha");
        assert_eq!(splitted[0].1, Span::new(0, 5));
    }

    #[test]
    fn test_leading_whitespace() {
        let s = "  alpha";
        let splitted: Vec<_> = split_whitespace_with_spans(s, Span::new(0, 0)).collect();
        assert_eq!(splitted.len(), 1);
        assert_eq!(splitted[0].0, "alpha");
        assert_eq!(splitted[0].1, Span::new(2, 7));
    }

    #[test]
    fn test_trailing_whitespace() {
        let s = "alpha  ";
        let splitted: Vec<_> = split_whitespace_with_spans(s, Span::new(0, 0)).collect();
        assert_eq!(splitted.len(), 1);
        assert_eq!(splitted[0].0, "alpha");
        assert_eq!(splitted[0].1, Span::new(0, 5));
    }
}
