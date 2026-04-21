use std::sync::LazyLock;
use tera::Tera;

pub static TEMPLATES: LazyLock<Tera> = LazyLock::new(|| {
    let mut tera = match Tera::new("templates/**/*.html") {
        Ok(t) => t,
        Err(e) => {
            println!("Error parsing templates! Error: {e}");
            ::std::process::exit(1);
        }
    };

    tera.autoescape_on(vec![".html", ".sql"]);
    tera
});
