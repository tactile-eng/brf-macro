#![warn(missing_docs)]

//! brf-macro is a simple procedural macro to generate unicode braille strings from
//! [Braille Ascii](https://en.wikipedia.org/wiki/Braille_ASCII), commonly used in BRF (Braille Ready File) files.

use braille_ascii::{ascii::AsciiString, BrailleAsciiString};
use proc_macro::TokenStream;
use quote::ToTokens;

/// Creates a static `str` of Unicode Braille from a string literal in Braille ASCII format.
///
/// The argument to this macro must be a `str` literal containing only ASCII characters. ASCII
/// control characters (codepoints < 32) will be included in the output string unchanged.
#[proc_macro]
pub fn brf(input: TokenStream) -> TokenStream {
    let literal: syn::LitStr =
        syn::parse(input).expect("brf macro can only be used with a string literal argument");
    let ascii = AsciiString::from_ascii(literal.value()).unwrap();
    let braille = BrailleAsciiString::from_ascii(ascii).to_unicode_braille();
    let braille_literal = syn::LitStr::new(&braille, literal.span());
    braille_literal.into_token_stream().into()
}

/// Creates a static `&[u8]` of Braille dot patterns from a string literal in Braille ASCII format.
///
/// The argument to this macro must be a `str` literal containing only ASCII characters. Any ASCII
/// control characters (codepoints < 32) will be rendered as empty Braille cells.
#[proc_macro]
pub fn brf_bytes(input: TokenStream) -> TokenStream {
    let literal: syn::LitStr =
        syn::parse(input).expect("brf macro can only be used with a string literal argument");
    let ascii = AsciiString::from_ascii(literal.value()).unwrap();
    let braille = BrailleAsciiString::from_ascii(ascii).to_unicode_braille();
    let cells: Vec<u8> = braille
        .chars()
        .map(|c| {
            if c < ' ' {
                0
            } else {
                u32::from(c).to_le_bytes()[0]
            }
        })
        .collect();
    let braille_literal = syn::LitByteStr::new(&cells, literal.span());
    braille_literal.into_token_stream().into()
}
