mod wikipedia_random;

use clap::{App, Arg};
use std::error::Error;
use wikipedia_random::{Fetcher, fetcher::FetcherType, printer::Alignment, Printer};


fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("Wikipedia API")
        .version("0.1.0")
        .author("Leo Blume")
        .about("Displays information from Wikipedia.")
        .arg(
            Arg::with_name("lang")
                .short("l")
                .long("lang")
                .takes_value(true)
                .help("Language"),
        )
        .arg(
            Arg::with_name("links")
                .short("x")
                .long("links")
                .takes_value(false)
                .help("Display links of article"),
        )
        .arg(
            Arg::with_name("raw")
                .short("R")
                .long("raw")
                .takes_value(false)
                .help("Print the raw json request"),
        )
        .arg(
            Arg::with_name("padding")
                .short("p")
                .long("padding")
                .takes_value(true)
                .help("Amount of padding."),
        )
        .arg(
            Arg::with_name("width")
                .short("w")
                .long("width")
                .takes_value(true)
                .help("Width of the box."),
        )
        .arg(
            Arg::with_name("align")
                .short("a")
                .long("align")
                .takes_value(true)
                .help("Align the text in the box."),
        )
        .get_matches();

        let alignment: Option<Alignment> = match matches.value_of("align") {
            None => None,
            Some("left"   | "l") => Some(Alignment::LEFT),
            Some("middle" | "m") => Some(Alignment::MIDDLE),
            Some("right"  | "r") => Some(Alignment::RIGHT),
            Some(&_) => None,
        };
    
        let width: Option<u16> = match matches.value_of("width") {
            None => None,
            Some(s) => match s.parse::<u16>() {
                Ok(n) => Some(n),
                Err(_) => None,
            },
        };
    
        let padding: Option<usize> = match matches.value_of("padding") {
            None => None,
            Some(s) => match s.parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => None,
            },
        };

    let language = matches.value_of("lang").unwrap_or("de");
    let url = format!(
        "https://{}.wikipedia.org/api/rest_v1/page/random/summary",
        language
    );

    let fetcher = Fetcher::new(url, FetcherType::SingleBlocking);
   
    let printer: Printer = Printer::new(
        alignment,
        width,
        padding,
    );

    match fetcher.fetch() {
        Ok(article) => {
            printer.print_message(&article.displaytitle,&article.extract);            
        }
        Err(e) => printer.print_error(e.to_string()),
    }









     Ok(())
}
