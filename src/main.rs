#![feature(proc_macro_hygiene)]

use clap::{clap_app, crate_version};
use maud::html;
use pulldown_cmark::{html::push_html, Event, Parser};

fn wrap_html(s: &str, css: Option<&str>) -> String {
    let res = html! {
        (maud::DOCTYPE)
        head{
            head{
                meta charset="utf-8";
                @if let Some(s) = css {
                    link rel="stylesheet" type="text/css" href=(s) {}
                }
            }
            body {
                (maud::PreEscaped(s))
            }
        }
    };
    res.into_string()
}

fn main() {
    let clap = clap_app!( mdrend =>
        (version:crate_version!())
        (author:"Yudi Yuswita")
        (about:"Render markdown as you like")
        (@arg input: +required "Sets the input file")
        (@arg wrap: -w "Wrap in html")
        (@arg event: -e "Print event")
        (@arg css: --css +takes_value "Link to css")
    )
    .get_matches();

    // println!("Input = {:?}", clap.value_of("input"));

    //declare variable input mardown files
    let infile =
        std::fs::read_to_string(clap.value_of("input").unwrap()).expect("Could not read file");

    let ps = Parser::new(&infile);
    let ps: Vec<Event> = ps.into_iter().collect();
    if clap.is_present("event") {
        // Show running event when process markdown file to html
        for p in &ps {
            println!("{:?}", p);
        }
    }

    let mut res = String::new();
    push_html(&mut res, ps.into_iter());

    if clap.is_present("wrap") {
        res = wrap_html(&res, clap.value_of("css"));
    }

    println!("{}", res);
}
