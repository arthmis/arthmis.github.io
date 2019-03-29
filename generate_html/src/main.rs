use pulldown_cmark as md_parser;
use pulldown_cmark::{Parser};
use std::env;
use std::fs::{File, create_dir};
use std::io::{Read, Write};
use horrorshow::prelude::*;
use horrorshow::helper::doctype;
use horrorshow::html;

// make a list of titles for each article to be used as title
// in html macro and assert that the list is as long as the list of
// all the articles

fn main() {

    let file_name = env::args().nth(1).unwrap();
    let mut file = File::open(&file_name).unwrap();
    let mut contents = String::new();
    let bytes = file.read_to_string(&mut contents);
    // println!("{}, md: {}", bytes.unwrap(), contents);

    let markdown_parser = Parser::new(&contents);
    let mut md_html_output: String = String::with_capacity(contents.len() * 3 / 2);
    md_parser::html::push_html(&mut md_html_output, markdown_parser);
    // println!("{}", md_html_output);
    

    let article_title = "deploying blog".to_string();

    let output_document = format!("{}", html! {
        : doctype::HTML;
        html {
            head {
                title : "Blog";
                meta(content="text/html", charset="utf-8");
                link(rel="stylesheet", type="text/css", href="../css/index.css");
                link(rel="stylesheet", type="text/css", href="../css/brands.css");
                link(rel="stylesheet", type="text/css", href="../css/fontawesome.css");
            }
            body {
                header {
                    nav(id="nav-ul") {
                        div {
                            h1 {
                                a(id="home", href="../index.html") {
                                    : "Lazy Passion's Blog"
                                }
                            }
                            a(id="github", class="fab fa-github", href="https://github.com/lazypassion", target="_blank") {
                                : ""
                            }
                        }
                    }
                }
                div(id="content") {
                    article {
                        h1(class="article-title") {
                            : &article_title;
                        }
                        : Raw(&md_html_output);
                    }
                }
            }
        }
    });

    let mut html_file = {
        // TODO: better error handling here
        match create_dir("frontend/articles") {
            Ok(()) => (),
            Err(_err) => (),
        }
        File::create("frontend/articles/temp.html").unwrap()
    };
    html_file.write_all(output_document.as_ref()).unwrap();
}
