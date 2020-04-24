#![feature(proc_macro_diagnostic)]

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::spanned::Spanned;
use syn::*;

use regex_automata::{DenseDFA, Regex as RRegex, RegexBuilder};

type Regex = RRegex<DenseDFA<Vec<u32>, u32>>;

#[proc_macro_attribute]
pub fn regex(
    _args: proc_macro::TokenStream,
    tk_in: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    // Parse the input
    //
    // This has to be done in the main function as the macro can return early if
    // the input token stream isn't a const item, and thus needs to have a
    // return type of TokenStream
    let input = parse_macro_input!(tk_in as ItemConst);
    // Traverse the AST from syn to capture a regex from the string literal
    let (regex, name) = get_regex(input);
    // Generate the code and convert it from proc_macro2 to proc_macro
    proc_macro::TokenStream::from(gen_matcher(regex, name))
    // proc_macro::TokenStream::from(quote! {

    // })
}

fn empty_regex() -> Regex {
    RegexBuilder::new().build_with_size("").unwrap()
}

fn get_regex(input: ItemConst) -> (Regex, String) {
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
            return (empty_regex(), String::new());
        }
    };
    let regex = RegexBuilder::new()
        .build_with_size::<u32>(&regex_str) // TODO: customizability.
        .expect("Invalid Regex"); // TODO: better error reporting.

    (regex, name)
}

fn gen_matcher(re: Regex, name: String) -> TokenStream {
    let forward_bytes = re.forward().to_bytes_native_endian().unwrap();
    let backward_bytes = re.reverse().to_bytes_native_endian().unwrap();
    let re_name = format_ident!("{}", name);
    quote! {
        use lazy_static;
        use regex_automata;
        lazy_static::lazy_static!{
            static ref #re_name: regex_automata::Regex::<regex_automata::DenseDFA<&'static [u32], u32>> = {
                let fwd_dfa: regex_automata::DenseDFA<_, u32> = unsafe{ regex_automata::DenseDFA::from_bytes( &[#(#forward_bytes),*] ) };
                let rvs_dfa: regex_automata::DenseDFA<_, u32> = unsafe{ regex_automata::DenseDFA::from_bytes( &[#(#backward_bytes),*]) };
                regex_automata::Regex::<regex_automata::DenseDFA<&'static [u32], u32>>::from_dfas(fwd_dfa, rvs_dfa)
            };
        }
    }
    // quote!{}
}
