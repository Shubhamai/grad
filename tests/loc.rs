// https://brenocon.com/dean_perf.html
// https://vercel.com/blog/latency-numbers-every-web-developer-should-know


// Lines of Code

use tokei::{Config, LanguageType, Languages};

#[test]
fn loc() {
    let paths = &["src"];

    let excluded = &["target", "tests"];

    let config = Config::default();

    let mut languages = Languages::new();
    languages.get_statistics(paths, excluded, &config);
    let rust = &languages[&LanguageType::Rust];

    assert!(rust.code < 1600);
}
