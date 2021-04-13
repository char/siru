use html5minify::Minify;

const MIN_HTML_HEADER: &'static str = concat!(
    "<!-- This file is minified, but you can find this site's original source at ",
    "https://github.com/videogame-hacker/char.lt/",
    " -->\n",
);

pub fn minify_html(html: String) -> Result<String, std::io::Error> {
    let mut minified_html =
        String::from_utf8(html.minify()?).expect("Minifed output was somehow not valid UTF-8");
    minified_html.insert_str(0, MIN_HTML_HEADER);

    Ok(minified_html)
}
