//! German character normalization.

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// Normalizes German characters (ß→ss, umlauts, ae/oe/ue digraphs).
///
/// Equivalent to Lucene's `GermanNormalizationFilter`.
#[derive(Clone, Debug, Default)]
pub struct GermanNormalizationFilter;

const N: u8 = 0; // ordinary
const V: u8 = 1; // vowel — blocks ue folding
const U: u8 = 2; // umlaut state — allows e-deletion

impl GermanNormalizationFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for GermanNormalizationFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.is_empty() {
            return (false, None);
        }

        let mut result = String::with_capacity(text.len());
        let mut state = N;
        let chars: Vec<char> = text.chars().collect();

        for &c in &chars {
            match c {
                'a' | 'o' => {
                    state = U;
                    result.push(c);
                }
                'u' => {
                    state = if state == N { U } else { V };
                    result.push(c);
                }
                'e' => {
                    if state == U {
                        // ae→a, oe→o, ue→u: delete the 'e'
                    } else {
                        result.push(c);
                    }
                    state = V;
                }
                'i' | 'q' | 'y' => {
                    state = V;
                    result.push(c);
                }
                'ä' => {
                    result.push('a');
                    state = V;
                }
                'ö' => {
                    result.push('o');
                    state = V;
                }
                'ü' => {
                    result.push('u');
                    state = V;
                }
                'ß' => {
                    result.push('s');
                    result.push('s');
                    state = N;
                }
                _ => {
                    state = N;
                    result.push(c);
                }
            }
        }

        if result != text {
            token.term = Cow::Owned(result);
        }
        (false, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sharp_s() {
        let f = GermanNormalizationFilter::new();
        let mut token = Token::new("weißbier", 0, 12, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "weissbier");
    }

    #[test]
    fn test_umlaut() {
        let f = GermanNormalizationFilter::new();
        let mut token = Token::new("über", 0, 6, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "uber");
    }

    #[test]
    fn test_ae_digraph() {
        let f = GermanNormalizationFilter::new();
        let mut token = Token::new("schoen", 0, 6, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "schon");
    }
}
