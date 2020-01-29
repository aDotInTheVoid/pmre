#![feature(proc_macro_diagnostic)]

extern crate proc_macro;
use self::proc_macro::TokenStream;
use regex_syntax::{hir::Hir as RegexHir, Parser};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, ExprLit, ItemConst, Lit};

#[proc_macro_attribute]
pub fn regex(_args: TokenStream, tk_in: TokenStream) -> TokenStream {
    // Parse the input
    //
    // This has to be done in the main function as the macro can return early if
    // the input token stream isn't a const item, and thus needs to have a
    // return type of TokenStream
    let input = parse_macro_input!(tk_in as ItemConst);
    let (regex, name) = get_regex(input);

    dbg!(regex);
    todo!("Bottom");
}

fn get_regex(input: ItemConst) -> (RegexHir, String) {
    let name = input.ident.to_string();
    // Grab the regex string
    let regex_str = match *input.expr {
        // String literal gives a regex
        Expr::Lit(ExprLit {
            lit: Lit::Str(y), ..
        }) => y.value(),
        // With anything else, emit an error
        _ => {
            input
                .expr
                .span()
                .unwrap()
                .error("Expected a string literal")
                .emit();
            return (RegexHir::empty(), String::new());
        }
    };
    let regex = Parser::new().parse(&regex_str).expect("Invalid Regex"); // TODO: better error reporting.
    (regex, name)
}
