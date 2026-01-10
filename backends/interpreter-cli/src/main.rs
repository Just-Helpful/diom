//! Diom code evaluation via the `interpreter` backend.
use std::fs::read_to_string;
use std::io;
use std::path::Path;
use std::str::FromStr;

use clap::{Args, Parser};
use diom_interpreter::interpret_expr;
use diom_lexer::parse_tokens;
use diom_parser::expressions::parse_expression;
use diom_syntax::fmt::MultiDisplay;
use diom_tokens::SpanTokens;
use nom::{error::Error, Err};

/// Interprets and executes the Diom language
#[derive(Debug, Parser)]
#[command(version)]
#[command(arg_required_else_help = true)]
struct MainArgs {
  /// Program code to evaluate
  #[command(flatten)]
  source: SourceArgs,
}

#[derive(Debug, Args)]
#[group(required = true, multiple = false)]
struct SourceArgs {
  /// A path source starting with `/`, `./` or raw code
  source: Option<ProgramSource>,

  /// The code to evaluate
  #[arg(short, long)]
  eval: Option<String>,

  /// The path to a file to evaluate
  #[arg(short, long)]
  file: Option<String>,
}

/// The source to run the program from
#[derive(Debug, Clone)]
pub enum ProgramSource {
  /// Evaluate a string from the terminal
  Eval(String),
  /// Load a program from a file and evaluate it
  File(String),
}

impl FromStr for ProgramSource {
  type Err = &'static str;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use ProgramSource::*;
    let is_path = s.starts_with("/") || s.starts_with("./");
    let variant = if is_path { File } else { Eval };
    Ok(variant(s.to_owned()))
  }
}

impl From<SourceArgs> for ProgramSource {
  fn from(SourceArgs { source, eval, file }: SourceArgs) -> Self {
    let src_e = eval.map(ProgramSource::Eval);
    let src_f = file.map(ProgramSource::File);
    source
      .xor(src_e)
      .xor(src_f)
      .expect("Only one source to be specified")
  }
}

impl ProgramSource {
  pub fn fetch(self) -> io::Result<String> {
    use ProgramSource::*;
    match self {
      Eval(code) => Ok(code),
      File(path) => read_to_string(Path::new(&path)),
    }
  }
}

fn main() {
  let args = MainArgs::parse();

  let src = ProgramSource::from(args.source);
  let code = src.fetch().unwrap();

  let (input, tokens) = parse_tokens(code.as_str().into()).unwrap();
  assert!(
    input.is_empty(),
    "Input was not fulled lexed, remaining input = {input}",
  );

  let result = parse_expression::<Error<SpanTokens>>(SpanTokens::from(&tokens));
  let (input, expr) = match result {
    Ok(res) => res,
    Err(Err::Error(err) | Err::Failure(err)) => {
      // let message = convert_error(code, err);
      panic!("Input failed to parse, with the errors:\n{}", err)
    }
    Err(Err::Incomplete(num)) => panic!("Input required {:?} more tokens", num),
  };
  assert!(
    input.is_empty(),
    "Input was not fully parsed, remaining input = `{input}`",
  );
  println!("{code}");
  println!("{}", expr.display());

  let value = interpret_expr(expr).unwrap();
  println!("{value:?}");
}
