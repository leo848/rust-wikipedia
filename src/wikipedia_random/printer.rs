
pub struct Printer {
    width: Option<u16>,
    padding: Option<usize>,
    alignment: Option<Alignment>,
}

impl Printer {

    pub fn new(alignment: Option<Alignment>, width: Option<u16>, padding: Option<usize>) -> Self {

        Printer {alignment, width, padding }

    }

    pub fn print_error(self, message: String) {
        println!("Error occured: {}", message);
    }

    pub fn print_message(self, title: &str, text: &str) {

        let padding_size = self.padding.unwrap_or(1);
        let alignment = self.alignment.unwrap_or(Alignment::LEFT);
    
        let words = text.split(" ");
        let mut lines: Vec<String> = Vec::new();
    
        lines.push("\x1b[1m".to_string() + title + "\x1b[0m");
        lines.push(String::new());
    
        for word in words.into_iter() {
            if lines.len() == 2 {
                lines.push(word.to_string());
                continue;
            }
            let last_length = lines.last().expect("No lines").chars().count();
            if last_length + word.len() < self.width.unwrap_or(50).into() || last_length == 0 {
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
    
            let padding: (usize, usize) = get_alignment_padding(alignment, length_diff, padding_size);
    
            let padding_left = " ".repeat(padding.0);
            let padding_right = " ".repeat(padding.1);
    
            println!("│{}{}{}│", padding_left, line, padding_right);
        }
        println!("╰{}╯", vertical_border);

    }

    
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Alignment {
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

fn get_alignment_padding(alignment:Alignment, length_diff: usize, padding_size: usize) -> (usize, usize) {
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



