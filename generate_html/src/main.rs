#![recursion_limit="256"]

use pulldown_cmark as md_parser;
use pulldown_cmark::{Parser};
use typed_html::{dom::DOMTree, html, text};
use std::env;
use std::fs::{File};
use std::io::{Read, Write};

// make a list of titles for each article to be used as title
// in html macro and assert that the list is as long as the list of
// all the articles

fn main() {

    let file_name = env::args().nth(1).unwrap();
    println!("{}", file_name);
    let mut file = File::open(&file_name).unwrap();
    let mut contents = String::new();
    let bytes = file.read_to_string(&mut contents);
    // println!("{}, md: {}", bytes.unwrap(), contents);

    let markdown_parser = Parser::new(&contents);
    let mut md_html_output: String = String::with_capacity(contents.len() * 3 / 2);
    md_parser::html::push_html(&mut md_html_output, markdown_parser);
    // println!("{}", md_html_output);
    

    let article_title = "deploying blog".to_string();
    let doc: String = html!(
        <html>
            <head>
                <title>"Blog"</title>
                <meta content="text/html" charset="utf-8"/>
                <link rel="stylesheet" type="text/css" href="index.css"/>
                <script src="index.js"></script>
            </head>

            <body>
                <h1>"Hello"</h1>
            <header>
                <nav id="nav-ul">
                    <h1><a id="home" href="index.html">"Lazy Passion's Blog"</a></h1>
                    <a id="github" class="menu" href="https://github.com/lazypassion" target="_blank">"git"</a>
                </nav>
            </header>
            <div id="content">
                <article>
                    <h2 class="article-title"><a href="">{text!("{}", article_title)}</a></h2>
                    {text!("{}", md_html_output)}
                </article>
            </div>

            </body>
        </html>
    :String).to_string();
    // println!("{}", doc);
    let mut html_file = File::create("articles/temp.html").unwrap();
    html_file.write_all(doc.as_ref()).unwrap();
}
