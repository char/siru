use crate::yaml::parse_yaml;

pub use comrak::markdown_to_html;
use comrak::ComrakOptions;
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

pub fn render_markdown<F>(content: &str) -> Result<(F, String), serde_yaml::Error>
where
    F: DeserializeOwned,
{
    render_markdown_with_options(content, &MarkdownOptions::default())
}

pub fn render_markdown_with_options<F>(
    content: &str,
    options: &MarkdownOptions,
) -> Result<(F, String), serde_yaml::Error>
where
    F: DeserializeOwned,
{
    let (frontmatter, content) = extract_frontmatter(content);
    let content = markdown_to_html(&content, options);
    let frontmatter: F = parse_yaml(&frontmatter)?;
    Ok((frontmatter, content))
}
