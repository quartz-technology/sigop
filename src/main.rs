mod optimizer;
mod preprocessor;

use crate::optimizer::{build_combinations, find_optimization};
use crate::preprocessor::{try_preprocess, PreProcessError};
use clap::Parser;
use env_logger::Builder;
use error_stack::Report;
use log::{error, LevelFilter};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The function signature to optimize.
    #[clap(short, long)]
    signature: String,

    /// The maximum size of the suffix following the original function name.
    #[clap(short, long, default_value_t = 3)]
    length: u8,

    /// The number of zero-bytes you want to have at the beginning of the optimized function.
    #[clap(short, long, default_value_t = 2)]
    target: u8,
}

fn run(
    function_signature: &str,
    suffix_length: u8,
    optimization_target: u8,
) -> Result<(), Report<PreProcessError>> {
    let mut function = try_preprocess(function_signature)?;
    let combinations = build_combinations(suffix_length);

    function.name.push('_');
    find_optimization(
        function.name.as_str(),
        function.params.as_str(),
        &combinations,
        suffix_length,
        optimization_target,
    );

    Ok(())
}

fn main() {
    let mut builder = Builder::new();

    builder.filter_level(LevelFilter::Info);
    builder.parse_default_env();
    builder.init();

    let cli = Cli::parse();

    match run(cli.signature.as_str(), cli.length, cli.target) {
        Ok(_) => {}
        Err(err) => match err.current_context() {
            PreProcessError::InvalidFunctionSignatureParenthesis(msg) => {
                error!("{msg}")
            }
            PreProcessError::InvalidFunctionSignatureParsing(msg) => {
                error!("{msg}")
            }
            PreProcessError::ErrorRegexParsing(msg) => {
                error!("{msg}")
            }
        },
    }
}
