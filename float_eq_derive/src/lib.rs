//! Derive macros for the traits provided by the [float_eq] crate.
//!
//! [float_eq]: https://crates.io/crates/float_eq

extern crate proc_macro;

use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, DeriveInput};

mod read;

/// Helper for deriving the various float_eq traits.
///
/// By default, this will derive [`FloatEqUlpsTol`], [`FloatEq`], [`FloatEqDebugUlpsDiff`]
/// and [`AssertFloatEq`].
///
/// Attribute parameters are passed through to the `#[float_eq(...)]` attribute:
/// `ulps_tol` is the name of the type generated for [`FloatEqUlpsTol`] and
/// `debug_ulps_diff` the name of the type generated for [`FloatEqDebugUlpsDiff`].
/// Both are required.
///
/// If the optional `all_tol` parameter is provided, which names the type to be
/// used by `FloatEqAll::AllTol` then [`FloatEqAll`] and [`AssertFloatEqAll`]
/// are also derived.
///
/// See [How to derive the traits] for more information and example usage.
///
/// [`FloatEqUlpsTol`]: trait.FloatEqUlpsTol.html
/// [`FloatEqDebugUlpsDiff`]: trait.FloatEqDebugUlpsDiff.html
/// [`FloatEq`]: trait.FloatEq.html
/// [`FloatEqAll`]: trait.FloatEqAll.html
/// [`AssertFloatEq`]: trait.AssertFloatEq.html
/// [`AssertFloatEqAll`]: trait.AssertFloatEqAll.html
/// [How to derive the traits]: https://jtempest.github.io/float_eq-rs/book/how_to/derive_the_traits.html
#[proc_macro_attribute]
pub fn derive_float_eq(
    args: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let args = parse_macro_input!(args as syn::AttributeArgs);
    let item = parse_macro_input!(item as syn::DeriveInput);

    expand_derive_float_eq(args, item)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_derive_float_eq(
    args: syn::AttributeArgs,
    item: syn::DeriveInput,
) -> Result<TokenStream, syn::Error> {
    let arg_pairs = args.iter().map(read::name_value_pair);
    let has_arg = |name| {
        arg_pairs.clone().any(|nv| {
            if let Ok(nv) = nv {
                nv.name == name
            } else {
                false
            }
        })
    };

    if !has_arg("ulps_tol") {
        let msg = format!(
            r#"Missing ULPs tolerance type name required to derive trait.

help: try specifying `ulps_tol = "{}Ulps"` in `derive_float_eq`."#,
            item.ident
        );
        return Err(syn::Error::new(Span::call_site(), msg));
    }

    if !has_arg("debug_ulps_diff") {
        let msg = format!(
            r#"Missing debug ULPs diff type name required to derive trait.

help: try specifying `debug_ulps_diff = "{}DebugUlpsDiff"` in `derive_float_eq`."#,
            item.ident
        );
        return Err(syn::Error::new(Span::call_site(), msg));
    }

    let mut trait_names = vec![
        "FloatEqUlpsTol",
        "FloatEq",
        "FloatEqDebugUlpsDiff",
        "AssertFloatEq",
    ];
    if has_arg("all_tol") {
        trait_names.push("FloatEqAll");
        trait_names.push("AssertFloatEqAll");
    }

    let mut traits = TokenStream::new();
    trait_names.into_iter().for_each(|ty| {
        let ident = Ident::new(ty, Span::call_site());
        traits.append_all(quote! { float_eq::#ident, });
    });

    Ok(quote! {
        #[derive(#traits)]
        #[float_eq(#(#args,)*)]
        #item
    })
}

#[doc(hidden)]
#[proc_macro_derive(FloatEqUlpsTol, attributes(float_eq))]
pub fn derive_float_eq_ulps_tol(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_float_eq_ulps_tol(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_float_eq_ulps_tol(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let vis = &input.vis;
    let struct_name = &input.ident;
    let fields = read::all_fields_info("FloatEqUlpsTol", &input)?;
    let params = read::float_eq_attr(&input)?;
    let ulps_name = params.ulps_tol_type()?;

    let ulps_type = match fields.ty {
        read::FieldListType::Named => {
            let ulps_fields = fields.expand(|field| {
                let name = &field.name;
                let ty = &field.ty;
                quote! { #vis #name: float_eq::UlpsTol<#ty> }
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
                quote! { #vis float_eq::UlpsTol<#ty> }
            });
            quote! {
                #vis struct #ulps_name( #(#ulps_fields,)* );
            }
        }
        read::FieldListType::Unit => quote! {
            #vis struct #ulps_name;
        },
    };

    let derive_types = params.ulps_tol_derive_types();

    let doc = format!(
        "Floating point ULPs tolerance representation derived from {}, used by float_eq.",
        struct_name
    );
    Ok(quote! {
        #[doc = #doc]
        #[derive(#(#derive_types,)*)]
        #ulps_type

        impl float_eq::FloatEqUlpsTol for #struct_name {
            type UlpsTol = #ulps_name;
        }
    })
}

#[doc(hidden)]
#[proc_macro_derive(FloatEqDebugUlpsDiff, attributes(float_eq))]
pub fn derive_float_eq_debug_ulps_diff(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_float_eq_debug_ulps_diff(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_float_eq_debug_ulps_diff(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let vis = &input.vis;
    let struct_name = &input.ident;
    let fields = read::all_fields_info("FloatEqDebugUlpsDiff", &input)?;
    let params = read::float_eq_attr(&input)?;
    let ulps_name = params.debug_ulps_diff()?;

    let ulps_type = match fields.ty {
        read::FieldListType::Named => {
            let ulps_fields = fields.expand(|field| {
                let name = &field.name;
                let ty = &field.ty;
                quote! { #name: float_eq::DebugUlpsDiff<#ty> }
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
                quote! { float_eq::DebugUlpsDiff<#ty> }
            });
            quote! {
                #vis struct #ulps_name( #(#ulps_fields,)* );
            }
        }
        read::FieldListType::Unit => quote! {
            #vis struct #ulps_name;
        },
    };

    let derive_types = params.debug_ulps_diff_derive_types();

    Ok(quote! {
        #[doc(hidden)]
        #[derive(#(#derive_types,)*)]
        #ulps_type

        impl float_eq::FloatEqDebugUlpsDiff for #struct_name {
            type DebugUlpsDiff = #ulps_name;
        }
    })
}

#[doc(hidden)]
#[proc_macro_derive(FloatEq, attributes(float_eq))]
pub fn derive_float_eq_attribute(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_float_eq(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_float_eq(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let fields = read::all_fields_info("FloatEq", &input)?;
    let params = read::float_eq_attr(&input)?;
    let ulps_name = params.ulps_tol_type()?;

    let expand_exprs = |method| {
        let mut expanded = fields.expand(|field| {
            let name = &field.name;
            let method = Ident::new(method, Span::call_site());
            quote! { self.#name.#method(&other.#name, &tol.#name) }
        });
        if expanded.is_empty() {
            expanded.push(quote! { true });
        }
        expanded
    };

    let eq_abs = expand_exprs("eq_abs");
    let eq_rmax = expand_exprs("eq_rmax");
    let eq_rmin = expand_exprs("eq_rmin");
    let eq_r1st = expand_exprs("eq_r1st");
    let eq_r2nd = expand_exprs("eq_r2nd");
    let eq_ulps = expand_exprs("eq_ulps");

    Ok(quote! {
        impl float_eq::FloatEq for #struct_name {
            type Tol = Self;

            #[inline]
            fn eq_abs(&self, other: &Self, tol: &Self) -> bool {
                #(#eq_abs)&&*
            }

            #[inline]
            fn eq_rmax(&self, other: &Self, tol: &Self) -> bool {
                #(#eq_rmax)&&*
            }

            #[inline]
            fn eq_rmin(&self, other: &Self, tol: &Self) -> bool {
                #(#eq_rmin)&&*
            }

            #[inline]
            fn eq_r1st(&self, other: &Self, tol: &Self) -> bool {
                #(#eq_r1st)&&*
            }

            #[inline]
            fn eq_r2nd(&self, other: &Self, tol: &Self) -> bool {
                #(#eq_r2nd)&&*
            }

            #[inline]
            fn eq_ulps(&self, other: &Self, tol: &#ulps_name) -> bool {
                #(#eq_ulps)&&*
            }
        }
    })
}

#[doc(hidden)]
#[proc_macro_derive(AssertFloatEq, attributes(float_eq))]
pub fn derive_assert_float_eq(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_assert_float_eq(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_assert_float_eq(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let fields = read::all_fields_info("AssertFloatEq", &input)?;
    let params = read::float_eq_attr(&input)?;
    let ulps_name = params.ulps_tol_type()?;
    let diff_name = params.debug_ulps_diff()?;

    let expand_diff_fields = |method| {
        fields.expand(|field| {
            let name = &field.name;
            let method = Ident::new(method, Span::call_site());
            quote! { #name: self.#name.#method(&other.#name) }
        })
    };

    let abs_diff_fields = expand_diff_fields("debug_abs_diff");
    let ulps_diff_fields = expand_diff_fields("debug_ulps_diff");

    let expand_eps_fields = |method| {
        fields.expand(|field| {
            let name = &field.name;
            let method = Ident::new(method, Span::call_site());
            quote! { #name: self.#name.#method(&other.#name, &tol.#name) }
        })
    };

    let abs_eps_fields = expand_eps_fields("debug_abs_tol");
    let rmax_eps_fields = expand_eps_fields("debug_rmax_tol");
    let rmin_eps_fields = expand_eps_fields("debug_rmin_tol");
    let r1st_eps_fields = expand_eps_fields("debug_r1st_tol");
    let r2nd_eps_fields = expand_eps_fields("debug_r2nd_tol");
    let ulps_eps_fields = expand_eps_fields("debug_ulps_tol");

    Ok(quote! {
        impl float_eq::AssertFloatEq for #struct_name {
            type DebugAbsDiff = Self;
            type DebugTol = Self;

            #[inline]
            fn debug_abs_diff(&self, other: &Self) -> Self {
                Self {
                    #(#abs_diff_fields,)*
                }
            }

            #[inline]
            fn debug_ulps_diff(&self, other: &Self) -> #diff_name {
                #diff_name {
                    #(#ulps_diff_fields,)*
                }
            }

            #[inline]
            fn debug_abs_tol(&self, other: &Self, tol: &Self) -> Self {
                Self {
                    #(#abs_eps_fields,)*
                }
            }

            #[inline]
            fn debug_rmax_tol(&self, other: &Self, tol: &Self) -> Self {
                Self {
                    #(#rmax_eps_fields,)*
                }
            }

            #[inline]
            fn debug_rmin_tol(&self, other: &Self, tol: &Self) -> Self {
                Self {
                    #(#rmin_eps_fields,)*
                }
            }

            #[inline]
            fn debug_r1st_tol(&self, other: &Self, tol: &Self) -> Self {
                Self {
                    #(#r1st_eps_fields,)*
                }
            }

            #[inline]
            fn debug_r2nd_tol(&self, other: &Self, tol: &Self) -> Self {
                Self {
                    #(#r2nd_eps_fields,)*
                }
            }

            #[inline]
            fn debug_ulps_tol(&self, other: &Self, tol: &#ulps_name) -> #ulps_name {
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
    let all_tol = params.all_tol_type()?;

    let expand_exprs = |method| {
        let mut expanded = fields.expand(|field| {
            let name = &field.name;
            let method = Ident::new(method, Span::call_site());
            quote! { self.#name.#method(&other.#name, tol) }
        });
        if expanded.is_empty() {
            expanded.push(quote! { true });
        }
        expanded
    };

    let eq_abs = expand_exprs("eq_abs_all");
    let eq_rmax = expand_exprs("eq_rmax_all");
    let eq_rmin = expand_exprs("eq_rmin_all");
    let eq_r1st = expand_exprs("eq_r1st_all");
    let eq_r2nd = expand_exprs("eq_r2nd_all");
    let eq_ulps = expand_exprs("eq_ulps_all");

    Ok(quote! {
        impl float_eq::FloatEqAll for #struct_name {
            type AllTol = #all_tol;

            #[inline]
            fn eq_abs_all(&self, other: &Self, tol: &#all_tol) -> bool {
                #(#eq_abs)&&*
            }

            #[inline]
            fn eq_rmax_all(&self, other: &Self, tol: &#all_tol) -> bool {
                #(#eq_rmax)&&*
            }

            #[inline]
            fn eq_rmin_all(&self, other: &Self, tol: &#all_tol) -> bool {
                #(#eq_rmin)&&*
            }

            #[inline]
            fn eq_r1st_all(&self, other: &Self, tol: &#all_tol) -> bool {
                #(#eq_r1st)&&*
            }

            #[inline]
            fn eq_r2nd_all(&self, other: &Self, tol: &#all_tol) -> bool {
                #(#eq_r2nd)&&*
            }

            #[inline]
            fn eq_ulps_all(&self, other: &Self, tol: &::float_eq::UlpsTol<Self::AllTol>) -> bool {
                #(#eq_ulps)&&*
            }
        }
    })
}

#[doc(hidden)]
#[proc_macro_derive(AssertFloatEqAll, attributes(float_eq))]
pub fn derive_assert_float_eq_all(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    expand_assert_float_eq_all(input)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

fn expand_assert_float_eq_all(input: DeriveInput) -> Result<TokenStream, syn::Error> {
    let struct_name = &input.ident;
    let fields = read::all_fields_info("AssertFloatEqAll", &input)?;
    let params = read::float_eq_attr(&input)?;
    let all_tol = params.all_tol_type()?;

    let expand_fields = |method| {
        fields.expand(|field| {
            let name = &field.name;
            let method = Ident::new(method, Span::call_site());
            quote! { #name: self.#name.#method(&other.#name, tol) }
        })
    };

    let abs_eps_fields = expand_fields("debug_abs_all_tol");
    let rmax_eps_fields = expand_fields("debug_rmax_all_tol");
    let rmin_eps_fields = expand_fields("debug_rmin_all_tol");
    let r1st_eps_fields = expand_fields("debug_r1st_all_tol");
    let r2nd_eps_fields = expand_fields("debug_r2nd_all_tol");
    let ulps_eps_fields = expand_fields("debug_ulps_all_tol");

    Ok(quote! {
        impl float_eq::AssertFloatEqAll for #struct_name {
            type AllDebugTol = Self;

            #[inline]
            fn debug_abs_all_tol(&self, other: &Self, tol: &#all_tol) -> Self {
                Self {
                    #(#abs_eps_fields,)*
                }
            }

            #[inline]
            fn debug_rmax_all_tol(&self, other: &Self, tol: &#all_tol) -> Self {
                Self {
                    #(#rmax_eps_fields,)*
                }
            }

            #[inline]
            fn debug_rmin_all_tol(&self, other: &Self, tol: &#all_tol) -> Self {
                Self {
                    #(#rmin_eps_fields,)*
                }
            }

            #[inline]
            fn debug_r1st_all_tol(&self, other: &Self, tol: &#all_tol) -> Self {
                Self {
                    #(#r1st_eps_fields,)*
                }
            }

            #[inline]
            fn debug_r2nd_all_tol(&self, other: &Self, tol: &#all_tol) -> Self {
                Self {
                    #(#r2nd_eps_fields,)*
                }
            }

            #[inline]
            fn debug_ulps_all_tol(
                &self,
                other: &Self,
                tol: &::float_eq::UlpsTol<Self::AllTol>
            ) -> ::float_eq::UlpsTol<Self::AllDebugTol> {
                ::float_eq::UlpsTol::<Self::AllDebugTol> {
                    #(#ulps_eps_fields,)*
                }
            }
        }
    })
}
