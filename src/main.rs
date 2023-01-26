static TIMESTAMP: &'static str = include_str!(concat!(env!("OUT_DIR"), "/timestamp.txt"));
static VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/version.txt"));
static TARGET: &'static str = include_str!(concat!(env!("OUT_DIR"), "/target.txt"));

mod parser;

use parser::combinators::Combinators;

fn main() {
    let version = format!(
        "{}{}",
        env!("CARGO_PKG_VERSION"),
        if VERSION != "" { " nightly" } else { "" }
    );
    println!("Majestic Lisp Refactored v{} {}", version, TARGET);
    if VERSION != "" {
        println!("Build {} {}", VERSION, TIMESTAMP);
    }
    println!("Copyright (c) 2020-2023 Lucas S. Vieira");

    let str = "\
(a . (b . (c d e)))

(defn square (x)
  (* x x))

[1 2 3 'a 'b 'c]
";

    use chumsky::Parser;

    println!("Example 1: {:#?}", Combinators::parser().parse(str));

    println!("Parsing majestic files...");
    let files = [
        "basic.maj",
        "bootstrap.maj",
        "church.maj",
        "cps.maj",
        "dynamic-scope.maj",
        "helper.maj",
        "lazy.maj",
        "lightweight-table.maj",
        "lightweight-table-alt.maj",
        "metacircular.maj",
        "prolog.maj",
        "unicode.maj",
        "word-equation-scheme.maj",
    ];

    for file in files {
        use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};

        println!("Parsing {}...", file);

        let path = format!("./examples/{}", file);
        let src = std::fs::read_to_string(path).unwrap();
        let (_maj, errs) = Combinators::parser().parse_recovery(src.trim());

        //println!("{:#?}", maj);

        errs.into_iter().for_each(|e| {
            let msg = if let chumsky::error::SimpleReason::Custom(msg) = e.reason() {
                msg.clone()
            } else {
                format!(
                    "{}{}, expected {}",
                    if e.found().is_some() {
                        "Unexpected token"
                    } else {
                        "Unexpected end of input"
                    },
                    if let Some(label) = e.label() {
                        format!(" while parsing {}", label)
                    } else {
                        String::new()
                    },
                    if e.expected().len() == 0 {
                        "something else".to_string()
                    } else {
                        e.expected()
                            .map(|expected| match expected {
                                Some(expected) => expected.to_string(),
                                None => "end of input".to_string(),
                            })
                            .collect::<Vec<_>>()
                            .join(", ")
                    },
                )
            };

            let report = Report::build(ReportKind::Error, (), e.span().start)
                .with_code(3)
                .with_message(msg)
                .with_label(
                    Label::new(e.span())
                        .with_message(match e.reason() {
                            chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                            _ => format!(
                                "Unexpected {}",
                                e.found()
                                    .map(|c| format!("token {}", c.fg(Color::Red)))
                                    .unwrap_or_else(|| "end of input".to_string())
                            ),
                        })
                        .with_color(Color::Red),
                );

            let report = match e.reason() {
                chumsky::error::SimpleReason::Unclosed { span, delimiter } => report.with_label(
                    Label::new(span.clone())
                        .with_message(format!(
                            "Unclosed delimiter {}",
                            delimiter.fg(Color::Yellow)
                        ))
                        .with_color(Color::Yellow),
                ),
                chumsky::error::SimpleReason::Unexpected => report,
                chumsky::error::SimpleReason::Custom(_) => report,
            };

            report.finish().print(Source::from(&src)).unwrap();
        });
    }
}
