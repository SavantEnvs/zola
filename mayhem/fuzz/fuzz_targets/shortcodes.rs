#![no_main]
use std::collections::HashMap;

use config::Config;
use libfuzzer_sys::fuzz_target;
use markdown::{RenderContext, render_content};
use templates::ZOLA_TERA;
use utils::types::InsertAnchor;

// Drives zola's markdown+shortcode rendering pipeline over arbitrary input.
// render_content() runs the shortcode parser (parse_for_shortcodes — the code path
// the original `shortcodes` target fuzzed) and the full markdown-to-HTML renderer.
// Context construction mirrors components/markdown/src/context.rs's RenderContext::new,
// the API this commit exposes (later renamed/reshaped into MarkdownContext upstream).
fuzz_target!(|data: &str| {
    let config = Config::default_for_test();
    let tera = ZOLA_TERA.clone();
    let permalinks: HashMap<String, String> = HashMap::new();
    let context = RenderContext::new(
        &tera,
        &config,
        &config.default_language,
        "https://www.getzola.org/test/",
        &permalinks,
        InsertAnchor::None,
    );
    let _ = render_content(data, &context);
});
