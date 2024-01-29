use majestic::parser::{self, combinators::Combinators};
use majestic::printer;
use majestic::vm::VirtualMachine;
// use std::ffi::{CStr, CString};
// use std::os::raw::c_char;
use wasm_bindgen::prelude::*;

static mut VM: Option<Box<VirtualMachine>> = None;

// #[no_mangle]
// pub extern "C" fn majestic_init() {
//     unsafe {
//         if VM.is_none() {
//             VM = Some(VirtualMachine::new());
//         }
//     }
// }

#[wasm_bindgen]
pub fn majestic_init() {
    unsafe {
        if VM.is_none() {
            VM = Some(VirtualMachine::new());
        }
    }
}

#[wasm_bindgen]
pub fn majestic_eval(line: String) -> String {
    use chumsky::Parser;

    unsafe {
        if VM.is_none() {
            return String::from("majestic vm not initialized");
        }
    }

    let (maj, errs) = Combinators::parser().parse_recovery(line.clone());
    match report_error(line.clone(), errs) {
        Some(errstr) => errstr,
        None => {
            if let Some(expressions) = maj.clone() {
                let mut vm = unsafe { VM.as_mut().unwrap() };

                if expressions.len() == 0 {
                    return "".to_string();
                }

                let expr = expressions.first().unwrap().to_owned();
                match parser::convert::build_ast(&mut vm, expr) {
                    Err(e) => {
                        format!("Error while converting to S-expression: {}", e)
                    }
                    Ok(ptr) => match vm.evaluate(ptr) {
                        Ok(ans) => printer::format_object(&vm, &ans),
                        Err(e) => format!("Error during evaluation: {}", e),
                    },
                }
            } else {
                "nothing to evaluate".to_string()
            }
        }
    }
}

// #[no_mangle]
// pub extern "C" fn majestic_eval(expr: *const c_char) -> *mut c_char {
//     use chumsky::Parser;

//     unsafe {
//         if VM.is_none() {
//             return CString::new("majestic vm not initialized")
//                 .unwrap()
//                 .into_raw();
//         }
//     }

//     let cstr = unsafe { CStr::from_ptr(expr) };
//     let line = cstr.to_str().unwrap().to_owned().trim().to_string();

//     let (maj, errs) = Combinators::parser().parse_recovery(line.clone());
//     match report_error(line.clone(), errs) {
//         Some(errstr) => CString::new(errstr).unwrap().into_raw(),
//         None => {
//             if let Some(expressions) = maj.clone() {
//                 let mut vm = unsafe { VM.as_mut().unwrap() };

//                 if expressions.len() == 0 {
//                     return CString::new("").unwrap().into_raw();
//                 }

//                 let expr = expressions.first().unwrap().to_owned();
//                 match parser::convert::build_ast(&mut vm, expr) {
//                     Err(e) => {
//                         CString::new(format!("Error while converting to S-expression: {}", e))
//                             .unwrap()
//                             .into_raw()
//                     }
//                     Ok(ptr) => match vm.evaluate(ptr) {
//                         Ok(ans) => CString::new(printer::format_object(&vm, &ans))
//                             .unwrap()
//                             .into_raw(),
//                         Err(e) => CString::new(format!("Error during evaluation: {}", e))
//                             .unwrap()
//                             .into_raw(),
//                     },
//                 }
//             } else {
//                 CString::new("nothing to evaluate").unwrap().into_raw()
//             }
//         }
//     }
// }

fn main() -> Result<(), JsValue> {
    majestic_init();
    Ok(())
}

// #[test]
// fn expr_test() {
//     majestic_init();

//     let expr = CString::new("(cons 1 2)").unwrap();

//     let ans_ptr = unsafe { CStr::from_ptr(majestic_eval(expr.clone().into_raw())) };

//     println!(
//         "{} => {}",
//         expr.to_str().unwrap(),
//         ans_ptr.to_str().unwrap()
//     );
// }

fn report_error(src: String, errs: Vec<chumsky::prelude::Simple<char>>) -> Option<String> {
    use ariadne::{Label, Report, ReportKind, Source};
    use std::io::BufWriter;

    let string: String = errs
        .into_iter()
        .map(|e| {
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
                .with_label(Label::new(e.span()).with_message(match e.reason() {
                    chumsky::error::SimpleReason::Custom(msg) => msg.clone(),
                    _ => format!(
                            "Unexpected {}",
                            e.found()
                                .map(|c| format!("token {}", c))
                                .unwrap_or_else(|| "end of input".to_string())
                        ),
                }));

            let report = match e.reason() {
                chumsky::error::SimpleReason::Unclosed { span, delimiter } => report.with_label(
                    Label::new(span.clone())
                        .with_message(format!("Unclosed delimiter {}", delimiter)),
                ),
                chumsky::error::SimpleReason::Unexpected => report,
                chumsky::error::SimpleReason::Custom(_) => report,
            };

            let mut v = Vec::new();
            let buf = BufWriter::new(&mut v);
            report.finish().write(Source::from(&src), buf).unwrap();
            let result: String = String::from_utf8(v).unwrap().trim().to_string();
            result
        })
        .reduce(|cur: String, nxt: String| cur + &nxt)
        .unwrap_or("".to_string())
        .to_string();

    if string == "" {
        return None;
    }
    Some(string)
}
