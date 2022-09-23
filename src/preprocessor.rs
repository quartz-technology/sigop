use error_stack::{Report, Result};
use regex::Regex;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Default)]
pub struct FunctionSignature {
    pub name: String,
    pub params: String,
}

#[derive(Debug)]
pub enum PreProcessError {
    InvalidFunctionSignatureParenthesis(String),
    InvalidFunctionSignatureParsing(String),
    ErrorRegexParsing(String),
}

impl Display for PreProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PreProcessor error: could not preprocess initial function signature")
    }
}

impl Error for PreProcessError {}

pub fn try_preprocess(function_signature: &str) -> Result<FunctionSignature, PreProcessError> {
    let cleaned_function_signature = remove_whitespaces(function_signature);
    try_validate(cleaned_function_signature.as_str())?;
    let function = try_parse(cleaned_function_signature.as_str())?;

    Ok(function)
}

// Tries to parse the function signature and splits the name from the parameters.
fn try_parse(function_signature: &str) -> Result<FunctionSignature, PreProcessError> {
    let re = Regex::new(r"(.*?)\((.*)").unwrap();
    let caps = re.captures(function_signature);

    match caps {
        None => {
            Err(Report::new(
                PreProcessError::
                ErrorRegexParsing(
                    "could not parse function signature correctly".to_string(),
                )
            ))
        }
        Some(captures) => {
            match captures.len() == 3 {
                true => Ok(FunctionSignature {
                    name: captures.get(1).unwrap().as_str().to_string(),
                    params: format!("({}", captures.get(2).unwrap().as_str()),
                }),
                false => Err(Report::new(
                    PreProcessError::InvalidFunctionSignatureParsing(
                        "function signature doesn't have a valid structure (should be function_name(function_parameters))".to_string(),
                    )
                )),
            }
        }
    }
}

/// Used to remove whitespaces from the given function signature.
fn remove_whitespaces(function_sig: &str) -> String {
    function_sig
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect()
}

/// Tries to validate that the initial function signature is valid for further usage.
fn try_validate(function_sig: &str) -> Result<(), PreProcessError> {
    verify_parenthesis(function_sig)?;

    Ok(())
}

/// Makes sure there are no issues with parenthesis in the initial function signature.
fn verify_parenthesis(function_sig: &str) -> Result<(), PreProcessError> {
    if !function_sig.ends_with(')') {
        return Err(Report::new(
            PreProcessError::InvalidFunctionSignatureParenthesis(
                "function signature doesn't end with a closing parenthesis".to_string(),
            ),
        ));
    }

    if function_sig.matches('(').count() != function_sig.matches(')').count() {
        return Err(Report::new(
            PreProcessError::InvalidFunctionSignatureParenthesis(
                "function signature doesn't have the same number of opening and closing parenthesis".to_string(),
            ),
        ));
    }

    Ok(())
}
