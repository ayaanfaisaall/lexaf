# lexaf

A simple lexical analyzer (tokenizer) for `afsh` (a shell).

Tokenizing is the "dumbest but fastest" step in this shell's pipeline. `lexaf` doesn't know whether a word is an external binary, a shell builtin, or an argument—it simply reads raw text and categorizes it into structured tokens (words, language keywords, strings, punctuation, and operators) so the parser can make sense of it later.

## Features

* **Custom Shell Operators:** Native support for pipes (`|`), redirects (`<`, `>`, `>>`), and background execution (`&`).
* **Built-in Scripting Keywords:** Tokenizes custom language keywords like `let`, `print`, `if`, `elif`, `else`, `for`, `while`, `in`, `to`, and `break`.
* **String Interpolation:** Parses variables embedded directly inside strings using curly braces (e.g., `"my name is {name}"`).
* **Math & Logic:** Recognizes standard math operators (`+`, `-`, `*`, `/`, `%`, `^`) and bash-style equality flags (`-eq`, `-le`, `-lt`, `-ge`, `-gt`).
* **No Standalone Parentheses:** By design, standard parentheses `()` are intentionally excluded from the general shell language and are only recognized as specific math tokens.

## Installation

Since the crate is published on crates.io, you can easily add it to your Rust project:

```bash
cargo add lexaf
```

## Usage

Using `lexaf` is incredibly simple. Just instantiate the `Lexer` with a string slice and call the `tokenize()` method.

```rust
use lexaf::Lexer;

fn main() {
    let any_str = String::from(r#"let a = "this is lexaf"; print "{a}" "#);
    let mut lexer = Lexer::new(&any_str);
    let tokens = lexer.tokenize();
    println!("{:?}", tokens);
}
```

### Output

```rust
[Let, Word("a"), Assign, Str([Literal("this is lexaf")]), SemiCln, Print, Str([Variable("a"), Literal("{}")]), EOF]
```

## Tokens

`lexaf` breaks down input into the following enum variants:

* **General:** `Word(String)`, `Str(Vec<StrIntr>)`
* **Control:** `NewLine`, `EOF`
* **Punctuation:** `SemiCln`, `Colon`, `Comma`, `LBrc {`, `RBrc }`, `LSqr [`, `RSqr ]`
* **Shell Operators:** `Pipe |`, `RdrctIn <`, `RdrctOut >`, `Append >>`, `Eval $`
* **Logical & Assignment:** `Assign =`, `AndAnd &&`, `OrOr ||`, `EqEq ==`, `Bang !`, `And &`
* **Keywords:** `Let`, `Print`, `If`, `Elif`, `Else`, `For`, `While`, `In`, `To`, `Break`, `True`, `False`
* **Comparisons:** `EqualTo -eq`, `LessEqual -le`, `LessThan -lt`, `GreaterEqual -ge`, `GreaterThan -gt`
* **Math:** `Plus`, `Minus`, `Multiply`, `Divide`, `Modulo`, `Power`, `LPths (`, `RPths )`

## Challenges

During the coding of this lexer, i had no internet and no connection to the outer world, so i coded all of this from my mind and a little bit offline documention i had. Hence it might not be perfectly optimized, but it does the work perfectly for which it was written.
I was at my homeland AJK, where government had disabled all sort of internet due to some public protests.

## License

MIT License
