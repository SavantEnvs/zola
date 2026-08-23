#![no_main]
use std::collections::HashMap;

use ahash::AHashMap;
use config::Config;
use libfuzzer_sys::fuzz_target;
use markdown::{MarkdownContext, render_content};
use templates::ZOLA_TERA;
use utils::types::InsertAnchor;

// Drives zola's markdown+shortcode rendering pipeline over arbitrary input.
// render_content() runs the shortcode parser (parse_for_shortcodes — the code path
// the original `shortcodes` target fuzzed) and the full markdown-to-HTML renderer.
// Context construction mirrors components/markdown/tests/common.rs's `render()` helper,
// which is the idiomatic way to build a MarkdownContext outside the full site pipeline.
fuzz_target!(|data: &str| {
    let config = Config::default_for_test();
    let tera = ZOLA_TERA.clone();
    let permalinks: HashMap<String, String> = HashMap::new();
    let colocated_assets: AHashMap<String, (String, String)> = AHashMap::new();
    let context = MarkdownContext {
        tera: &tera,
        config: &config,
        permalinks: &permalinks,
        colocated_assets: &colocated_assets,
        lang: &config.default_language,
        current_permalink: "https://www.getzola.org/test/",
        current_path: "my_page.md",
        insert_anchor: InsertAnchor::None,
    };
    let _ = render_content(data, &context);
});
