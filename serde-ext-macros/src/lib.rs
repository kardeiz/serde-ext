extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;

use syn::*;

#[proc_macro_attribute]
pub fn extend_serde(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let out = item.clone();
    let mut input = parse_macro_input!(item as DeriveInput);

    let mut new_fns = Vec::new();

    match input.data {
        Data::Struct(ref mut ds) => {
            for (idx, field) in ds.fields.iter_mut().enumerate() {
                let field_ref = &field.clone();
                let fn_name_ref = &fn_name(&input.ident, field_ref, idx);

                for attr in field.attrs.iter_mut() {
                    if let Ok(meta) = attr.parse_meta() {
                        let name = meta.name().to_string();
                        if name == "serde_ext" {
                            attr.path = parse_quote!(serde);
                            let (new_fn, new_tts) = parse_meta(meta, fn_name_ref, &field_ref.ty);
                            new_fns.push(new_fn);
                            attr.tts = new_tts;
                        }
                    }
                }
            }
        }
        _ => {}
    };

    let out = quote! {
        #(#new_fns)*
        #input
    };

    out.into()
}

fn fn_name(data_name: &Ident, field: &Field, idx: usize) -> Ident {
    let data_name_s = data_name.to_string();
    let field_name_s =
        field.ident.as_ref().map(|x| x.to_string()).unwrap_or_else(|| idx.to_string());
    Ident::new(
        &format!("serde_ext_default_for_{}_{}", data_name_s, field_name_s),
        Span::call_site(),
    )
}

fn parse_meta(
    meta: Meta,
    fn_name: &Ident,
    fn_ty: &Type,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut new_fn = None;
    let mut new_tts = None;

    let fn_name_s = &fn_name.to_string();

    if let Meta::List(ml) = meta {
        for nm in ml.nested {
            if let NestedMeta::Meta(Meta::List(ml2)) = nm {
                if ml2.ident.to_string().as_str() == "default" {
                    for nm2 in ml2.nested {
                        if let NestedMeta::Meta(Meta::NameValue(mnv)) = nm2 {
                            if mnv.ident.to_string().as_str() == "literal" {
                                let lit = &mnv.lit;
                                new_fn = Some(quote! {
                                    fn #fn_name() -> #fn_ty {
                                        std::convert::From::from(#lit)
                                    }
                                });
                                new_tts = Some(quote!((default = #fn_name_s)))
                            }
                            if mnv.ident.to_string().as_str() == "inline" {
                                if let Lit::Str(ls) = mnv.lit {
                                    if let Ok(lsv) = syn::parse_str::<Expr>(&ls.value()) {
                                        new_fn = Some(quote! {
                                            fn #fn_name() -> #fn_ty {
                                                (#lsv)()
                                            }
                                        });
                                        new_tts = Some(quote!((default = #fn_name_s)));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    (new_fn.unwrap(), new_tts.unwrap())
}
