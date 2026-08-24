#[cfg(test)]
mod tests {
    use lexaf::lexer::Lexer; 
    use lexaf::tokens::{Token, StrIntr};

    #[test]
    fn test_colon_and_urls() {
        let input = "ping https://www.ayaanfaisaall.cc";
        let tokens = Lexer::new(&input).tokenize();
        assert_eq!(
            tokens,
            vec![
            Token::Word(String::from("ping")),
            Token::Word(String::from("https://www.ayaanfaisaall.cc")),
            Token::EOF,
            ]
        );
    }

    #[test]
    fn test_file_paths_and_dots() {
        let input = "git add . && cat ~/Downloads/abc/dc.jpg";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Word(String::from("git")),
                Token::Word(String::from("add")),
                Token::Word(String::from(".")),
                Token::AndAnd,
                Token::Word(String::from("cat")),
                Token::Word(String::from("~/Downloads/abc/dc.jpg")),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_variable_declaration() {
        let input = "let n1 = 43\n";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Let,
                Token::Word(String::from("n1")),
                Token::Assign,
                Token::Word(String::from("43")),
                Token::NewLine,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_for_loop_with_to() {
        let input = "for i in 0 to 10 { break }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::For,
                Token::Word(String::from("i")),
                Token::In,
                Token::Word(String::from("0")),
                Token::To,
                Token::Word(String::from("10")),
                Token::LBrc,
                Token::Break,
                Token::RBrc,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_math_evaluation_block() {
        let input = "${ 84 - (44 -43) * 34 }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Eval,
                Token::LBrc,
                Token::Word(String::from("84")),
                Token::Minus,
                Token::LPths,
                Token::Word(String::from("44")),
                Token::Minus,
                Token::Word(String::from("43")),
                Token::RPths,
                Token::Multiply,
                Token::Word(String::from("34")),
                Token::RBrc,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_string_interpolation_and_escapes() {
        let input = r#"print "name: {name}" | awk '\{print\}'"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Print,
                Token::Str(vec![
                    StrIntr::Variable(String::from("name")),
                    StrIntr::Literal(String::from("name: {}")),
                ]),
                Token::Pipe,
                Token::Word(String::from("awk")),
                Token::Word(String::from("'{print}'")),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_comments_are_ignored() {
        let input = "print 1 # this is a comment\nprint 2";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Print,
                Token::Word(String::from("1")),
                Token::NewLine,
                Token::Print,
                Token::Word(String::from("2")),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_logical_and_redirections() {
        let input = "okay --l > jj --help>> hhff < in.txt";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Word(String::from("okay")),
                Token::Word(String::from("--l")),
                Token::RdrctOut,
                Token::Word(String::from("jj")),
                Token::Word(String::from("--help")),
                Token::Append,
                Token::Word(String::from("hhff")),
                Token::RdrctIn,
                Token::Word(String::from("in.txt")),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_if_elif_else_flow() {
        let input = "if n1 -eq 43 { print \"yes\" } elif n1 -le 23 { print \"no\" } else { print \"maybe\" }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::Word(String::from("n1")),
                Token::EqualTo,
                Token::Word(String::from("43")),
                Token::LBrc,
                Token::Print,
                Token::Str(vec![StrIntr::Literal(String::from("yes"))]),
                Token::RBrc,
                Token::Elif,
                Token::Word(String::from("n1")),
                Token::LessEqual,
                Token::Word(String::from("23")),
                Token::LBrc,
                Token::Print,
                Token::Str(vec![StrIntr::Literal(String::from("no"))]),
                Token::RBrc,
                Token::Else,
                Token::LBrc,
                Token::Print,
                Token::Str(vec![StrIntr::Literal(String::from("maybe"))]),
                Token::RBrc,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_while_loop_with_booleans() {
        let input = "while true { break } while false { }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::While,
                Token::True,
                Token::LBrc,
                Token::Break,
                Token::RBrc,
                Token::While,
                Token::False,
                Token::LBrc,
                Token::RBrc,
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_all_comparison_operators() {
        let input = "a -lt b -ge c -gt d";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::Word(String::from("a")),
                Token::LessThan,
                Token::Word(String::from("b")),
                Token::GreaterEqual,
                Token::Word(String::from("c")),
                Token::GreaterThan,
                Token::Word(String::from("d")),
                Token::EOF,
            ]
        );
    }

    #[test]
    fn test_mixed_logical_and_shell_operators() {
        let input = "if false || true && n1 -gt 5 { theme 4 }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize();

        assert_eq!(
            tokens,
            vec![
                Token::If,
                Token::False,
                Token::OrOr,
                Token::True,
                Token::AndAnd,
                Token::Word(String::from("n1")),
                Token::GreaterThan,
                Token::Word(String::from("5")),
                Token::LBrc,
                Token::Word(String::from("theme")),
                Token::Word(String::from("4")),
                Token::RBrc,
                Token::EOF,
            ]
        );
    }
}
