use diom_tokens::SpanTokens;
use nom::error::ContextError;
use nom::error::ErrorKind;
use nom::error::FromExternalError;
use nom::error::ParseError;
use nom::Err;
use nom::Offset as _;
use std::fmt::from_fn;
use std::fmt::Debug;
use std::ops::Range;

#[derive(Clone, Eq, PartialEq)]
/// Error context for `ParserError`
pub enum DebugErrorKind {
  /// Static string added by the `context` function
  Context(&'static str),
  /// Indicates which character was expected by the `char` function
  Char(char),
  /// Error kind given by various nom parsers
  Nom(ErrorKind),
  /// Any external errors that have been thrown
  External(ErrorKind, String),
}

impl Debug for DebugErrorKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Context(c) => write!(f, "Context({c:?})"),
      Self::Char(c) => write!(f, "Char({c:?})"),
      Self::Nom(k) => write!(f, "Nom({k:?})"),
      Self::External(k, d) => write!(f, "External({k:?}, {d})"),
    }
  }
}

/// Similar to nom's `VerboseError` in that it creates a stack of errors\
/// but it also allows for custom error kinds that implement `Debug`.\
/// With some post processing, it can be used to display user friendly error messages
#[derive(Debug)]
pub struct DebugError<I> {
  pub(crate) errors: Vec<(I, DebugErrorKind)>,
}

impl<I> ParseError<I> for DebugError<I> {
  fn append(input: I, kind: ErrorKind, mut other: Self) -> Self {
    other.errors.push((input, DebugErrorKind::Nom(kind)));
    other
  }

  fn from_char(input: I, c: char) -> Self {
    Self {
      errors: vec![(input, DebugErrorKind::Char(c))],
    }
  }

  fn from_error_kind(input: I, kind: ErrorKind) -> Self {
    Self {
      errors: vec![(input, DebugErrorKind::Nom(kind))],
    }
  }
}

impl<I> ContextError<I> for DebugError<I> {
  fn add_context(input: I, ctx: &'static str, mut other: Self) -> Self {
    other.errors.push((input, DebugErrorKind::Context(ctx)));
    other
  }
}

impl<I, E: Debug> FromExternalError<I, E> for DebugError<I> {
  fn from_external_error(input: I, kind: ErrorKind, e: E) -> Self {
    Self {
      errors: vec![(input, DebugErrorKind::External(kind, format!("{e:?}")))],
    }
  }
}

impl<I> DebugError<I> {
  /// Alters the input type taken by the error
  pub fn map_input<I1>(self, mut f: impl FnMut(I) -> I1) -> DebugError<I1> {
    DebugError {
      errors: self
        .errors
        .into_iter()
        .map(|(input, kind)| (f(input), kind))
        .collect(),
    }
  }
}

impl<'a> From<DebugError<SpanTokens<'a>>> for DebugError<&'a str> {
  fn from(value: DebugError<SpanTokens<'a>>) -> Self {
    value.map_input(|input| input.origin)
  }
}

pub(crate) const LINE_LENGTH: usize = 80;

impl DebugError<&str> {
  /// Calculates the range within which the `index` fits in `input`
  pub(crate) fn range_for(input: &str, index: usize) -> Range<usize> {
    let mut start = input[..index].rfind('\n').unwrap_or(0);
    let mut end = input[start..].find('\n').unwrap_or(input.len());

    if end - start <= LINE_LENGTH {
      return start..end;
    }
    start = start
      .min(index.saturating_sub(LINE_LENGTH / 2))
      .min(end.saturating_sub(LINE_LENGTH));
    end = end
      .max(index.saturating_add(LINE_LENGTH / 2))
      .max(start.saturating_add(LINE_LENGTH));
    start..end
  }

  pub(crate) fn cursor_position(input: &str, rest: &str) -> (Range<usize>, usize) {
    let index = input.offset(rest);
    let Range { start, end } = Self::range_for(input, index);
    let offset = input[start..index].chars().count() + 2;
    (start..end, offset)
  }

  /// Displays a stack trace for the parsing input
  pub fn display<'a>(&'a self, input: &'a str) -> impl Debug + 'a {
    from_fn(move |f| {
      let mut iter = self.errors.iter();
      let Some((rest, kind)) = iter.next() else {
        return Ok(());
      };
      let (mut range, idx) = Self::cursor_position(input, rest);
      writeln!(f, "`{}`", &input[range.clone()])?;
      write!(f, "{:>idx$} {kind:?}", '^')?;

      for (rest, kind) in iter {
        let (nrange, idx) = Self::cursor_position(input, rest);
        if nrange == range {
          write!(f, "\n{:>idx$} {kind:?}", '^')?;
          continue;
        }
        range = nrange;

        writeln!(f, "\n\n`{}`", &input[range.clone()])?;
        write!(f, "{:>idx$} {kind:?}", '^')?;
      }
      Ok(())
    })
  }
}

pub fn display_err<'a>(err: Err<DebugError<&'a str>>, input: &'a str) -> impl Debug + 'a {
  from_fn(move |f| match &err {
    Err::Incomplete(n) => write!(f, "needed {n:?} more chars"),
    Err::Failure(e) => write!(f, "Failed with:\n{:?}", e.display(input)),
    Err::Error(e) => write!(f, "Errored with:\n{:?}", e.display(input)),
  })
}
