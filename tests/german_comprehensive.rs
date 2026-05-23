//! Comprehensive tests for pizza-analysis-german.

use pizza_analysis_german::*;
use pizza_engine::analysis::{AnalysisFactory, Token, TokenFilter};

fn make_token(term: &str) -> Token<'_> {
    Token::new(term, 0, term.len() as u32, 0)
}

// ═══════════════════════════════════════════════════════════════════════════════
// GermanNormalizationFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn normalization_construction() {
    let _f = GermanNormalizationFilter::new();
}

#[test]
fn normalization_eszett() {
    let f = GermanNormalizationFilter::new();
    // "straße" → "strasse"
    let mut token = make_token("straße");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "strasse");
}

#[test]
fn normalization_umlaut_a() {
    let f = GermanNormalizationFilter::new();
    // "ä" → "a"
    let mut token = make_token("mädchen");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(!token.term.contains('ä'));
}

#[test]
fn normalization_umlaut_o() {
    let f = GermanNormalizationFilter::new();
    // "ö" → "o"
    let mut token = make_token("schön");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(!token.term.contains('ö'));
}

#[test]
fn normalization_umlaut_u() {
    let f = GermanNormalizationFilter::new();
    // "ü" → "u"
    let mut token = make_token("über");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert!(!token.term.contains('ü'));
}

#[test]
fn normalization_digraph_ae() {
    let f = GermanNormalizationFilter::new();
    // "ae" → "a"
    let mut token = make_token("aequivalent");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn normalization_ascii_passthrough() {
    let f = GermanNormalizationFilter::new();
    let mut token = make_token("hello");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
    assert_eq!(token.term.as_ref(), "hello");
}

#[test]
fn normalization_empty_string() {
    let f = GermanNormalizationFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// GermanLightStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn light_stem_construction() {
    let _f = GermanLightStemFilter::new();
}

#[test]
fn light_stem_plural_e() {
    let f = GermanLightStemFilter::new();
    // "hunde" (dogs) → stem
    let mut token = make_token("hunde");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn light_stem_plural_en() {
    let f = GermanLightStemFilter::new();
    // "blumen" (flowers) → stem
    let mut token = make_token("blumen");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn light_stem_plural_er() {
    let f = GermanLightStemFilter::new();
    // "kinder" (children) → stem
    let mut token = make_token("kinder");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn light_stem_adjective_es() {
    let f = GermanLightStemFilter::new();
    // "großes" (big) → stem
    let mut token = make_token("grosses");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn light_stem_short_word() {
    let f = GermanLightStemFilter::new();
    let mut token = make_token("ab");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

#[test]
fn light_stem_empty_string() {
    let f = GermanLightStemFilter::new();
    let mut token = make_token("");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// GermanMinimalStemFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn minimal_stem_construction() {
    let _f = GermanMinimalStemFilter::new();
}

#[test]
fn minimal_stem_plural() {
    let f = GermanMinimalStemFilter::new();
    let mut token = make_token("katzen");
    let (deleted, _) = f.filter(&mut token);
    assert!(!deleted);
}

// ═══════════════════════════════════════════════════════════════════════════════
// GermanStopFilter
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn stop_construction() {
    let _f = GermanStopFilter::new();
}

#[test]
fn stop_filters_common_words() {
    let f = GermanStopFilter::new();
    let stop_words = ["der", "die", "das", "und", "ist", "in", "von", "den", "mit", "nicht"];
    for word in &stop_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(deleted, "stop word '{}' should be filtered", word);
    }
}

#[test]
fn stop_keeps_content_words() {
    let f = GermanStopFilter::new();
    let content_words = ["haus", "buch", "schule", "stadt"];
    for word in &content_words {
        let mut token = make_token(word);
        let (deleted, _) = f.filter(&mut token);
        assert!(!deleted, "content word '{}' should be kept", word);
    }
}

#[test]
fn stop_empty_string() {
    let f = GermanStopFilter::new();
    let mut token = make_token("");
    let _ = f.filter(&mut token);
}

// ═══════════════════════════════════════════════════════════════════════════════
// Registration
// ═══════════════════════════════════════════════════════════════════════════════

#[test]
fn register_all_no_panic() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
}

#[test]
fn register_all_filters_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_token_filter("german_normalization").is_some());
    assert!(factory.get_token_filter("german_light_stem").is_some());
    assert!(factory.get_token_filter("german_stop").is_some());
}

#[test]
fn register_all_analyzer_present() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    assert!(factory.get_analyzer("german").is_some());
}

#[test]
fn analyzer_pipeline_produces_tokens() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("german").unwrap();
    let mut input = String::from("Das Haus ist groß und schön");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}

#[test]
fn analyzer_pipeline_removes_stops() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("german").unwrap();
    let mut input = String::from("der hund ist in dem haus");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    let terms: Vec<&str> = tokens.iter().map(|t| t.term.as_ref()).collect();
    assert!(!terms.contains(&"der"));
    assert!(!terms.contains(&"ist"));
    assert!(!terms.contains(&"in"));
}

#[test]
fn analyzer_pipeline_empty_input() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("german").unwrap();
    let mut input = String::from("");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(tokens.is_empty());
}

#[test]
fn analyzer_pipeline_umlaut_handling() {
    let mut factory = AnalysisFactory::new();
    register_all(&mut factory);
    let analyzer = factory.get_analyzer("german").unwrap();
    let mut input = String::from("über straße");
    let tokens = analyzer.analyze_and_return_tokens(&mut input);
    assert!(!tokens.is_empty());
}
