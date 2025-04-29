use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn process_srt_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let input_file = File::open(input_path)?;
    let reader = BufReader::new(input_file);
    let mut output_file = File::create(output_path)?;

    for line in reader.lines() {
        let mut line = line?;

        // only process lines that aren't just numbers (sequence numbers)
        // and aren't timestamp lines (containing -->)
        if !line.trim().chars().all(|c| c.is_numeric())
            && !line.contains("-->")
            && !line.trim().is_empty()
        {
            // remove periods that are at the end of words but not at the end of sentences
            let mut processed = String::new();
            let words: Vec<&str> = line.split_whitespace().collect();

            for (i, word) in words.iter().enumerate() {
                let mut cleaned_word = *word;

                // remove period if it's at the end of a word and:
                // (1) it's not the last word in the line, or
                // (2) the word is all uppercase
                if word.ends_with('.') && (i != words.len() - 1 || word == &word.to_uppercase()) {
                    cleaned_word = &word[..word.len() - 1];
                }

                processed.push_str(cleaned_word);
                if i != words.len() - 1 {
                    processed.push(' ');
                }
            }

            line = processed;
        }

        writeln!(output_file, "{}", line)?;
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: {} <input.srt> <output.srt>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    if let Err(e) = process_srt_file(input_path, output_path) {
        eprintln!("Error processing SRT file: {}", e);
        std::process::exit(1);
    }

    println!("SRT file processed successfully!");
}
