use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use regex::Regex;
use lazy_static::lazy_static;
use itertools::Itertools;

const YIELD_INSTRUCTION: &str = "yield";
const ZERO: &str = "0";

lazy_static! {
    static ref LABEL_REGEX: Regex = Regex::new(r"^(\w+):").unwrap();
    static ref JUMP_REGEX: Regex = Regex::new(r"j (\w+)").unwrap();
}

fn main() {
    for arg in env::args().skip(1) {
        if let Err(e) = process_file(&arg) {
            eprintln!("Error processing \"{}\": {}", arg, e);
        }
    }
}

struct Label {
    label: String,
    line: String,
}

fn process_file(filename: &str) -> io::Result<()> {
    println!("Processing \"{}\"", filename);
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();

    let mut program: Vec<String> = Vec::new();
    let mut labels: Vec<Label> = Vec::new();
    let mut line_number: usize = 0;
    for line in lines {
        let line = line?;

        if let Some(captures) = LABEL_REGEX.captures(&line) {
            let label = captures.get(1).unwrap().as_str();
            labels.push(Label {
                label: label.to_owned(),
                line: line_number.to_string(),
            });
        } else if let Some(captures) = JUMP_REGEX.captures(&line) {
            let label = captures.get(1).unwrap().as_str();
            let previous_was_yield: bool = program.last()
                .map(|last| last == YIELD_INSTRUCTION)
                .unwrap_or(false);

            let mut label_is_zero = false;
            for l in labels.iter() {
                if l.line != ZERO {
                    // if we find a nonzero line, then our label cannot have been zero and we can stop iterating
                    break;
                } else if l.label == label {
                    // if we find our label and we haven't found a nonzero line, then our label must be zero
                    label_is_zero = true;
                    break;
                }
                // finally, if we don't find the label at all then it must come after this line, and therefore be zero
            }

            if previous_was_yield && !label_is_zero {
                // nuke the yield and sneak a - in front of the label
                program.pop();
                program.push(format!("j -{}", label));
            } else {
                // do nothing special
                program.push(line);
                line_number += 1;
            }
        } else {
            program.push(line);
            line_number += 1;
        }
    }


    let program: String = program.into_iter().join("\n");

    let mut file = File::create(filename)?;
    file.write_all(replace_label(program, &labels).as_bytes())?;
    file.flush()?;
    Ok(())
}

// this is sort of horrible and allocates a bunch
fn replace_label(mut line: String, labels: &Vec<Label>) -> String {
    for label in labels {
        line = line.replace(&label.label, &label.line);
    }
    line
}
