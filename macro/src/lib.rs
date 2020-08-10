#![warn(rust_2018_idioms)]

use quote::ToTokens;
use syn::parse_macro_input;

fn fourcc_impl(str_lit: &syn::LitStr) -> syn::Result<proc_macro2::TokenStream> {
    let text = str_lit.value();
    let span = str_lit.span();

    if !text.is_ascii() {
        return Err(syn::Error::new_spanned(
            str_lit,
            "must be all ASCII characters",
        ));
    }
    if text.len() != 4 {
        return Err(syn::Error::new_spanned(
            str_lit,
            "must be exactly 4 characters",
        ));
    }
    let value = text.chars().fold(0, |acc, c| acc << 8 | (c as u32));

    let generated_lit_code = format!("0x{:X}u32", value);
    let generated_lit = syn::LitInt::new(&generated_lit_code, span);
    Ok(generated_lit.into_token_stream())
}

#[proc_macro]
pub fn fourcc(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lit = parse_macro_input!(input as syn::LitStr);
    match fourcc_impl(&lit) {
        Ok(output) => output,
        Err(err) => err.to_compile_error(),
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fourcc_generated_literal(text: &str) -> String {
        let lit_str = syn::LitStr::new(text, proc_macro2::Span::call_site());
        fourcc_impl(&lit_str).unwrap().to_string()
    }

    #[test]
    fn fourcc() {
        assert_eq!(fourcc_generated_literal("soun"), "0x736F756Eu32");
        assert_eq!(fourcc_generated_literal("text"), "0x74657874u32");
    }
}
