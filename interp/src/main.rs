//! This is an entire parser and interpreter for a dynamically-typed Rust-like expression-oriented
//! programming language. See `sample.nrs` for sample source code.
//! Run it with the following command:
//! cargo run --example nano_rust -- examples/sample.nrs

use ariadne::{sources, Color, Label, Report, ReportKind};
use chumsky::prelude::*;
use std::{env,fs};

use crate::{parser::funcs_parser, runner::eval_expr};

pub type Span = SimpleSpan<usize>;

mod common;
mod intrinsic;
mod lexer;
mod parser;
mod runner;
mod symbol;

fn main() {
    let filename = env::args().nth(1).expect("Expected file argument");

    let src = fs::read_to_string(&filename).expect("Failed to read file");

    let (tokens, mut errs) = lexer::lexer().parse(src.as_str()).into_output_errors();
    println!("{:?}", tokens);
    let parse_errs = if let Some(tokens) = &tokens {
        let (ast, parse_errs) = funcs_parser()
            .map_with_span(|ast, span| (ast, span))
            .parse(tokens.as_slice().spanned((src.len()..src.len()).into()))
            .into_output_errors();
        println!("{:#?}", ast);
        if let Some((funcs, file_span)) = ast.filter(|_| errs.len() + parse_errs.len() == 0) {
            println!("{:#?}", funcs.keys());
            if let Some(main) = funcs.get("main") {
                if main.args.len() != 0 {
                    errs.push(Rich::custom(
                        main.span,
                        format!("The main function cannot have arguments"),
                    ))
                } else {
                    match eval_expr(&main.body, &funcs, &mut Vec::new()) {
                        Ok(val) => println!("Return value: {}", val),
                        Err(e) => errs.push(Rich::custom(e.span, e.msg)),
                    }
                }
            } else {
                errs.push(Rich::custom(
                    file_span,
                    format!("Programs need a main function but none was found"),
                ));
            }
        }

        parse_errs
    } else {
        Vec::new()
    };

    errs.into_iter()
        .map(|e| e.map_token(|c| c.to_string()))
        .chain(
            parse_errs
                .into_iter()
                .map(|e| e.map_token(|tok| tok.to_string())),
        )
        .for_each(|e| {
            Report::build(ReportKind::Error, filename.clone(), e.span().start)
                .with_message(e.to_string())
                .with_label(
                    Label::new((filename.clone(), e.span().into_range()))
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .with_labels(e.contexts().map(|(label, span)| {
                    Label::new((filename.clone(), span.into_range()))
                        .with_message(format!("while parsing this {}", label))
                        .with_color(Color::Yellow)
                }))
                .finish()
                .print(sources([(filename.clone(), src.clone())]))
                .unwrap()
        });
}
