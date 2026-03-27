
use antlr_rust::{InputStream, common_token_stream::CommonTokenStream};
use bin_words::parser::{binwordslexer::BinWordsLexer, binwordsparser::BinWordsParser};
use std::io::Write;
use std::process::{Command, Stdio};
use std::error::Error;

fn main() {
    println!("Enter a string to parse:");

    // Get user input from stdin
    let mut input_string = String::new();
    std::io::stdin()
        .read_line(&mut input_string)
        .expect("Failed to read line");
    let input = InputStream::new(input_string.trim());

    // Create a TokenSource from the CharStream using the BinWords grammar
    let lexer = BinWordsLexer::new(input);

    // Obtain the tokens from the TokenSource as a TokenStream
    let tokens = CommonTokenStream::new(lexer);

    // Create a parser that parses the BinWords grammar
    let mut parser = BinWordsParser::new(tokens);

    // Execute the grammar from the 'main' nonterminal symbol
    let _tree = parser.main();

    println!("Enter a mode to show the tree:\n
        (1) T (Tree)
        (2) G (Gui)");
    let mut show_tree = String::new();
    std::io::stdin()
        .read_line(&mut show_tree)
        .expect("Failed to read line");

    print_tree(&show_tree.trim(), input_string.trim())
        .expect("Failed to print tree");

}

/// Prints the parsed tree in the selected mode
fn print_tree(mode: &str, input: &str) -> Result<(), Box<dyn Error>> {

    let arg_mode = match mode {
        "t" | "T" => "-tree",
        "g" | "G" => "-gui",
        _ => {
            println!("Invalid mode. Use t (tree) or g (gui).");
            return Ok(());
        }
    };

    let mut child = Command::new("antlr4-parse")
            .arg("grammars/BinWords.g4") // This should be selectable
            .arg("main")
            .arg(arg_mode)
            .stdin(Stdio::piped())
            .spawn()
            .expect("antlr4-parse failed to start");

    // Write the user's string directly into the Java tool's stdin
    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input.as_bytes())
            .expect("Failed to write to stdin");
    }

    let output = child.wait_with_output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        panic!(
            "antlr4-parse command failed with status {}.\n{}\n{}",
            output.status, stderr, stdout
        );
    }

    Ok(())
}
