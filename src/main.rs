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

#[allow(dead_code)]
enum Alignment {
    LEFT,
    MIDDLE,
    RIGHT,
}

fn max_length(strings: &Vec<String>) -> usize {
    let mut max_length = 0usize;
    for string in strings {
        let length = string.chars().count();
        if length > max_length {
            max_length = length
        }
    }
    max_length
}

fn get_alignment_padding(alignment: &Alignment, length_diff: usize, padding_size: usize) -> (usize, usize) {
    let float_length_diff: f64 = length_diff as f64;
    let half_float_length = float_length_diff / 2.0;
    let padding = match alignment {
        Alignment::LEFT => (0, length_diff),
        Alignment::MIDDLE => (
            half_float_length.ceil() as usize,
            half_float_length.floor() as usize,
        ),
        Alignment::RIGHT => (length_diff, 0),
    };
    (padding.0 + padding_size, padding.1 + padding_size)
}

fn print_box(
    string: String,
    title: String,
    width: Option<u16>,
    padding: Option<usize>,
    alignment: Option<Alignment>,
) {
    let padding_size = padding.unwrap_or(1);
    let alignment = alignment.unwrap_or(Alignment::LEFT);

    let words = string.split(" ");
    let mut lines: Vec<String> = Vec::new();

    lines.push("\x1b[1m".to_string() + &title + "\x1b[0m");
    lines.push(String::new());

    for word in words {
        if lines.len() == 2 {
            lines.push(word.to_string());
            continue;
        }
        let last_length = lines.last().expect("No lines").chars().count();
        if last_length + word.len() < width.unwrap_or(50).into() || last_length == 0 {
            *lines.last_mut().unwrap() = lines.last_mut().unwrap().to_owned() + " " + word;
        } else {
            lines.push(word.to_string());
        }
    }

    let max_length = max_length(&lines);
    let vertical_border = "─".repeat(max_length + padding_size * 2);

    println!("╭{}╮", vertical_border);
    for line in lines.iter() {
        let mut length_diff: usize = max_length - line.chars().count();
        if str::ends_with(line, "\x1b[0m") {
            length_diff += 8;
        }

        let padding: (usize, usize) = get_alignment_padding(&alignment, length_diff, padding_size);

        let padding_left = " ".repeat(padding.0);
        let padding_right = " ".repeat(padding.1);

        println!("│{}{}{}│", padding_left, line, padding_right);
    }
    println!("╰{}╯", vertical_border);
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

    let alignment: Option<Alignment> = match matches.value_of("align") {
        None => None,
        Some("left" | "l") => Some(Alignment::LEFT),
        Some("middle" | "m") => Some(Alignment::MIDDLE),
        Some("right" | "r") => Some(Alignment::RIGHT),
        Some(&_) => None,
    };

    let width: Option<u16> = match matches.value_of("width") {
        None => None,
        Some(s) => match s.parse::<u16>() {
            Ok(n) => Some(n),
            Err(_e) => None,
        },
    };

    let padding: Option<usize> = match matches.value_of("padding") {
        None => None,
        Some(s) => match s.parse::<usize>() {
            Ok(n) => Some(n),
            Err(_e) => None,
        },
    };

    print_box(article.extract, article.title, width, padding, alignment);

    Ok(())
}
