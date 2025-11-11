use {
    clap::Parser,
    clap_cargo::style::CLAP_STYLING,
    idna_cli::{Domain, print_csv_result, read_lines},
    indexmap::IndexMap,
    std::path::PathBuf,
};

const EXIT_CODE_INVALID_OUTPUT_FORMAT: i32 = 1;
const EXIT_CODE_INPUT_FILES_DONT_EXIST: i32 = 2;

const OUTPUT_FORMATS: &[&str] = &["csv", "json", "json-pretty", "rust", "rust-pretty"];

#[derive(Parser)]
#[command(name = "idna", about, version, max_term_width = 80, styles = CLAP_STYLING)]
struct Cli {
    /// Decode IDNA ASCII input to Unicode
    #[arg(short, long)]
    decode: bool,

    /// One or more files
    #[arg(short, long)]
    files: Vec<PathBuf>,

    /// Output format (csv, json, json-pretty, rust, rust-pretty)
    #[arg(short, long, value_name = "FORMAT", default_value = "csv")]
    output: String,

    /// One or more domains
    domains: Vec<String>,
}

#[allow(clippy::too_many_lines)]
fn main() {
    let cli = Cli::parse();

    // Error if output format is invalid
    if !OUTPUT_FORMATS.contains(&cli.output.as_str()) {
        eprintln!(
            "Invalid output format: {:?}; valid output formats: {}",
            cli.output,
            OUTPUT_FORMATS.join(", "),
        );
        std::process::exit(EXIT_CODE_INVALID_OUTPUT_FORMAT);
    }

    // Immediately error with a helpful message if any path(s) do not exist
    let mut dont_exist = vec![];
    for path in &cli.files {
        if !path.exists() {
            dont_exist.push(path.clone());
        }
    }
    if !dont_exist.is_empty() {
        eprintln!(
            "Paths do not exist: {}",
            dont_exist
                .iter()
                .map(|x| format!("{}", x.display()))
                .collect::<Vec<_>>()
                .join(", ")
        );
        std::process::exit(EXIT_CODE_INPUT_FILES_DONT_EXIST);
    }

    if cli.output == "csv" {
        // Print header
        if cli.decode {
            println!("\"ASCII\",\"Unicode\",\"Errors\"");
        } else {
            println!("\"Unicode\",\"ASCII\",\"Errors\"");
        }

        // Process arguments
        for domain in &cli.domains {
            print_csv_result(domain, cli.decode);
        }

        // Process files
        for path in &cli.files {
            match read_lines(path) {
                Ok(lines) => {
                    for line in lines {
                        match line {
                            Ok(line) => {
                                print_csv_result(&line, cli.decode);
                            }
                            Err(_e) => {}
                        }
                    }
                }
                Err(_e) => {}
            }
        }
    } else {
        let mut results = IndexMap::new();

        // Process arguments
        if !cli.domains.is_empty() {
            let mut arg_results = IndexMap::new();
            for domain in &cli.domains {
                arg_results.insert(domain.clone(), Domain::new(domain, cli.decode));
            }
            results.insert(String::from("Arguments"), arg_results);
        }

        // Process files
        if !cli.files.is_empty() {
            for path in &cli.files {
                let mut file_results = IndexMap::new();
                match read_lines(path) {
                    Ok(lines) => {
                        for line in lines {
                            match line {
                                Ok(domain) => {
                                    file_results
                                        .insert(domain.clone(), Domain::new(&domain, cli.decode));
                                }
                                Err(_e) => {}
                            }
                        }
                    }
                    Err(_e) => {}
                }
                results.insert(format!("File: {:?}", path.display()), file_results);
            }
        }

        // Print results
        match cli.output.as_str() {
            "json" => {
                println!("{}", serde_json::to_string(&results).unwrap());
            }
            "json-pretty" => {
                println!("{}", serde_json::to_string_pretty(&results).unwrap());
            }
            "rust" => {
                println!("{results:?}");
            }
            "rust-pretty" => {
                println!("{results:#?}");
            }
            _ => {
                eprintln!(
                    "Invalid output format: {:?}; valid output formats: {}",
                    cli.output,
                    OUTPUT_FORMATS.join(", "),
                );
                std::process::exit(EXIT_CODE_INVALID_OUTPUT_FORMAT);
            }
        }
    }
}
