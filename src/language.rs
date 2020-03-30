use serde::Serialize;
use std::path::Path;

#[derive(Debug, Copy, Clone, PartialEq, Serialize)]
pub enum Language {
    CSS,
    Elixir,
    Elm,
    HTML,
    JSON,
    JavaScript,
    Markdown,
    Ruby,
    SCSS,
    Sh,
    SVG,
    TypeScript,
    XML,
}

pub fn calculate_language(input: &str) -> Option<Language> {
    match Path::new(input).extension().and_then(|v| v.to_str()) {
        Some("css") => Some(Language::CSS),
        Some("ex") => Some(Language::Elixir),
        Some("exs") => Some(Language::Elixir),
        Some("elm") => Some(Language::Elm),
        Some("html") => Some(Language::HTML),
        Some("json") => Some(Language::JSON),
        Some("js") => Some(Language::JavaScript),
        Some("jsx") => Some(Language::JavaScript),
        Some("md") => Some(Language::Markdown),
        Some("rb") => Some(Language::Ruby),
        Some("scss") => Some(Language::SCSS),
        Some("svg") => Some(Language::SVG),
        Some("ts") => Some(Language::TypeScript),
        Some("tsx") => Some(Language::TypeScript),
        Some("xml") => Some(Language::XML),
        None => Some(Language::Sh),
        _ => None,
    }
}

#[test]
fn calculates_common_files() {
    assert_eq!(calculate_language("../foo/bar.rb"), Some(Language::Ruby));
    assert_eq!(calculate_language("/tmp/foo.md"), Some(Language::Markdown));
    assert_eq!(calculate_language("bin/rails"), Some(Language::Sh));
    assert_eq!(calculate_language("file.unknown"), None);
}
