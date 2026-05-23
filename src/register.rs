//! Register German analysis components into [`AnalysisFactory`].

use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;

use pizza_engine::analysis::{
    Analyzer, AnalysisFactory, LowercaseNormalizer, Normalizer, StandardTokenizer, TokenFilter,
    Tokenizer,
};

use crate::{GermanLightStemFilter, GermanNormalizationFilter, GermanStopFilter};

/// Register German token filters and the `"german"` analyzer.
pub fn register_all(factory: &mut AnalysisFactory) {
    factory.register_token_filter("german_normalization", Box::new(GermanNormalizationFilter::new()));
    factory.register_token_filter("german_light_stem", Box::new(GermanLightStemFilter::new()));
    factory.register_token_filter("german_stop", Box::new(GermanStopFilter::new()));

    let normalizers: Vec<Box<dyn Normalizer>> = vec![Box::new(LowercaseNormalizer::new())];
    let tokenizer: Box<dyn Tokenizer> = Box::new(StandardTokenizer::new());
    let filters: Vec<Box<dyn TokenFilter>> = vec![
        Box::new(GermanStopFilter::new()),
        Box::new(GermanNormalizationFilter::new()),
        Box::new(GermanLightStemFilter::new()),
    ];
    factory.register_analyzer("german", Analyzer::new(normalizers, tokenizer, filters));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_all_no_panic() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
    }

    #[test]
    fn test_filters_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_token_filter("german_normalization").is_some());
        assert!(factory.get_token_filter("german_light_stem").is_some());
        assert!(factory.get_token_filter("german_stop").is_some());
    }

    #[test]
    fn test_analyzer_registered() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        assert!(factory.get_analyzer("german").is_some());
    }

    #[test]
    fn test_analyzer_pipeline() {
        let mut factory = AnalysisFactory::new();
        register_all(&mut factory);
        let analyzer = factory.get_analyzer("german").unwrap();
        let mut input = String::from("Die Straße ist nicht lang");
        let tokens = analyzer.analyze_and_return_tokens(&mut input);
        // "die" and "ist" and "nicht" are stop words
        assert!(!tokens.iter().any(|t| t.term == "die"));
        assert!(!tokens.iter().any(|t| t.term == "ist"));
        assert!(tokens.len() >= 1);
    }
}
