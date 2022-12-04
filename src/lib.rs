#![warn(missing_docs)]

//! brf-macro is a simple procedural macro to generate unicode braille strings from
//! [Braille Ascii](https://en.wikipedia.org/wiki/Braille_ASCII), commonly used in BRF (Braille Ready File) files.

use proc_macro::TokenStream;
use quote::ToTokens;

const NORTH_AMERICAN_BRAILLE_ASCII_CODE: [char; 96] = [
    '⠀', '⠮', '⠐', '⠼', '⠫', '⠩', '⠯', '⠄', '⠷', '⠾', '⠡', '⠬', '⠠', '⠤', '⠨', '⠌', '⠴', '⠂', '⠆',
    '⠒', '⠲', '⠢', '⠖', '⠶', '⠦', '⠔', '⠱', '⠰', '⠣', '⠿', '⠜', '⠹', '⠈', '⠁', '⠃', '⠉', '⠙', '⠑',
    '⠋', '⠛', '⠓', '⠊', '⠚', '⠅', '⠇', '⠍', '⠝', '⠕', '⠏', '⠟', '⠗', '⠎', '⠞', '⠥', '⠧', '⠺', '⠭',
    '⠽', '⠵', '⠪', '⠳', '⠻', '⠘', '⠸', '⠈', '⠁', '⠃', '⠉', '⠙', '⠑', '⠋', '⠛', '⠓', '⠊', '⠚', '⠅',
    '⠇', '⠍', '⠝', '⠕', '⠏', '⠟', '⠗', '⠎', '⠞', '⠥', '⠧', '⠺', '⠭', '⠽', '⠵', '⠪', '⠳', '⠻', '⠘',
    '⠸',
];

/// Creates a static `str` of Unicode Braille from a string literal in Braille ASCII format.
///
/// The argument to this macro must be a `str` literal containing only printable ASCII characters,
/// carriage returns and newlines.
#[proc_macro]
pub fn brf(input: TokenStream) -> TokenStream {
    let ascii: syn::LitStr =
        syn::parse(input).expect("brf macro can only be used with a string literal argument");
    let braille: String = ascii
        .value()
        .chars()
        .map(|c| {
            if c == '\r' || c == '\n' {
                c
            } else {
                assert!(
                    c.is_ascii() && c >= ' ',
                    "{:?} is an invalid brf character",
                    c
                );
                let i = (u32::from(c) as usize) - 0x20;
                NORTH_AMERICAN_BRAILLE_ASCII_CODE[i]
            }
        })
        .collect();
    let braille = syn::LitStr::new(&braille, ascii.span());
    braille.into_token_stream().into()
}

/// Creates a static `&[u8]` of Braille dot patterns from a string literal in Braille ASCII format.
///
/// The argument to this macro must be a `str` literal containing only printable ASCII characters.
#[proc_macro]
pub fn brf_bytes(input: TokenStream) -> TokenStream {
    let ascii: syn::LitStr =
        syn::parse(input).expect("brf macro can only be used with a string literal argument");
    let braille: Vec<u8> = ascii
        .value()
        .chars()
        .map(|c| {
            assert!(
                c.is_ascii() && c >= ' ',
                "{:?} is an invalid brf character",
                c
            );
            let i = (u32::from(c) as usize) - 0x20;
            u32::from(NORTH_AMERICAN_BRAILLE_ASCII_CODE[i]).to_le_bytes()[0]
        })
        .collect();
    let braille = syn::LitByteStr::new(&braille, ascii.span());
    braille.into_token_stream().into()
}
