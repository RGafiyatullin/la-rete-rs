#[macro_use]
extern crate failure;

use std::io::Read;

use json_utils::json::JsValue;
use la_rete::core::ruleset::*;
use la_rete::core::Matcher;
use la_rete::core::TrieBuildFailure;
use la_rete::json::parse_ruleset as parse_json_ruleset;

#[derive(Debug, Fail)]
enum Failure {
    #[fail(display = "Failure::Unimplemented: {}", _0)]
    Unimplemented(&'static str),

    #[fail(display = "Failure::RulesetParseError: {}", _0)]
    RulesetParseError(#[cause] serde_json::Error),

    #[fail(display = "Failure::TrieError: {}", _0)]
    TrieBuildFailure(#[cause] TrieBuildFailure),

    #[fail(display = "Failure::IoFailure: {}", _0)]
    IoFailure(#[cause] std::io::Error),

    #[fail(display = "Failure::RulesetUnspecified")]
    RulesetUnspecified,
}

fn main() -> () {
    if let Err(reason) = main_impl() {
        println!("{}", reason);
        std::process::exit(1)
    }
}

fn main_impl() -> Result<(), Failure> {
    let ruleset_file = std::env::args()
        .collect::<Vec<_>>()
        .get(1)
        .cloned()
        .ok_or(Failure::RulesetUnspecified)?;
    let ruleset = read_ruleset_from_file(&ruleset_file)?;
    println!("Ruleset parsed");
    let matcher = Matcher::create(ruleset).map_err(Failure::TrieBuildFailure)?;
    println!("Ruleset compiled");

    let _ = match_from_stdin(&matcher)?;
    Ok(())
}

fn read_ruleset_from_file(filename: &str) -> Result<Ruleset<JsValue, JsValue>, Failure> {
    let mut input = Vec::new();
    let mut file = std::fs::File::open(filename).map_err(Failure::IoFailure)?;
    let _ = file.read_to_end(&mut input).map_err(Failure::IoFailure)?;
    parse_json_ruleset(&input).map_err(Failure::RulesetParseError)
}

fn match_from_stdin(matcher: &Matcher<JsValue, JsValue>) -> Result<(), Failure> {
    let stdin = std::io::stdin();

    loop {
        let mut line = String::new();
        if stdin.read_line(&mut line).map_err(Failure::IoFailure)? == 0 {
            break;
        }
        let () = process_line(matcher, &line)?;
    }
    Ok(())
}

fn process_line(matcher: &Matcher<JsValue, JsValue>, line: &str) -> Result<(), Failure> {
    match serde_json::from_str::<JsValue>(line) {
        Ok(input_js_value) => println!("Match result: {:?}", matcher.lookup(&input_js_value)),
        Err(reason) => println!("Failed to parse as JSON: {:#?}", reason),
    }
    Ok(())
}
