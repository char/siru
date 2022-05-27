use std::collections::HashMap;

use crate::yaml::parse_yaml;

use crate::syntax_highlighting::SyntaxHighlighter;
pub use comrak::markdown_to_html;
use comrak::{
    adapters::SyntaxHighlighterAdapter, markdown_to_html_with_plugins, ComrakOptions, ComrakPlugins,
};
use extract_frontmatter::Extractor as FrontmatterExtractor;
use serde::de::DeserializeOwned;

pub type MarkdownOptions = ComrakOptions;

fn create_frontmatter_extractor(content: &str) -> FrontmatterExtractor {
    let mut extractor = FrontmatterExtractor::new(content);
    extractor.discard_first_line();
    extractor.select_by_terminator("---");

    extractor
}

pub fn extract_frontmatter(content: &str) -> (String, String) {
    let extractor = create_frontmatter_extractor(content);
    let frontmatter = extractor.extract();
    let content = extractor.remove();

    (frontmatter, content.to_string())
}

pub fn parse_frontmatter<F>(content: &str) -> Result<F, serde_yaml::Error>
where
    F: DeserializeOwned,
{
    let extractor = create_frontmatter_extractor(content);
    let frontmatter: F = parse_yaml(&extractor.extract())?;

    Ok(frontmatter)
}

struct ChromaSyntaxisAdapter<'a, 'b>(&'a SyntaxHighlighter<'b>);

impl SyntaxHighlighterAdapter for ChromaSyntaxisAdapter<'_, '_> {
    fn highlight(&self, lang: Option<&str>, code: &str) -> String {
        match lang {
            Some(lang) => self.0.highlight(lang, code),
            None => code.to_string(),
        }
    }
    fn build_pre_tag(&self, _attributes: &HashMap<String, String>) -> String {
        String::from("<pre>")
    }
    fn build_code_tag(&self, attributes: &HashMap<String, String>) -> String {
        let mut s = String::from("<code");

        for (k, v) in attributes {
            s.push(' ');
            s.push_str(k);
            s.push('=');
            s.push('"');
            s.push_str(v);
            s.push('"');
        }

        s.push('>');
        s
    }
}

pub fn render_markdown<F>(
    content: &str,
    options: &MarkdownOptions,
    syntax_highlighter: Option<&SyntaxHighlighter>,
) -> Result<(F, String), serde_yaml::Error>
where
    F: DeserializeOwned,
{
    let (frontmatter, content) = extract_frontmatter(content);
    let mut plugins = ComrakPlugins::default();

    let adapter = syntax_highlighter.map(ChromaSyntaxisAdapter);
    plugins.render.codefence_syntax_highlighter =
        adapter.as_ref().map(|s| s as &dyn SyntaxHighlighterAdapter);

    let content = markdown_to_html_with_plugins(&content, options, &plugins);
    let frontmatter: F = parse_yaml(&frontmatter)?;
    Ok((frontmatter, content))
}
