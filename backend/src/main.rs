use rocket::{get, post, routes, launch};
use rocket::serde::json::Json;
use rocket::response::content::RawHtml;
use pulldown_cmark::{html, Parser};

// Handler for the root path "/"
#[get("/")]
fn index() -> &'static str {
    "Welcome to the Markdown to HTML Converter! Use /convert to convert markdown."
}

// Handler for the favicon.ico request (avoiding 404)
#[get("/favicon.ico")]
fn favicon() -> RawHtml<&'static str> {
    RawHtml("<!DOCTYPE html><html><head><link rel='icon' href='/favicon.ico'></head></html>")
}

// The actual markdown conversion handler
#[post("/markdown", format = "json", data = "<markdown>")]
fn render_markdown(markdown: Json<String>) -> String {
    let parser = Parser::new(&markdown);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, favicon, render_markdown])
}
