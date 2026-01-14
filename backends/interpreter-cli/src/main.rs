//! Diom code evaluation via the `interpreter` backend.
use clap::{Args, Parser};
use diom_info_traits::InfoMap;
use diom_interpreter::Eval;
use diom_lexer::parse_tokens;
use diom_parser::expressions::parse_expression;
use diom_syntax::fmt::CustomDisplay;
use diom_tokens::SpanTokens;
use nom::Err;
use nom_language::error::VerboseError;
use std::fs::read_to_string;
use std::io;
use std::path::Path;
use std::str::FromStr;

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
  println!("\n# Lexed Tokens");
  println!("{}", SpanTokens::from(&tokens));

  let result = parse_expression::<VerboseError<SpanTokens>>(SpanTokens::from(&tokens));
  let (input, expr) = match result {
    Ok(res) => res,
    Err(Err::Error(err) | Err::Failure(err)) => {
      panic!("Input failed to parse, with the errors:\n{}", err)
    }
    Err(Err::Incomplete(num)) => panic!("Input required {:?} more tokens", num),
  };
  assert!(
    input.is_empty(),
    "Input was not fully parsed, remaining input = `{input}`",
  );
  let expr = expr.map(|src| unsafe { src.str_range(&code) }.unwrap());
  println!("\n# Parsed Syntax");
  println!("{code}");
  println!("{}", expr.display());

  let value = expr.eval().unwrap();
  println!("{value:?}");
}
