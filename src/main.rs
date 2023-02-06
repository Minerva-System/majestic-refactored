static TIMESTAMP: &'static str = include_str!(concat!(env!("OUT_DIR"), "/timestamp.txt"));
static VERSION: &'static str = include_str!(concat!(env!("OUT_DIR"), "/version.txt"));
static TARGET: &'static str = include_str!(concat!(env!("OUT_DIR"), "/target.txt"));

use colored::*;
use log::{debug, error, info};
use rustyline::error::ReadlineError;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::Editor;
use rustyline_derive::{Completer, Helper, Highlighter, Hinter};

mod parser;
mod printer;
mod vm;

use parser::combinators::Combinators;

use crate::vm::VirtualMachine;

fn load_log_config() {
    let mut cfg = std::env::current_dir().unwrap();
    cfg.push("log_config.yml");

    if let Ok(_) = log4rs::init_file(cfg.clone(), Default::default()) {
        debug!("Loaded log config file: {}", cfg.display());
    }
}

#[derive(Completer, Helper, Highlighter, Hinter)]
struct MajInputValidator {}

impl Validator for MajInputValidator {
    fn validate(&self, ctx: &mut ValidationContext) -> Result<ValidationResult, ReadlineError> {
        use ValidationResult::{Incomplete, Invalid, Valid};

        let input = ctx.input();
        let mut ignore_one = false;
        let mut count = 0;
        for c in input.chars() {
            if ignore_one {
                ignore_one = false;
            } else {
                match c {
                    '(' => {
                        count += 1;
                    }
                    ')' => {
                        if count == 0 {
                            count = -1;
                            break;
                        } else {
                            count -= 1;
                        }
                    }
                    '\\' => {
                        // Gambiarra for preventing
                        // incomplete input on characters
                        // such as #\( and #\)
                        ignore_one = true;
                    }
                    _ => {}
                }
            }
        }

        if count > 0 {
            Ok(Incomplete)
        } else if count < 0 {
            Ok(Invalid(Some("No matching parenthesis found".to_owned())))
        } else {
            Ok(Valid(None))
        }
    }
}

fn repl(mut vm: &mut VirtualMachine) {
    let history_path = {
        let mut path = std::env::current_dir().unwrap();
        path.push(".majestic_history");
        path
    };

    let validator = MajInputValidator {};

    let config = rustyline::Config::builder()
        .history_ignore_space(true)
        .completion_type(rustyline::CompletionType::List)
        .edit_mode(rustyline::EditMode::Emacs)
        .build();

    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(validator));

    match rl.load_history(&history_path) {
        Ok(()) => info!("History loaded."),
        Err(e) => debug!("Unable to load history: {}", e),
    }

    let prompt = format!("{}", "> ".green());
    let prompt_dbg = format!("{}", "> ".red());

    let mut ast = false;
    let mut echo = false;

    println!("Press C-c or C-d to quit");
    loop {
        let readline = rl.readline(if !ast && !echo { &prompt } else { &prompt_dbg });
        match readline {
            Ok(line) if line.trim() == "#debrief" => vm.debrief(),
            Ok(line) if line.trim() == "#atom" => vm.print_atom_table(),
            Ok(line) if line.trim() == "#number" => vm.print_number_table(),
            Ok(line) if line.trim() == "#list" => vm.print_list_area(),
            Ok(line) if line.trim() == "#ast" => ast = !ast,
            Ok(line) if line.trim() == "#echo" => echo = !echo,
            Ok(line) if line.trim().starts_with("#env") => match line.trim()[4..].trim().parse() {
                Ok(num) => vm.print_env(num),
                Err(_) => println!("Could not parse environment number"),
            },
            Ok(line) if (line.trim().len() > 0) && line.trim().get(0..1).unwrap() == "#" => {
                eprintln!("Unknown command {}.", line.trim())
            }
            Ok(line) => {
                use chumsky::Parser;
                rl.add_history_entry(line.clone().trim());

                let (maj, errs) = Combinators::parser().parse_recovery(line.trim());
                report_error(line, errs);

                if ast {
                    println!("{}", format!("{:#?}", maj).cyan());
                }

                if let Some(expressions) = maj.clone() {
                    for expr in expressions {
                        match parser::convert::build_ast(&mut vm, expr) {
                            Err(e) => eprintln!("Error while converting to S-expression: {}", e),
                            Ok(ptr) => {
                                if ast {
                                    println!("{}", format!("{}", ptr).magenta());
                                }

                                if echo {
                                    print!("echo: ");
                                    printer::print_object(&vm, &ptr);
                                    println!();
                                }

                                match vm.evaluate(ptr) {
                                    Ok(ret) => {
                                        printer::print_object(&vm, &ret);
                                        println!();
                                    }
                                    Err(e) => eprintln!("Error during evaluation: {}", e),
                                }
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("{}", "C-c".cyan().dimmed());
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("{}", "C-d".cyan().dimmed());
                break;
            }
            Err(err) => {
                eprintln!("REPL Error: {:?}", err);
                break;
            }
        }
    }

    debug!("Saving history...");

    if let Err(e) = rl.save_history(&history_path) {
        error!("Failed saving history to .maj_history: {}", e);
    } else {
        debug!("History file saved.");
    }

    println!("Quaerendo invenietis.");
}

fn report_error(src: String, errs: Vec<chumsky::prelude::Simple<char>>) {
    use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};

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

fn main() {
    load_log_config();

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

    let mut vm = VirtualMachine::new();

    repl(&mut vm);
}
