mod optimizer;
mod preprocessor;

use crate::optimizer::{build_combinations, find_optimization};
use crate::preprocessor::{try_preprocess, PreProcessError};
use env_logger::Builder;
use error_stack::Report;
use log::{error, LevelFilter};

fn run(
    function_signature: &str,
    suffix_length: u8,
    optimization_target: u8,
    _debug: bool,
) -> Result<(), Report<PreProcessError>> {
    let mut function = try_preprocess(function_signature)?;
    let combinations = build_combinations(suffix_length);

    function.name.push('_');
    find_optimization(
        function.name.as_str(),
        function.params.as_str(),
        &combinations,
        optimization_target,
    );

    Ok(())
}

fn main() {
    let mut builder = Builder::new();

    builder.filter_level(LevelFilter::Info);
    builder.parse_default_env();
    builder.init();

    let function_signature = "myFunction(address)";
    let suffix_length = 4;
    let optimization_target = 3;

    match run(
        function_signature,
        suffix_length,
        optimization_target,
        false,
    ) {
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
