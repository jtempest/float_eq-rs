extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

mod read;

#[doc(hidden)]
#[proc_macro_derive(FloatUlps, attributes(float_eq))]
pub fn derive_float_ulps(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_float_ulps(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_float_ulps(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let vis = &input.vis;
    let struct_name = &input.ident;
    let fields = read::all_fields_info("FloatUlps", &input)?;
    let params = read::float_eq_attr(&input)?;
    let ulps_name = params.ulps_type()?;

    let ulps_type = match fields.ty {
        read::FieldListType::Named => {
            let ulps_fields = fields.expand(|field| {
                let name = &field.name;
                let ty = &field.ty;
                quote! { #name: float_eq::Ulps<#ty> }
            });
            quote! {
                #vis struct #ulps_name {
                    #(#ulps_fields,)*
                }
            }
        }
        read::FieldListType::Tuple => {
            let ulps_fields = fields.expand(|field| {
                let ty = &field.ty;
                quote! { float_eq::Ulps<#ty> }
            });
            quote! {
                #vis struct #ulps_name( #(#ulps_fields,)* );
            }
        }
        read::FieldListType::Unit => quote! {
            #vis struct #ulps_name;
        },
    };

    let doc = format!(
        "Floating point ULPs representation derived from {}",
        struct_name.to_string()
    );
    Ok(quote! {
        #[doc = #doc]
        #[derive(Debug, PartialEq)]
        #ulps_type

        impl FloatUlps for #struct_name {
            type Ulps = #ulps_name;
        }
    })
}

#[doc(hidden)]
#[proc_macro_derive(FloatDiff, attributes(float_eq))]
pub fn derive_float_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_float_diff(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_float_diff(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let fields = read::all_fields_info("FloatDiff", &input)?;
    let params = read::float_eq_attr(&input)?;
    let ulps_name = params.ulps_type()?;

    let expand_fields = |method| {
        let method = Ident::new(method, Span::call_site());
        fields.expand(|field| {
            let name = &field.name;
            quote! { #name: self.#name.#method(&other.#name) }
        })
    };

    let abs_diff_fields = expand_fields("abs_diff");
    let ulps_diff_fields = expand_fields("ulps_diff");

    Ok(quote! {
        impl FloatDiff for #struct_name {
            type Output = Self;

            #[inline]
            fn abs_diff(&self, other: &Self) -> Self {
                Self {
                    #(#abs_diff_fields,)*
                }
            }

            #[inline]
            fn ulps_diff(&self, other: &Self) -> Option<#ulps_name> {
                Some(
                    #ulps_name {
                        #(#ulps_diff_fields?,)*
                    }
                )
            }
        }
    })
}

#[doc(hidden)]
#[proc_macro_derive(FloatEq, attributes(float_eq))]
pub fn derive_float_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_float_eq(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_float_eq(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let fields = read::all_fields_info("FloatEq", &input)?;
    let params = read::float_eq_attr(&input)?;
    let ulps_name = params.ulps_type()?;

    let expand_exprs = |method| {
        let mut expanded = fields.expand(|field| {
            let name = &field.name;
            let method = Ident::new(method, Span::call_site());
            quote! { self.#name.#method(&other.#name, &max_diff.#name) }
        });
        if expanded.is_empty() {
            expanded.push(quote! { true });
        }
        expanded
    };

    let eq_abs = expand_exprs("eq_abs");
    let eq_rel = expand_exprs("eq_rel");
    let eq_ulps = expand_exprs("eq_ulps");

    Ok(quote! {
        impl FloatEq for #struct_name {
            type Epsilon = Self;

            #[inline]
            fn eq_abs(&self, other: &Self, max_diff: &Self) -> bool {
                #(#eq_abs)&&*
            }

            #[inline]
            fn eq_rel(&self, other: &Self, max_diff: &Self) -> bool {
                #(#eq_rel)&&*
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, max_diff: &#ulps_name) -> bool {
                #(#eq_ulps)&&*
            }
        }
    })
}

#[doc(hidden)]
#[proc_macro_derive(FloatEqDebug, attributes(float_eq))]
pub fn derive_float_eq_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_float_eq_debug(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_float_eq_debug(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let fields = read::all_fields_info("FloatEqDebug", &input)?;
    let params = read::float_eq_attr(&input)?;
    let ulps_name = params.ulps_type()?;

    let expand_fields = |method| {
        fields.expand(|field| {
            let name = &field.name;
            let method = Ident::new(method, Span::call_site());
            quote! { #name: self.#name.#method(&other.#name, &max_diff.#name) }
        })
    };

    let abs_eps_fields = expand_fields("debug_abs_epsilon");
    let rel_eps_fields = expand_fields("debug_rel_epsilon");
    let ulps_eps_fields = expand_fields("debug_ulps_epsilon");

    Ok(quote! {
        impl FloatEqDebug for #struct_name {
            type DebugEpsilon = Self;

            #[inline]
            fn debug_abs_epsilon(&self, other: &Self, max_diff: &Self) -> Self {
                Self {
                    #(#abs_eps_fields,)*
                }
            }

            #[inline]
            fn debug_rel_epsilon(&self, other: &Self, max_diff: &Self) -> Self {
                Self {
                    #(#rel_eps_fields,)*
                }
            }

            #[inline]
            fn debug_ulps_epsilon(&self, other: &Self, max_diff: &#ulps_name) -> #ulps_name {
                #ulps_name {
                    #(#ulps_eps_fields,)*
                }
            }
        }
    })
}

#[doc(hidden)]
#[proc_macro_derive(FloatEqAll, attributes(float_eq))]
pub fn derive_float_eq_all(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_float_eq_all(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_float_eq_all(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let fields = read::all_fields_info("FloatEqAll", &input)?;
    let params = read::float_eq_attr(&input)?;
    let all_epsilon = params.all_epsilon_type()?;

    let expand_exprs = |method| {
        let mut expanded = fields.expand(|field| {
            let name = &field.name;
            let method = Ident::new(method, Span::call_site());
            quote! { self.#name.#method(&other.#name, max_diff) }
        });
        if expanded.is_empty() {
            expanded.push(quote! { true });
        }
        expanded
    };

    let eq_abs = expand_exprs("eq_abs_all");
    let eq_rel = expand_exprs("eq_rel_all");
    let eq_ulps = expand_exprs("eq_ulps_all");

    Ok(quote! {
        impl FloatEqAll for #struct_name {
            type AllEpsilon = #all_epsilon;

            #[inline]
            fn eq_abs_all(&self, other: &Self, max_diff: &#all_epsilon) -> bool {
                #(#eq_abs)&&*
            }

            #[inline]
            fn eq_rel_all(&self, other: &Self, max_diff: &#all_epsilon) -> bool {
                #(#eq_rel)&&*
            }

            #[inline]
            fn eq_ulps_all(&self, other: &Self, max_diff: &::float_eq::Ulps<Self::AllEpsilon>) -> bool {
                #(#eq_ulps)&&*
            }
        }
    })
}

#[doc(hidden)]
#[proc_macro_derive(FloatEqAllDebug, attributes(float_eq))]
pub fn derive_float_eq_all_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_float_eq_all_debug(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_float_eq_all_debug(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let fields = read::all_fields_info("FloatEqAllDebug", &input)?;
    let params = read::float_eq_attr(&input)?;
    let all_epsilon = params.all_epsilon_type()?;

    let expand_fields = |method| {
        fields.expand(|field| {
            let name = &field.name;
            let method = Ident::new(method, Span::call_site());
            quote! { #name: self.#name.#method(&other.#name, max_diff) }
        })
    };

    let abs_eps_fields = expand_fields("debug_abs_all_epsilon");
    let rel_eps_fields = expand_fields("debug_rel_all_epsilon");
    let ulps_eps_fields = expand_fields("debug_ulps_all_epsilon");

    Ok(quote! {
        impl FloatEqAllDebug for #struct_name {
            type AllDebugEpsilon = Self;

            #[inline]
            fn debug_abs_all_epsilon(&self, other: &Self, max_diff: &#all_epsilon) -> Self {
                Self {
                    #(#abs_eps_fields,)*
                }
            }

            #[inline]
            fn debug_rel_all_epsilon(&self, other: &Self, max_diff: &#all_epsilon) -> Self {
                Self {
                    #(#rel_eps_fields,)*
                }
            }

            #[inline]
            fn debug_ulps_all_epsilon(
                &self,
                other: &Self,
                max_diff: &::float_eq::Ulps<Self::AllEpsilon>
            ) -> ::float_eq::Ulps<Self::AllDebugEpsilon> {
                ::float_eq::Ulps::<Self::AllDebugEpsilon> {
                    #(#ulps_eps_fields,)*
                }
            }
        }
    })
}
