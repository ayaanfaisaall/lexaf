# lexaf

A simple zero-copy lexical analyzer (tokenizer) for `afsh` (a shell).

Tokenizing is the "dumbest but fastest" step in this shell's pipeline. `lexaf` doesn't know whether a word is an external binary, a shell builtin, or an argument—it simply reads raw text and categorizes it into structured tokens (words, language keywords, strings, punctuation, and operators) so the parser can make sense of it later.

## Features

* **Custom Shell Operators:** Native support for pipes (`|`), redirects (`<`, `>`, `>>`), and background execution (`&`).
* **Built-in Scripting Keywords:** Tokenizes custom language keywords like `let`, `print`, `if`, `elif`, `else`, `for`, `while`, `in`, `to`, and `break`.
* **String Interpolation:** Parses variables embedded directly inside strings using curly braces (e.g., `"my name is {name}"`).
* **Math & Logic:** Recognizes standard math operators (`+`, `-`, `*`, `/`, `%`, `^`).
* **No Standalone Parentheses:** By design, standard parentheses `()` are intentionally excluded from the general shell language and are only recognized as specific math tokens.

## Installation

Since the crate is published on crates.io, you can easily add it to your Rust project:

```bash
cargo add lexaf
```

## Usage

Using `lexaf` is incredibly simple. Just instantiate the `Lexer` with a string slice and call the `tokenize()` method.

```rust
use lexaf::{
    Lexer,
    Report,
};

fn main() {
    let any_str = String::from(r#"let a = "this is lexaf"; print "{a}" "#);
    let mut lexer = Lexer::new(&any_str);
    match lexer.tokenize() {
        Ok(tokens) => println!("{:#?}", tokens),
        Err(e) => {
            let error = Report::new(e).with_source_code(any_str.to_string());
            println!("{:?}", error);
        }
    }
}
```

### Output

```rust
[
    SpannedToken {
        token: Let,
        span: Span {
            start: 0,
            end: 3,
        },
    },
    SpannedToken {
        token: Word(
            "a",
        ),
        span: Span {
            start: 4,
            end: 5,
        },
    },
    SpannedToken {
        token: Assign,
        span: Span {
            start: 6,
            end: 7,
        },
    },
    SpannedToken {
        token: Str(
            [
                Literal(
                    "this is lexaf",
                ),
            ],
        ),
        span: Span {
            start: 8,
            end: 23,
        },
    },
    SpannedToken {
        token: SemiCln,
        span: Span {
            start: 23,
            end: 24,
        },
    },
    SpannedToken {
        token: Print,
        span: Span {
            start: 25,
            end: 30,
        },
    },
    SpannedToken {
        token: Str(
            [
                Variable(
                    "a",
                ),
            ],
        ),
        span: Span {
            start: 31,
            end: 36,
        },
    },
]
```

## Tokens

`lexaf` breaks down input into the following enum variants:

* **General:** `Word(String)`, `Str(Vec<StrIntr>)`
* **Control:** `NewLine`, `EOF`
* **Punctuation:** `SemiCln`, `Colon`, `Comma`, `LBrc {`, `RBrc }`, `LSqr [`, `RSqr ]`
* **Shell Operators:** `Pipe |`, `RdrctIn <`, `RdrctOut >`, `Append >>`, `Eval $`
* **Logical & Assignment:** `Assign =`, `AndAnd &&`, `OrOr ||`, `EqEq ==`, `Bang !`, `And &`
* **Keywords:** `Let`, `Print`, `If`, `Elif`, `Else`, `For`, `While`, `In`, `To`, `Break`, `True`, `False`
* **Math:** `Plus`, `Minus`, `Multiply`, `Divide`, `Modulo`, `Power`, `LPths (`, `RPths )`

## Custom Errors

`lexaf` uses `miette` for presenting the errors. A sample output is:

```rust
afsh::lexaf::unclosed_delimiter

  × unclosed delimiter: '}'
   ╭────
 1 │ "my name is {name "
   · ─────────┬─────────
   ·          ╰── '}' might not be closed properly
   ╰────
  help: close: '}' properly
```

## Spans

Every token emitted by `lexaf` is returned as a `SpannedToken`, which bundles the core token variant with a `Span` struct detailing its exact byte location in the original string:

```rust
pub struct Span {
    pub start: usize,
    pub end: usize,
}
```

This `Span` tracking is a critical foundation for the subsequent steps of the shell pipeline. It allows developers to:
* **Provide Rich Error Diagnostics:** Pinpoint exact character ranges where syntax errors or runtime issues occur (i-e, `miette`).
* **Preserve Source Mapping:** Trace interpreted or tokenized structures back to their origin in user-written scripts.
* **Enable Syntax Highlighting:** Accurately apply colorization and formatting rules based on the original token's exact location.

## Challenges

During the coding of first working version this lexer (August 2026), i had no internet and no connection to the outer world, so i coded all of this from my mind and a little bit offline documention i had. Hence it might not be perfectly optimized, but it does the work perfectly for which it was written.
I was at my homeland AJK, where government had disabled all sort of internet due to some public protests.

## License

MIT License
