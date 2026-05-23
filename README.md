# pizza-analysis-german

German language analysis with umlaut/ß normalization, light/minimal stemmers, and stop words.

Part of the [Pizza](https://pizza.rs) search engine.

## Components

| Name | Type | Description |
|------|------|-------------|
| `german_normalization` | Token Filter | Normalizes ä→a, ö→o, ü→u, ß→ss, and ae/oe/ue digraphs |
| `german_stem` | Token Filter | German light stemmer — removes common suffixes |
| `german_minimal_stem` | Token Filter | German minimal stemmer — conservative suffix removal |
| `german_stop` | Token Filter | German stop words filter (231 words) |
| `german` | Analyzer | Full pipeline: lowercase → normalization → stop → stem |

## Usage

### Built-in Analyzer

```json
{
  "analyzer": {
    "type": "german"
  }
}
```

### Custom Pipeline

```json
{
  "analyzer": {
    "type": "custom",
    "tokenizer": "standard",
    "filter": ["german_normalization", "german_stem", "german_minimal_stem", "german_stop"]
  }
}
```

## License

MIT — see [LICENSE](LICENSE).

## Related Crates

- [analysis-core](https://github.com/pizza-rs/analysis-core) — Core analysis components and pipeline
- [analysis-icu](https://github.com/pizza-rs/analysis-icu) — ICU Unicode normalization and tokenization
- [analysis-english](https://github.com/pizza-rs/analysis-english) — English analysis
- [analysis-all](https://github.com/pizza-rs/analysis-all) — Meta-crate registering all analyzers
