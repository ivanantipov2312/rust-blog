use pulldown_cmark::{Event, Parser};
use ammonia::clean;

pub fn markdown_to_html(markdown: &str) -> String {
    let options = pulldown_cmark::Options::all();
    let parser = Parser::new_ext(markdown, options)
        .filter(|event| !matches!(event, Event::Html(_)));
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    clean(&html_output)
}
