extern mod std;
use std::getopts::*;

// A Query consists of a list of words that all must match
struct Query {
  words: ~[~str]
}

// Only used for tests
fn parse(q: &str) -> Query {
  let mut words: ~[~str] = ~[];
  for str::each_word(q) |word| {
    words.push(word.to_owned());
  }
  Query { words: words }
}


impl Query {
  fn matches(&self, string: &str) -> bool {
    for self.words.each |&word| {
      if (!word_matches(string, word)) {
        return false;
      }
    }
    return true;
  }
}

fn word_matches(string: &str, word: &str) -> bool {
  let mut pos = 0;
  while (true) {
    // Find substrings
    match str::find_str_from(string, word, pos) {
      Some(idx) =>
        if (word_matches_at(string, word, idx)) {
          return true;
        } else {
          pos = idx + 1;
        },
      None => break
    }
  }
  return false;
}

fn word_matches_at(string: &str, word: &str, pos: uint) -> bool {
  at_boundary(string, pos) || is_capitalized(word)
}

fn at_boundary(string: &str, pos: uint) -> bool {
  if pos == 0 { return true }
  let prev = string.char_at(pos - 1);
  !char::is_alphanumeric(prev)
}

fn is_capitalized(string: &str) -> bool {
  char::is_uppercase(string.char_at(0))
}

#[test]
fn test_basic() {
  let query = parse("h");
  assert!(query.matches("hello"));
  assert!(!query.matches("world"));
}

#[test]
fn test_not_in_middle() {
  let query = parse("h");
  assert!(!query.matches("blah"));
}

#[test]
fn test_after_non_numeric() {
  let query = parse("h");
  assert!(query.matches("blah/hello"));
}

#[test]
fn test_multiple() {
  let query = parse("core Art");
  assert!(query.matches("vendor/core/Artifact.pm"));
  assert!(!query.matches("core"));
}

#[test]
fn test_empty() {
  let query = parse("");
  assert!(query.matches("foobar"));
}

#[test]
fn test_case() {
  let query = parse("Con");
  assert!(query.matches("MainController"));
}

// MAIN
fn main() {
  let args = os::args();

  let opts = ~[
    optopt("limit"),
    optopt("manifest"),
    optflag("dotfiles"),
    optflag("no-dotfiles"),
  ];

  let matches = match(getopts(vec::tail(args), opts)) {
    result::Ok(m) => { m }
    result::Err(f) => { fail!(fail_str(f)) }
  };

  let limit_opt = opt_maybe_str(&matches, "limit").chain(|s| int::from_str(s));
  let limit = limit_opt.get_or_default(20);

  let manifest_opt = opt_maybe_str(&matches, "manifest")
    .map(|&s| io::file_reader(&path::Path(s)).unwrap());
  let files = manifest_opt.get_or_default(io::stdin());

  let query = Query { words: copy matches.free };
  let mut count = 0;

  while (true) {
    let line = files.read_line();
    if (line.len() == 0) { break; }

    if (query.matches(line)) {
      println(line);
      count += 1;

      if (count >= limit) { break; }
    }
  }
}

