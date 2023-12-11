use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{
    braced, parenthesized,
    parse::Parse,
    parse_macro_input,
    token::{self},
    Ident, Type,
};

struct InputParams {
    arg_type: Type,
}

impl Parse for InputParams {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<Ident>()?;
        input.parse::<token::Colon>()?;
        let res = InputParams {
            arg_type: input.parse()?,
        };
        if !input.is_empty() {
            Err(input.error("Day function needs to only take one argument"))
        } else {
            if let Type::Reference(ref_type) = &res.arg_type {
                if let Type::Path(sub_type) = ref_type.elem.as_ref() {
                    if let Some(ident) = sub_type.path.get_ident() {
                        if ident == "str" {
                            return Ok(res);
                        }
                    }
                }
            }
            return Err(input.error("Day function needs to take an &str as input argument"));
        }
    }
}

struct DayCode {
    func_name: Ident,
    result_type: Type,
}

impl Parse for DayCode {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let args;
        let code;

        input.parse::<token::Fn>()?;
        let func_name = input.parse::<Ident>()?;
        parenthesized!(args in input);
        args.parse::<InputParams>()?;
        input.parse::<token::RArrow>()?;
        let result_type = input.parse::<Type>()?;
        braced!(code in input);
        code.parse::<proc_macro2::TokenStream>()?;

        Ok(DayCode {
            func_name,
            result_type,
        })
    }
}

struct Attributes {
    attr: Vec<Ident>,
}

impl Attributes {
    fn contains(&self, s: &str) -> bool {
        self.attr.iter().any(|a| a == s)
    }
}

impl Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut attr = Vec::new();
        for token in input.parse::<proc_macro2::TokenStream>()? {
            match token {
                TokenTree::Group(_) => todo!(),
                TokenTree::Ident(i) => attr.push(i),
                TokenTree::Punct(_) => todo!(),
                TokenTree::Literal(_) => todo!(),
            }
        }
        Ok(Attributes { attr })
    }
}

#[proc_macro_attribute]
pub fn make_day(attr: TokenStream, mut item: TokenStream) -> TokenStream {
    let attr = parse_macro_input!(attr as Attributes);

    let to_parse = item.clone();
    let day_code = parse_macro_input!(to_parse as DayCode);

    let result_type = day_code.result_type;
    let func_name = day_code.func_name;

    let res = if attr.contains("trim") {
        quote! {
            pub fn get_day_func() -> impl Fn(&str) -> #result_type {
                |input| { #func_name(input.trim()) }
            }
        }
    } else {
        quote! {
            pub fn get_day_func() -> impl Fn(&str) -> #result_type {
                #func_name
            }
        }
    };

    item.extend(TokenStream::from(res));
    item
}
