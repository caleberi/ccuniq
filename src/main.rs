/// uniq utility reads the specified input_file comparing adjacent lines, and writes a copy of each unique input line to the output_file.
/// If input_file is a single dash (‘-’) or absent, the standard input is read.  If output_file is absent, standard output is used for output.
/// The second and succeeding copies of identical adjacent input lines are not written.
/// Repeated lines in the input will not be detected if they are not adjacent, so it may be necessary to sort the files first.
//TODO: Create a commandline config to figure out how to do the line parsing
use std::{
    collections::HashMap,
    env::Args,
    fs::{self, File},
    io::{stdin, stdout, Write},
    process::exit,
};
mod config;
mod constant;

fn write_outstream(mut outstream: Option<Result<File, std::io::Error>>, buffer: Vec<u8>) {
    if let Some(file) = outstream.take() {
        file.unwrap()
            .write_all(&buffer.as_slice())
            .unwrap_or_else(move |err| {
                println!("Error: {}", err);
                exit(1);
            });
        return;
    }
    stdout()
        .write_all(&buffer.as_slice())
        .unwrap_or_else(move |err| {
            println!("Error: {}", err);
            exit(1);
        });
}

fn reset_buffer(buffer: &mut Vec<u8>) {
    if buffer.len() > 0 {
        buffer.clear()
    }
}

fn main() {
    let args: Args = std::env::args();
    let config = config::Config::new(args);
    let input_content = if config.input_file.is_some() {
        let file_name = config.input_file.as_ref().unwrap();
        std::fs::read_to_string(file_name).unwrap()
    } else {
        stdin().lines().into_iter().fold(String::new(), |acc, s| {
            let mut new_acc = String::from(acc.as_str());
            new_acc.push_str(s.unwrap().as_str());
            new_acc.push_str("\n");
            new_acc
        })
    };

    let mut buffer: Vec<u8> = Vec::new();
    let mut dictionary: HashMap<&str, isize> = HashMap::new();
    let lines: Vec<&str> = input_content
        .split('\n')
        .into_iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect();
    lines.iter().for_each(|line| {
        let entry = dictionary.entry(line).or_insert(0);
        *entry += 1;
    });

    let outstream = if let Some(file_path) = &config.output_file {
        Some(fs::File::create(file_path))
    } else {
        None
    };

    handle_default_operation(&config, &dictionary, &mut buffer);
    handle_count_operation(&config, &mut buffer, &dictionary);
    handle_repeat_operation(&config, &mut buffer, &dictionary);
    handle_unique_operation(config, &mut buffer, dictionary);

    write_outstream(outstream, buffer);
}

fn handle_default_operation(
    config: &config::Config<'_>,
    dictionary: &HashMap<&str, isize>,
    buffer: &mut Vec<u8>,
) {
    if !config.get_flag_status() {
        for (line, _) in dictionary {
            let data = format!("{}\n", *line);
            buffer.extend_from_slice(data.as_bytes())
        }
        _ = buffer.pop(); // remove the terminating "\n"
    }
}

fn handle_unique_operation(
    config: config::Config<'_>,
    buffer: &mut Vec<u8>,
    dictionary: HashMap<&str, isize>,
) {
    if config.unique {
        reset_buffer(buffer);
        for (line, frequency) in &dictionary {
            if *frequency == 1 {
                let data = format!("{}\n", *line);
                buffer.extend_from_slice(data.as_bytes());
            }
        }
        _ = buffer.pop();
    }
}

fn handle_repeat_operation(
    config: &config::Config<'_>,
    buffer: &mut Vec<u8>,
    dictionary: &HashMap<&str, isize>,
) {
    if config.repeated {
        reset_buffer(buffer);
        for (line, frequency) in dictionary {
            if *frequency > 1 {
                let data = format!("{}\n", *line);
                buffer.extend_from_slice(data.as_bytes());
            }
        }
        _ = buffer.pop();
    }
}

fn handle_count_operation(
    config: &config::Config<'_>,
    buffer: &mut Vec<u8>,
    dictionary: &HashMap<&str, isize>,
) {
    if config.count {
        reset_buffer(buffer);
        for (line, frequency) in dictionary {
            let data = format!("{} {}\n", *frequency, *line);
            buffer.extend_from_slice(data.as_bytes())
        }
        _ = buffer.pop();
    }
}
