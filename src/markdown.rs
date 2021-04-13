use crate::yaml::parse_yaml;

use comrak::{markdown_to_html, ComrakOptions};
use extract_frontmatter::Extractor as FrontmatterExtractor;
use serde::de::DeserializeOwned;

fn create_frontmatter_extractor(content: &str) -> FrontmatterExtractor {
    let mut extractor = FrontmatterExtractor::new(content);
    extractor.discard_first_line();
    extractor.select_by_terminator("---");

    extractor
}

fn extract_frontmatter(content: &str) -> (String, String) {
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

pub fn render_markdown<F>(content: &str) -> Result<(F, String), serde_yaml::Error>
where
    F: DeserializeOwned,
{
    let (frontmatter, content) = extract_frontmatter(content);
    let content = markdown_to_html(&content, &ComrakOptions::default());
    let frontmatter: F = parse_yaml(&frontmatter)?;
    Ok((frontmatter, content))
}
