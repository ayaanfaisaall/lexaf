#[cfg(test)]
mod tests {
    use lexaf::lexer::Lexer; 
    use lexaf::tokens::{Token, StrIntr, SpannedToken, Span};
    use lexaf::error::LexafError;

    #[test]
    fn test_colon_and_urls() {
        let input = "ping https://www.ayaanfaisaall.cc";
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize().unwrap();
        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::Word("ping"), span: Span { start: 0, end: 4 } },
                SpannedToken { token: Token::Word("https://www.ayaanfaisaall.cc"), span: Span { start: 5, end: 33 } },
                SpannedToken { token: Token::EOF, span: Span { start: 33, end: 33 } },
            ]
        );
    }

    #[test]
    fn test_file_paths_and_dots() {
        let input = "git add . && cat ~/Downloads/abc/dc.jpg";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::Word("git"), span: Span { start: 0, end: 3 } },
                SpannedToken { token: Token::Word("add"), span: Span { start: 4, end: 7 } },
                SpannedToken { token: Token::Word("."), span: Span { start: 8, end: 9 } },
                SpannedToken { token: Token::AndAnd, span: Span { start: 10, end: 12 } },
                SpannedToken { token: Token::Word("cat"), span: Span { start: 13, end: 16 } },
                SpannedToken { token: Token::Word("~/Downloads/abc/dc.jpg"), span: Span { start: 17, end: 39 } },
                SpannedToken { token: Token::EOF, span: Span { start: 39, end: 39 } },
            ]
        );
    }

    #[test]
    fn test_variable_declaration() {
        let input = "let n1 = 43\n";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::Let, span: Span { start: 0, end: 3 } },
                SpannedToken { token: Token::Word("n1"), span: Span { start: 4, end: 6 } },
                SpannedToken { token: Token::Assign, span: Span { start: 7, end: 8 } },
                SpannedToken { token: Token::Num(43), span: Span { start: 9, end: 11 } },
                SpannedToken { token: Token::NewLine, span: Span { start: 11, end: 12 } },
                SpannedToken { token: Token::EOF, span: Span { start: 12, end: 12 } },
            ]
        );
    }

    #[test]
    fn test_for_loop_with_to() {
        let input = "for i in 0 to 10 { break }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::For, span: Span { start: 0, end: 3 } },
                SpannedToken { token: Token::Word("i"), span: Span { start: 4, end: 5 } },
                SpannedToken { token: Token::In, span: Span { start: 6, end: 8 } },
                SpannedToken { token: Token::Num(0), span: Span { start: 9, end: 10 } },
                SpannedToken { token: Token::To, span: Span { start: 11, end: 13 } },
                SpannedToken { token: Token::Num(10), span: Span { start: 14, end: 16 } },
                SpannedToken { token: Token::LBrc, span: Span { start: 17, end: 18 } },
                SpannedToken { token: Token::Break, span: Span { start: 19, end: 24 } },
                SpannedToken { token: Token::RBrc, span: Span { start: 25, end: 26 } },
                SpannedToken { token: Token::EOF, span: Span { start: 26, end: 26 } },
            ]
        );
    }

    #[test]
    fn test_math_evaluation_block() {
        let input = "${ 84 - (44 -43) * 34 }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::Eval, span: Span { start: 0, end: 1 } },
                SpannedToken { token: Token::LBrc, span: Span { start: 1, end: 2 } },
                SpannedToken { token: Token::Num(84), span: Span { start: 3, end: 5 } },
                SpannedToken { token: Token::Minus, span: Span { start: 6, end: 7 } },
                SpannedToken { token: Token::LPths, span: Span { start: 8, end: 9 } },
                SpannedToken { token: Token::Num(44), span: Span { start: 9, end: 11 } },
                SpannedToken { token: Token::Minus, span: Span { start: 12, end: 13 } },
                SpannedToken { token: Token::Num(43), span: Span { start: 13, end: 15 } },
                SpannedToken { token: Token::RPths, span: Span { start: 15, end: 16 } },
                SpannedToken { token: Token::Multiply, span: Span { start: 17, end: 18 } },
                SpannedToken { token: Token::Num(34), span: Span { start: 19, end: 21 } },
                SpannedToken { token: Token::RBrc, span: Span { start: 22, end: 23 } },
                SpannedToken { token: Token::EOF, span: Span { start: 23, end: 23 } },
            ]
        );
    }

    #[test]
    fn test_string_interpolation_and_escapes() {
        let input = r#"print "name: {name}" | awk '{print}'"#;
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::Print, span: Span { start: 0, end: 5 } },
                SpannedToken { token: Token::Str(vec![
                    StrIntr::Literal("name: "),
                    StrIntr::Variable("name"),
                ]), span: Span { start: 6, end: 20 } },
                SpannedToken { token: Token::Pipe, span: Span { start: 21, end: 22 } },
                SpannedToken { token: Token::Word("awk"), span: Span { start: 23, end: 26 } },
                SpannedToken { token: Token::Str(vec![
                    StrIntr::Literal("{print}")
                ]), span: Span { start: 27, end: 36 } },
                SpannedToken { token: Token::EOF, span: Span { start: 36, end: 36 } },
            ]
        );
    }

    #[test]
    fn test_comments_are_ignored() {
        let input = "print 1 # this is a comment\nprint 2";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::Print, span: Span { start: 0, end: 5 } },
                SpannedToken { token: Token::Num(1), span: Span { start: 6, end: 7 } },
                SpannedToken { token: Token::NewLine, span: Span { start: 27, end: 28 } },
                SpannedToken { token: Token::Print, span: Span { start: 28, end: 33 } },
                SpannedToken { token: Token::Num(2), span: Span { start: 34, end: 35 } },
                SpannedToken { token: Token::EOF, span: Span { start: 35, end: 35 } },
            ]
        );
    }

    #[test]
    fn test_logical_and_redirections() {
        let input = "okay o> out.txt e>> err.txt abco>>file < in.txt";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::Word("okay"), span: Span { start: 0, end: 4 } },
                SpannedToken { token: Token::RdrctOut, span: Span { start: 5, end: 7 } },
                SpannedToken { token: Token::Word("out.txt"), span: Span { start: 8, end: 15 } },
                SpannedToken { token: Token::AppendErr, span: Span { start: 16, end: 19 } },
                SpannedToken { token: Token::Word("err.txt"), span: Span { start: 20, end: 27 } },
                SpannedToken { token: Token::Word("abco"), span: Span { start: 28, end: 32 } },
                SpannedToken { token: Token::AppendBoth, span: Span { start: 32, end: 34 } },
                SpannedToken { token: Token::Word("file"), span: Span { start: 34, end: 38 } },
                SpannedToken { token: Token::RdrctIn, span: Span { start: 39, end: 40 } },
                SpannedToken { token: Token::Word("in.txt"), span: Span { start: 41, end: 47 } },
                SpannedToken { token: Token::EOF, span: Span { start: 47, end: 47 } },
            ]
        );
    }

    #[test]
    fn test_if_elif_else_flow() {
        let input = "if n1 -eq 43 { print \"yes\" } elif n1 -le 23 { print \"no\" } else { print \"maybe\" }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::If, span: Span { start: 0, end: 2 } },
                SpannedToken { token: Token::Word("n1"), span: Span { start: 3, end: 5 } },
                SpannedToken { token: Token::Word("-eq"), span: Span { start: 6, end: 9 } },
                SpannedToken { token: Token::Num(43), span: Span { start: 10, end: 12 } },
                SpannedToken { token: Token::LBrc, span: Span { start: 13, end: 14 } },
                SpannedToken { token: Token::Print, span: Span { start: 15, end: 20 } },
                SpannedToken { token: Token::Str(vec![StrIntr::Literal("yes")]), span: Span { start: 21, end: 26 } },
                SpannedToken { token: Token::RBrc, span: Span { start: 27, end: 28 } },
                SpannedToken { token: Token::Elif, span: Span { start: 29, end: 33 } },
                SpannedToken { token: Token::Word("n1"), span: Span { start: 34, end: 36 } },
                SpannedToken { token: Token::Word("-le"), span: Span { start: 37, end: 40 } },
                SpannedToken { token: Token::Num(23), span: Span { start: 41, end: 43 } },
                SpannedToken { token: Token::LBrc, span: Span { start: 44, end: 45 } },
                SpannedToken { token: Token::Print, span: Span { start: 46, end: 51 } },
                SpannedToken { token: Token::Str(vec![StrIntr::Literal("no")]), span: Span { start: 52, end: 56 } },
                SpannedToken { token: Token::RBrc, span: Span { start: 57, end: 58 } },
                SpannedToken { token: Token::Else, span: Span { start: 59, end: 63 } },
                SpannedToken { token: Token::LBrc, span: Span { start: 64, end: 65 } },
                SpannedToken { token: Token::Print, span: Span { start: 66, end: 71 } },
                SpannedToken { token: Token::Str(vec![StrIntr::Literal("maybe")]), span: Span { start: 72, end: 79 } },
                SpannedToken { token: Token::RBrc, span: Span { start: 80, end: 81 } },
                SpannedToken { token: Token::EOF, span: Span { start: 81, end: 81 } },
            ]
        );
    }

    #[test]
    fn test_while_loop_with_booleans() {
        let input = "while true { break } while false { }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::While, span: Span { start: 0, end: 5 } },
                SpannedToken { token: Token::True, span: Span { start: 6, end: 10 } },
                SpannedToken { token: Token::LBrc, span: Span { start: 11, end: 12 } },
                SpannedToken { token: Token::Break, span: Span { start: 13, end: 18 } },
                SpannedToken { token: Token::RBrc, span: Span { start: 19, end: 20 } },
                SpannedToken { token: Token::While, span: Span { start: 21, end: 26 } },
                SpannedToken { token: Token::False, span: Span { start: 27, end: 32 } },
                SpannedToken { token: Token::LBrc, span: Span { start: 33, end: 34 } },
                SpannedToken { token: Token::RBrc, span: Span { start: 35, end: 36 } },
                SpannedToken { token: Token::EOF, span: Span { start: 36, end: 36 } },
            ]
        );
    }

    #[test]
    fn test_all_comparison_operators() {
        let input = "a -lt b -ge c -gt d";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::Word("a"), span: Span { start: 0, end: 1 } },
                SpannedToken { token: Token::Word("-lt"), span: Span { start: 2, end: 5 } },
                SpannedToken { token: Token::Word("b"), span: Span { start: 6, end: 7 } },
                SpannedToken { token: Token::Word("-ge"), span: Span { start: 8, end: 11 } },
                SpannedToken { token: Token::Word("c"), span: Span { start: 12, end: 13 } },
                SpannedToken { token: Token::Word("-gt"), span: Span { start: 14, end: 17 } },
                SpannedToken { token: Token::Word("d"), span: Span { start: 18, end: 19 } },
                SpannedToken { token: Token::EOF, span: Span { start: 19, end: 19 } },
            ]
        );
    }

    #[test]
    fn test_mixed_logical_and_shell_operators() {
        let input = "if false || true && n1 -gt 5 { theme 4 }";
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                SpannedToken { token: Token::If, span: Span { start: 0, end: 2 } },
                SpannedToken { token: Token::False, span: Span { start: 3, end: 8 } },
                SpannedToken { token: Token::OrOr, span: Span { start: 9, end: 11 } },
                SpannedToken { token: Token::True, span: Span { start: 12, end: 16 } },
                SpannedToken { token: Token::AndAnd, span: Span { start: 17, end: 19 } },
                SpannedToken { token: Token::Word("n1"), span: Span { start: 20, end: 22 } },
                SpannedToken { token: Token::Word("-gt"), span: Span { start: 23, end: 26 } },
                SpannedToken { token: Token::Num(5), span: Span { start: 27, end: 28 } },
                SpannedToken { token: Token::LBrc, span: Span { start: 29, end: 30 } },
                SpannedToken { token: Token::Word("theme"), span: Span { start: 31, end: 36 } },
                SpannedToken { token: Token::Num(4), span: Span { start: 37, end: 38 } },
                SpannedToken { token: Token::RBrc, span: Span { start: 39, end: 40 } },
                SpannedToken { token: Token::EOF, span: Span { start: 40, end: 40 } },
            ]
        );
    }

    #[test]
    fn test_unclosed_string_error() {
        let input = r#"let a = "unclosed"#;
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();

        assert!(result.is_err());
        match result.unwrap_err() {
            LexafError::UnclosedDelimiter { delimiter, .. } => {
                assert_eq!(delimiter, "\"");
            },
            _ => panic!("Expected UnclosedDelimiter error"),
        }
    }

    #[test]
    fn test_unclosed_interpolation_error() {
        let input = r#"let a = "hello {name""#;
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();

        assert!(result.is_err());
        match result.unwrap_err() {
            LexafError::UnclosedDelimiter { delimiter, .. } => {
                assert_eq!(delimiter, "}");
            },
            _ => panic!("Expected UnclosedDelimiter error"),
        }
    }

    #[test]
    fn test_eval_block_oper_not_allowed() {
        let input = "${ 1 & 2 }";
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();

        assert!(result.is_err());
        match result.unwrap_err() {
            LexafError::OperNotAllowed { operator, .. } => {
                assert_eq!(operator, "&");
            },
            _ => panic!("Expected OperNotAllowed error"),
        }
    }

    #[test]
    fn test_eval_block_unexpected_token() {
        let input = "$ invalid { 1 + 1 }";
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();

        assert!(result.is_err());
        match result.unwrap_err() {
            LexafError::UnexpectedToken { token, .. } => {
                assert_eq!(token, "invalid");
            },
            _ => panic!("Expected UnexpectedToken error"),
        }
    }

    #[test]
    fn test_eval_block_unclosed_delimiter() {
        // Missing the closing `}`
        let input = "${ 1 + 2 ";
        let mut lexer = Lexer::new(input);
        let result = lexer.tokenize();

        assert!(result.is_err());
        match result.unwrap_err() {
            LexafError::UnclosedDelimiter { delimiter, .. } => {
                assert_eq!(delimiter, "}");
            },
            _ => panic!("Expected UnclosedDelimiter error"),
        }
    }
}
