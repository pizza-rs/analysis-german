<div align="center">

# 🇩🇪 pizza-analysis-german

**German text analysis plugin for [INFINI Pizza](https://pizza.rs)**

[![Crate](https://img.shields.io/badge/crate-pizza--analysis--german-blue)](https://github.com/pizza-rs/analysis-german)
[![License](https://img.shields.io/badge/license-MIT-green)](LICENSE)

</div>

---

## Overview

German language analysis with umlaut normalization, light stemming, and stop words.
Handles German-specific orthographic conventions including ä→ae, ö→oe, ü→ue, ß→ss
normalization.

## Components

| Type | Name | Description |
|:-----|:-----|:------------|
| TokenFilter | `german_normalization` | Normalize umlauts and eszett (ä→a, ö→o, ü→u, ß→ss) |
| TokenFilter | `german_light_stem` | German light stemmer |
| TokenFilter | `german_stop` | German stop words (231 entries) |
| Analyzer | `german` | Full pipeline: lowercase → normalization → light_stem → stop |

### Normalization Rules

| Input | Output | Rule |
|:------|:-------|:-----|
| ä / ae | a | Umlaut-a |
| ö / oe | o | Umlaut-o |
| ü / ue | u | Umlaut-u |
| ß | ss | Eszett |

## Example

```rust
use pizza_engine::analysis::AnalysisFactory;

let mut factory = AnalysisFactory::new();
pizza_analysis_german::register_all(&mut factory);

let analyzer = factory.get_analyzer("german").unwrap();
// "Straßenbäume" → ["strassbaum"]
```

## Installation

```toml
[dependencies]
pizza-analysis-german = "0.1"
```

Or via `pizza-analysis-all`:

```toml
[dependencies]
pizza-analysis-all = { version = "0.1", features = ["german"] }
```

## License

MIT

---

<div align="center">
<sub>Part of the <a href="https://pizza.rs">INFINI Pizza</a> ecosystem</sub>
</div>
