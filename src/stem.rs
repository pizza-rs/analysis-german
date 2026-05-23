//! German stemmers (light and minimal).

use alloc::borrow::Cow;
use alloc::string::String;
use alloc::vec::Vec;
use pizza_engine::analysis::{Token, TokenFilter};

/// German light stemmer — removes plural, case, and derivational suffixes.
#[derive(Clone, Debug, Default)]
pub struct GermanLightStemFilter;

impl GermanLightStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for GermanLightStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 5 {
            return (false, None);
        }
        let stemmed = stem_german_light(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_german_light(word: &str) -> String {
    let mut result = String::from(word);

    if result.ends_with("ern") || result.ends_with("eln") {
        result.truncate(result.len() - 2);
    } else if result.ends_with("en") || result.ends_with("er") || result.ends_with("es") {
        result.truncate(result.len() - 2);
    } else if result.ends_with('e') || result.ends_with('s') || result.ends_with('n') {
        result.pop();
    }

    // Remove umlaut
    let chars: Vec<char> = result.chars().collect();
    let mut out = String::with_capacity(result.len());
    let mut changed = false;
    for c in &chars {
        match c {
            'ä' => { out.push('a'); changed = true; }
            'ö' => { out.push('o'); changed = true; }
            'ü' => { out.push('u'); changed = true; }
            _ => out.push(*c),
        }
    }
    if changed { result = out; }

    result
}

/// German minimal stemmer — only removes basic plural markers.
#[derive(Clone, Debug, Default)]
pub struct GermanMinimalStemFilter;

impl GermanMinimalStemFilter {
    pub fn new() -> Self {
        Self
    }
}

impl TokenFilter for GermanMinimalStemFilter {
    fn filter<'a>(&self, token: &mut Token<'a>) -> (bool, Option<Vec<Token<'a>>>) {
        let text = token.term.as_ref();
        if text.len() < 5 {
            return (false, None);
        }
        let stemmed = stem_german_minimal(text);
        if stemmed != text {
            token.term = Cow::Owned(stemmed);
        }
        (false, None)
    }
}

fn stem_german_minimal(word: &str) -> String {
    let mut result = String::from(word);

    if result.ends_with("ern") {
        result.truncate(result.len() - 3);
    } else if result.ends_with("en") || result.ends_with("er") || result.ends_with("es") {
        result.truncate(result.len() - 2);
    } else if result.ends_with('e') || result.ends_with('s') {
        result.pop();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_light_plural() {
        let f = GermanLightStemFilter::new();
        let mut token = Token::new("bücher", 0, 8, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "buch");
    }

    #[test]
    fn test_light_en() {
        let f = GermanLightStemFilter::new();
        let mut token = Token::new("katzen", 0, 6, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "katz");
    }

    #[test]
    fn test_minimal() {
        let f = GermanMinimalStemFilter::new();
        let mut token = Token::new("häuser", 0, 8, 0);
        f.filter(&mut token);
        assert_eq!(token.term, "häus");
    }
}
