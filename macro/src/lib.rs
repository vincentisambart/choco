#![warn(rust_2018_idioms)]

use quote::{format_ident, quote, ToTokens};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NSObjectProtocol, attributes(choco))]
pub fn nsobjectprotocol_derive_wrapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    nsobjectprotocol_derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

enum ChocoAttr {
    Owned(syn::Ident),
    Framework(syn::Ident),
    ObjCClass(syn::Ident),
    Base(syn::Ident),
}

impl syn::parse::Parse for ChocoAttr {
    fn parse(input: syn::parse::ParseStream<'_>) -> syn::Result<Self> {
        let attr_name: syn::Ident = input.parse()?;
        if attr_name == "base" {
            Ok(ChocoAttr::Base(attr_name))
        } else if attr_name == "framework" {
            input.parse::<syn::Token![=]>()?;
            let ident: syn::Ident = input.parse()?;
            Ok(ChocoAttr::Framework(ident))
        } else if attr_name == "objc_class" {
            input.parse::<syn::Token![=]>()?;
            let ident: syn::Ident = input.parse()?;
            Ok(ChocoAttr::ObjCClass(ident))
        } else if attr_name == "owned" {
            input.parse::<syn::Token![=]>()?;
            let ident: syn::Ident = input.parse()?;
            Ok(ChocoAttr::Owned(ident))
        } else {
            Err(input.error("unexpected attribute name"))
        }
    }
}

fn is_repr_transparent(meta: &syn::Meta) -> bool {
    if !meta.path().is_ident("repr") {
        return false;
    }

    let list = match meta {
        syn::Meta::List(list) => list,
        _ => return false,
    };

    if list.nested.len() != 1 {
        return false;
    }

    let nested = match &list.nested[0] {
        syn::NestedMeta::Meta(nested) => nested,
        _ => return false,
    };

    nested.path().is_ident("transparent")
}

fn nsobjectprotocol_derive(input: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    // ObjC objects should not be directly passed to C (to keep a correct refcount),
    // so technically #[repr(transparent)] doesn't bring much,
    // but at least it requires the struct to have only one non-empty field.
    if let Some(repr) = input.attrs.iter().find(|attr| attr.path.is_ident("repr")) {
        let meta = repr.parse_meta()?;
        if !is_repr_transparent(&meta) {
            return Err(syn::Error::new_spanned(
                repr,
                "#[repr(transparent)] required",
            ));
        }
    } else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[repr(transparent)] required",
        ));
    }

    let data = match &input.data {
        syn::Data::Struct(struct_data) => struct_data,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "only structs are supported",
            ))
        }
    };

    let struct_name = input.ident;

    let attrs: Punctuated<ChocoAttr, syn::Token![,]>;
    if let Some(choco_attr) = input.attrs.iter().find(|attr| attr.path.is_ident("choco")) {
        attrs = choco_attr.parse_args_with(Punctuated::parse_terminated)?;
    } else {
        attrs = Punctuated::new();
    }

    let (owned, is_owned_different) = if let Some(owned) =
        attrs.iter().find_map(|attr| match attr {
            ChocoAttr::Owned(owned) => Some(owned),
            _ => None,
        }) {
        (owned, true)
    } else {
        (&struct_name, false)
    };

    let objc_class = if let Some(objc_class) = attrs.iter().find_map(|attr| match attr {
        ChocoAttr::ObjCClass(objc_class) => Some(objc_class),
        _ => None,
    }) {
        objc_class
    } else {
        owned
    };

    let main_field = match data.fields.iter().next() {
        Some(first_field) => match &first_field.ident {
            Some(ident) => proc_macro2::TokenTree::Ident(ident.clone()),
            None => proc_macro2::TokenTree::Literal(proc_macro2::Literal::u8_unsuffixed(0)),
        },
        None => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "the struct should have at least one field",
            ))
        }
    };

    let other_fields_init: Vec<proc_macro2::TokenStream> = if is_owned_different {
        Vec::new()
    } else {
        data.fields
            .iter()
            .skip(1)
            .map(|field| {
                let name = field.ident.as_ref().unwrap();
                quote! {
                    #name: std::marker::PhantomData
                }
            })
            .collect()
    };

    let location = if let Some(location) = attrs.iter().find_map(|attr| match attr {
        ChocoAttr::Base(ident) => Some(ident),
        ChocoAttr::Framework(ident) => Some(ident),
        _ => None,
    }) {
        location
    } else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "where the struct comes from must be specified, for example #[choco(framework = Foundation)]",
        ));
    };

    let class_func = format_ident!("choco_{}_{}_class", location, objc_class);
    let expect_message = format!(
        "expecting +[{} class] to return a non null pointer",
        objc_class
    );

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    if is_owned_different {
        Ok(quote! {
            impl #impl_generics crate::base::AsRawObjCPtr for #struct_name #ty_generics #where_clause {
                fn as_raw(&self) -> crate::base::RawObjCPtr {
                    self.#main_field
                }
            }

            impl #impl_generics crate::base::NSObjectProtocol for #struct_name #ty_generics #where_clause {
                type Owned = #owned;

                fn class() -> crate::base::ObjCClassPtr {
                    unsafe { #class_func() }
                        .into_opt()
                        .expect(#expect_message)
                }
            }
        })
    } else {
        Ok(quote! {
            impl #impl_generics crate::base::AsRawObjCPtr for #struct_name #ty_generics #where_clause {
                fn as_raw(&self) -> crate::base::RawObjCPtr {
                    self.#main_field.as_raw()
                }
            }

            impl #impl_generics crate::base::TypedOwnedObjCPtr for #struct_name #ty_generics #where_clause {
                unsafe fn from_owned_unchecked(#main_field: crate::base::OwnedObjCPtr) -> Self {
                    Self {
                        #main_field,
                        #(#other_fields_init),*
                    }
                }
            }

            impl #impl_generics crate::base::NSObjectProtocol for #struct_name #ty_generics #where_clause {
                type Owned = Self;

                fn class() -> crate::base::ObjCClassPtr {
                    unsafe { #class_func() }
                        .into_opt()
                        .expect(#expect_message)
                }
            }
        })
    }
}

#[proc_macro_derive(CFType)]
pub fn cftype_derive_wrapper(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    cftype_derive(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

fn cftype_derive(input: DeriveInput) -> Result<proc_macro2::TokenStream, syn::Error> {
    // CFType objects should not be directly passed to C (to keep a correct refcount),
    // so technically #[repr(transparent)] doesn't bring much,
    // but at least it requires the struct to have only one non-empty field.
    if let Some(repr) = input.attrs.iter().find(|attr| attr.path.is_ident("repr")) {
        let meta = repr.parse_meta()?;
        if !is_repr_transparent(&meta) {
            return Err(syn::Error::new_spanned(
                repr,
                "#[repr(transparent)] required",
            ));
        }
    } else {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "#[repr(transparent)] required",
        ));
    }

    let data = match &input.data {
        syn::Data::Struct(struct_data) => struct_data,
        _ => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "only structs are supported",
            ))
        }
    };

    let struct_name = input.ident;

    let main_field = match data.fields.iter().next() {
        Some(first_field) => match &first_field.ident {
            Some(ident) => proc_macro2::TokenTree::Ident(ident.clone()),
            None => proc_macro2::TokenTree::Literal(proc_macro2::Literal::u8_unsuffixed(0)),
        },
        None => {
            return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "the struct should have at least one field",
            ))
        }
    };

    let other_fields_init: Vec<proc_macro2::TokenStream> = data
        .fields
        .iter()
        .skip(1)
        .map(|field| {
            let name = field.ident.as_ref().unwrap();
            quote! {
                #name: std::marker::PhantomData
            }
        })
        .collect();

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    Ok(quote! {
        impl #impl_generics crate::base::AsRawObjCPtr for #struct_name #ty_generics #where_clause {
            fn as_raw(&self) -> RawObjCPtr {
                self.#main_field.as_raw().into()
            }
        }

        impl #impl_generics crate::base::TypedOwnedObjCPtr for #struct_name #ty_generics #where_clause {
            unsafe fn from_owned_unchecked(#main_field: OwnedObjCPtr) -> Self {
                Self {
                    #main_field: #main_field.into(),
                    #(#other_fields_init),*
                }
            }
        }

        impl #impl_generics crate::base::CFTypeInterface for #struct_name #ty_generics #where_clause {
            fn as_raw(&self) -> RawCFTypeRef {
                self.#main_field.as_raw()
            }
        }
    })
}

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
