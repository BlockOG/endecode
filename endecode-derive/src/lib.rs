use proc_macro2::{Literal, Span, TokenTree};
use quote::quote;
use syn::{
    parse_macro_input, parse_quote_spanned, Data, DeriveInput, Fields, Ident, WherePredicate,
};

#[proc_macro_derive(Encode)]
pub fn derive_encode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        data,
        mut generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    match data {
        Data::Struct(st) => {
            generics
                .make_where_clause()
                .predicates
                .extend(st.fields.iter().map(|field| -> WherePredicate {
                    let ty = &field.ty;
                    parse_quote_spanned! {
                        Span::call_site() => #ty: endecode::encode::Encode
                    }
                }));
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

            let fields = match st.fields {
                Fields::Named(fields) => fields
                    .named
                    .into_iter()
                    .map(|field| TokenTree::Ident(field.ident.clone().unwrap()))
                    .collect(),
                Fields::Unnamed(fields) => (0..fields.unnamed.len())
                    .map(|field| TokenTree::Literal(Literal::usize_unsuffixed(field)))
                    .collect(),
                Fields::Unit => vec![],
            };

            quote! {
                #[automatically_derived]
                impl #impl_generics endecode::encode::Encode for #ident #ty_generics #where_clause {
                    fn encode_internal(&self, vec: &mut Vec<u8>) {
                        #(
                            self.#fields.encode_internal(vec);
                        )*
                    }
                }
            }
            .into()
        }
        Data::Enum(en) => {
            let variants = en.variants.into_iter().enumerate().map(|(num, variant)| {
                let num = Literal::usize_unsuffixed(num);
                let ident = variant.ident;
                match variant.fields {
                    Fields::Named(fields) => {
                        let fields: Vec<_> = fields
                            .named
                            .into_iter()
                            .map(|field| field.ident.unwrap())
                            .collect();

                        quote! {
                            #ident { #(#fields),* } => {
                                vec.push(#num);
                                #(
                                    #fields.encode_internal(vec);
                                )*
                            }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let fields: Vec<_> = (0..fields.unnamed.len())
                            .map(|field| Ident::new(&format!("_{field}"), Span::call_site()))
                            .collect();

                        quote! {
                            #ident(#(#fields),*) => {
                                vec.push(#num);
                                #(
                                    #fields.encode_internal(vec);
                                )*
                            }
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            #ident => vec.push(#num),
                        }
                    }
                }
            });

            quote! {
                #[automatically_derived]
                impl endecode::encode::Encode for #ident {
                    fn encode_internal(&self, vec: &mut Vec<u8>) {
                        match self {
                            #(Self::#variants)*
                        }
                    }
                }
            }
            .into()
        }
        Data::Union(_) => {
            panic!("Union... go implement it yourself");
        }
    }
}

#[proc_macro_derive(Decode)]
pub fn derive_decode(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let DeriveInput {
        ident,
        data,
        mut generics,
        ..
    } = parse_macro_input!(input as DeriveInput);

    match data {
        Data::Struct(st) => {
            generics
                .make_where_clause()
                .predicates
                .extend(st.fields.iter().map(|field| -> WherePredicate {
                    let ty = &field.ty;
                    parse_quote_spanned! {
                        Span::call_site() => #ty: endecode::encode::Encode
                    }
                }));
            let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

            let fields = match st.fields {
                Fields::Named(fields) => {
                    let mut types = vec![];
                    let fields: Vec<_> = fields
                        .named
                        .into_iter()
                        .map(|field| {
                            types.push(field.ty);
                            field.ident.unwrap()
                        })
                        .collect();

                    quote! {
                        {
                            #(
                                #fields: <#types>::decode(iter)
                            ),*
                        }
                    }
                }
                Fields::Unnamed(fields) => {
                    let mut types = vec![];
                    for field in fields.unnamed {
                        types.push(field.ty);
                    }

                    quote! {
                        (#(<#types>::decode(iter)),*)
                    }
                }
                Fields::Unit => quote! {},
            };

            quote! {
                #[automatically_derived]
                impl #impl_generics endecode::decode::Decode for #ident #ty_generics #where_clause {
                    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
                        Self #fields
                    }
                }
            }
            .into()
        }
        Data::Enum(en) => {
            let nums = (0..en.variants.len()).map(|num| Literal::usize_unsuffixed(num));
            let variants = en.variants.into_iter().map(|variant| {
                let ident = variant.ident;
                match variant.fields {
                    Fields::Named(fields) => {
                        let mut types = vec![];
                        let fields: Vec<_> = fields
                            .named
                            .into_iter()
                            .map(|field| {
                                types.push(field.ty);
                                field.ident.unwrap()
                            })
                            .collect();

                        quote! {
                            #ident { #(#fields: <#types>::decode(iter)),* }
                        }
                    }
                    Fields::Unnamed(fields) => {
                        let mut types = vec![];
                        for field in fields.unnamed {
                            types.push(field.ty);
                        }

                        quote! {
                            #ident(#(<#types>::decode(iter)),*)
                        }
                    }
                    Fields::Unit => quote! { #ident },
                }
            });

            quote! {
                #[automatically_derived]
                impl endecode::decode::Decode for #ident {
                    fn decode(iter: &mut impl Iterator<Item = u8>) -> Self {
                        match iter.next().unwrap() {
                            #(#nums => Self::#variants,)*
                            i => panic!(concat!("index {} out of bounds for enum ", stringify!(#ident)), i)
                        }
                    }
                }
            }
            .into()
        }
        Data::Union(_) => {
            panic!("Now decoding?! You have to be kidding me");
        }
    }
}
