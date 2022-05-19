use clap::{App, Arg};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize)]
#[allow(dead_code)]
struct Namespace {
    id: i32,
    text: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Titles {
    canonical: String,
    normalized: String,
    display: String,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Image {
    source: String,
    width: i32,
    height: i32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Coordinates {
    lat: f32,
    lon: f32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct Article {
    title: String,
    extract: String,
    displaytitle: String,
    namespace: Namespace,
    wikibase_item: String,
    titles: Titles,
    pageid: i128,
    thumbnail: Image,
    coordinates: Option<Coordinates>,
    lang: String,
    dir: String,
    revision: String,
    tid: String,
    timestamp: String,
    description: String,
    description_source: String,
}

fn print_box(string: String, width: Option<u16>, padding: Option<(u8, u8)>) {
    let unwrapped_padding = padding.unwrap_or((1, 1));

    let words = string.split(" ");
    let mut lines: Vec<String> = Vec::new();

    for word in words {
        if lines.len() == 0 {
            lines.push(word.to_string());
            continue;
        }
        let last_length = lines.last().unwrap().chars().count();
        if last_length < width.unwrap_or(50).into() || last_length == 0 {
            *lines.last_mut().unwrap() = lines.last_mut().unwrap().to_owned() + " " + word;
        } else {
            lines.push(word.to_string());
        }
    }

    let max_length = lines
        .iter()
        .min_by(|x, y| (x.chars().count().cmp(&y.chars().count()).reverse()))
        .unwrap()
        .len();

    let padding_left = " ".repeat(unwrapped_padding.0.into());

    for line in lines.iter() {
        let padding_length: usize = max_length - line.chars().count() + 1;
        let padding_right: String = " "
            .repeat(unwrapped_padding.1.into())
            .repeat(padding_length);
        println!("|{}{}{}|", padding_left, line, padding_right);
    }
}

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
        .get_matches();
    let language = matches.value_of("lang").unwrap_or("de");
    let url = &format!(
        "https://{}.wikipedia.org/api/rest_v1/page/random/summary",
        language
    );

    let request = reqwest::blocking::get(url);
    let json = match request {
        Ok(file) => file.text()?,
        Err(_error) => {
            println!("Netzwerkfehler.");
            return Ok(());
        }
    };

    if matches.is_present("raw") {
        print!("{}", json);
        return Ok(());
    }

    let article = serde_json::from_str::<Article>(&json).unwrap();

    println!("TITEL:\n{}", article.title);
    print_box(article.extract, Some(30), None);

    Ok(())
}
