use html5minify::Minify;

pub fn minify_html(html: impl AsRef<[u8]>) -> Result<String, std::io::Error> {
    let minified_html =
        String::from_utf8(html.minify()?).expect("Minifed output was somehow not valid UTF-8");

    Ok(minified_html)
}
