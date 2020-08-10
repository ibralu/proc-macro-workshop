extern crate proc_macro;

use proc_macro::TokenStream;

use quote::{format_ident, quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder, attributes(builder))]
pub fn derive(input: TokenStream) -> TokenStream {
    let _ = input;

    let input = parse_macro_input!(input as DeriveInput);
    let ident = input.ident.clone();
    let fields = if let syn::Data::Struct(v) = input.data {
        if let syn::Fields::Named(fields) = v.fields {
            fields
        } else {
            unimplemented!()
        }
    } else {
        return compile_err(input.span(), "Builder macro can only be used with structs");
    };

    let b_ident = format_ident!("{}Builder", ident);
    let b_fields = fields.named.iter().map(|f| {
        let ident = f.ident.clone();
        let ty = f.ty.clone();
        if let Some(_opt_ty) = extract_option_ty(&ty) {
            quote! {
                #ident: #ty
            }
        } else {
            quote! {
                #ident: Option<#ty>
            }
        }
    });
    let b_setters = fields.named.iter().map(|f| {
        let ident = f.ident.clone();
        let ty = f.ty.clone();
        if let Some(opt_ty) = extract_option_ty(&ty) {
            quote! {
                pub fn #ident(&mut self, v: #opt_ty) -> &mut Self {
                    self.#ident = Some(v);
                    self
                }
            }
        } else {
            quote! {
                pub fn #ident(&mut self, v: #ty) -> &mut Self {
                    self.#ident = Some(v);
                    self
                }
            }
        }
    });

    let b_build_fields = fields.named.iter().map(|f| {
        let ident = f.ident.clone();
        let ident_str = ident.clone().unwrap().to_string();
        let ty = f.ty.clone();
        if let Some(_opt_ty) = extract_option_ty(&ty) {
            quote! {
                #ident: self.#ident.clone()
            }
        } else {
            quote! {
                #ident: self.#ident.take().ok_or_else(|| format!("field {} has not been set", #ident_str))?
             }
        }
    });

    let tokens = quote! {
      impl #ident {
        pub fn builder() -> #b_ident {
            Default::default()
        }
      }


      #[derive(Default)]
      pub struct #b_ident {
           #(#b_fields),*
       }

      impl #b_ident {
        #(#b_setters)*

        pub fn build(&mut self) -> std::result::Result<#ident, std::boxed::Box<dyn std::error::Error>> {
           Ok(#ident {
               #(#b_build_fields),*
           })
        }
      }
    };

    tokens.into()
}

fn compile_err(span: syn::export::Span, msg: impl std::fmt::Display) -> TokenStream {
    syn::Error::new(span, msg).to_compile_error().into()
}

fn extract_option_ty(ty: &syn::Type) -> Option<syn::Type> {
    if let syn::Type::Path(syn::TypePath {
        qself: _,
        path: syn::Path { segments, .. },
    }) = ty
    {
        if let Some(segment) = segments.first() {
            // TODO: Check for std paths
            if segment.ident == "Option" {
                if let syn::PathArguments::AngleBracketed(ref gen) = segment.arguments {
                    if let Some(arg) = gen.args.first() {
                        if let syn::GenericArgument::Type(a) = arg {
                            Some(a.clone())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}
