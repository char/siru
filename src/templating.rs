use tera::{Error, Tera};

pub fn create_templating_engine(template_glob: &str) -> Result<Tera, Error> {
    Tera::new(template_glob).map(|mut tera| {
        tera.autoescape_on(vec![".html", ".html.j2"]);
        tera
    })
}
