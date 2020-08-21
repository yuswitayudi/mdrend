use clap::{clap_app, crate_version};
use pulldown_cmark::{html::push_html,Event,Parser};


fn main() {

    let clap = clap_app!( mdrend => 
        (version:crate_version!())
        (author:"Yudi Yuswita")
        (about:"Render markdown as you like")
        (@arg input: +required "Sets the input file")
    )
    .get_matches();
    
    // println!("Input = {:?}", clap.value_of("input"));

    //declare vairable input mardown files
    let infile = std::fs::read_to_string(clap.value_of("input").unwrap()).expect("Could not read file");

    let mut res = String::new();
    let ps = Parser::new(&infile);

    // Show running event when process markdown file to html
    let ps : Vec<Event> = ps.into_iter().collect();
    for p in &ps {
        println!("{:?}", p);
    }

    push_html(&mut res, ps.into_iter());

    println!("{}", res);



}
