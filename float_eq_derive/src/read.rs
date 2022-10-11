use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::{
    spanned::Spanned, Attribute, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Lit,
    LitInt, LitStr, Meta, NestedMeta, Type, Visibility,
};

pub enum FieldName<'a> {
    Ident(&'a Ident),
    Num(Lit),
}

impl ToTokens for FieldName<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FieldName::Ident(ident) => ident.to_tokens(tokens),
            FieldName::Num(num) => num.to_tokens(tokens),
        }
    }
}

pub struct FieldInfo<'a> {
    pub name: FieldName<'a>,
    pub ty: &'a Type,
    pub vis: &'a Visibility,
}

pub enum FieldListType {
    Named,
    Tuple,
    Unit,
}

pub struct FieldInfoList<'a> {
    pub ty: FieldListType,
    fields: Vec<FieldInfo<'a>>,
}

impl FieldInfoList<'_> {
    pub fn expand<F: std::ops::Fn(&FieldInfo) -> TokenStream>(&self, func: F) -> Vec<TokenStream> {
        self.fields.iter().map(func).collect()
    }
}

pub fn all_fields_info<'a>(
    trait_name: &str,
    input: &'a DeriveInput,
) -> Result<FieldInfoList<'a>, syn::Error> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new(
            Span::call_site(),
            "This trait does not yet support derive for generic types.",
        ));
    }

    match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(FieldsNamed { named, .. }) => Ok(FieldInfoList {
                ty: FieldListType::Named,
                fields: named.iter().map(named_field_info).collect(),
            }),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => Ok(FieldInfoList {
                ty: FieldListType::Tuple,
                fields: unnamed.iter().enumerate().map(unnamed_field_info).collect(),
            }),
            Fields::Unit => Ok(FieldInfoList {
                ty: FieldListType::Unit,
                fields: Vec::new(),
            }),
        },
        _ => Err(syn::Error::new(
            input.ident.span(),
            format!("{} may only be derived for structs.", trait_name),
        )),
    }
}

fn named_field_info(field: &syn::Field) -> FieldInfo {
    FieldInfo {
        name: FieldName::Ident(field.ident.as_ref().expect("Expected named field")),
        ty: &field.ty,
        vis: &field.vis,
    }
}

fn unnamed_field_info((n, field): (usize, &syn::Field)) -> FieldInfo {
    FieldInfo {
        name: FieldName::Num(Lit::Int(LitInt::new(&format!("{}", n), Span::call_site()))),
        ty: &field.ty,
        vis: &field.vis,
    }
}

#[derive(Default)]
pub struct FloatEqAttr {
    struct_name: String,
    ulps_tol_type_name: Option<Ident>,
    ulps_tol_derive_types: Option<Vec<Ident>>,
    debug_ulps_diff_type_name: Option<Ident>,
    debug_ulps_diff_derive_types: Option<Vec<Ident>>,
    all_tol_type_name: Option<Ident>,
}

impl FloatEqAttr {
    pub fn ulps_tol_type(&self) -> Result<&Ident, syn::Error> {
        self.ulps_tol_type_name.as_ref().ok_or({
            let msg = format!(
                r#"Missing ULPs tolerance type name required to derive trait.

help: try adding `#[float_eq(ulps_tol = "{}Ulps")]` to your type."#,
                self.struct_name
            );
            syn::Error::new(Span::call_site(), msg)
        })
    }

    pub fn ulps_tol_derive_types(&self) -> Vec<Ident> {
        self.ulps_tol_derive_types
            .as_ref()
            .map_or_else(Vec::new, |v| v.clone())
    }

    pub fn debug_ulps_diff_derive_types(&self) -> Vec<Ident> {
        self.debug_ulps_diff_derive_types
            .as_ref()
            .map_or_else(Vec::new, |v| v.clone())
    }

    pub fn debug_ulps_diff(&self) -> Result<&Ident, syn::Error> {
        self.debug_ulps_diff_type_name.as_ref().ok_or({
            let msg = format!(
                r#"Missing debug ULPs diff type name required to derive trait.

help: try adding `#[float_eq(debug_ulps_diff = "{}DebugUlpsDiff")]` to your type."#,
                self.struct_name
            );
            syn::Error::new(Span::call_site(), msg)
        })
    }

    pub fn all_tol_type(&self) -> Result<&Ident, syn::Error> {
        self.all_tol_type_name.as_ref().ok_or({
            let msg = r#"Missing Tol type name required to derive trait.

help: try adding `#[float_eq(all_tol = "T")]` to your type, where T is commonly `f32` or `f64`."#;
            syn::Error::new(Span::call_site(), msg)
        })
    }
}

pub fn float_eq_attr(input: &DeriveInput) -> Result<FloatEqAttr, syn::Error> {
    let nv_pair_lists: Vec<Vec<NameValuePair>> = input
        .attrs
        .iter()
        .filter(|a| a.path.is_ident("float_eq"))
        .map(|a| name_value_pair_list(&input.ident, a))
        .collect::<Result<_, _>>()?;

    let mut attr_values = FloatEqAttr {
        struct_name: input.ident.to_string(),
        ..Default::default()
    };

    for nv in nv_pair_lists.into_iter().flatten() {
        let name = nv.name.to_string();
        if name == "ulps_tol" {
            set_float_eq_attr(&mut attr_values.ulps_tol_type_name, &nv, &parse_ident)?;
        } else if name == "debug_ulps_diff" {
            set_float_eq_attr(
                &mut attr_values.debug_ulps_diff_type_name,
                &nv,
                &parse_ident,
            )?;
        } else if name == "all_tol" {
            set_float_eq_attr(&mut attr_values.all_tol_type_name, &nv, &parse_ident)?;
        } else if name == "ulps_tol_derive" {
            set_float_eq_attr(
                &mut attr_values.ulps_tol_derive_types,
                &nv,
                &parse_ident_list,
            )?;
        } else if name == "debug_ulps_diff_derive" {
            set_float_eq_attr(
                &mut attr_values.debug_ulps_diff_derive_types,
                &nv,
                &parse_ident_list,
            )?;
        } else {
            let msg = format!(r"'{}' is not a valid float_eq derive option.", name);
            return Err(syn::Error::new(nv.name.span(), msg));
        }
    }

    Ok(attr_values)
}

fn set_float_eq_attr<TAttr>(
    attr_value: &mut Option<TAttr>,
    name_value_pair: &NameValuePair,
    parse_fn: &dyn Fn(&LitStr) -> Result<TAttr, syn::Error>,
) -> Result<(), syn::Error> {
    if attr_value.is_none() {
        if let Ok(value) = parse_fn(&name_value_pair.value) {
            *attr_value = Some(value);
            Ok(())
        } else {
            let msg = format!(
                "Invalid value `{}` for attribute `{}`.",
                name_value_pair.value.value(),
                name_value_pair.name
            );
            Err(syn::Error::new(name_value_pair.value.span(), msg))
        }
    } else {
        let msg = format!("Duplicate `{}` argument", name_value_pair.name);
        Err(syn::Error::new(name_value_pair.name.span(), msg))
    }
}

fn parse_ident(value: &LitStr) -> Result<Ident, syn::Error> {
    value.parse::<Ident>()
}

fn parse_ident_list(value: &LitStr) -> Result<Vec<Ident>, syn::Error> {
    Ok(value
        .value()
        .split(',')
        .map(str::trim)
        .map(|trait_name| Ident::new(trait_name, Span::call_site()))
        .collect())
}

fn name_value_pair_list(
    struct_name: &Ident,
    attr: &Attribute,
) -> Result<Vec<NameValuePair>, syn::Error> {
    if let Meta::List(list) = attr.parse_meta()? {
        list.nested.iter().map(name_value_pair).collect()
    } else {
        let msg = format!(
            r#"float_eq attribute must be a list of options, for example `#[float_eq(ulps_tol = "{}Ulps")]`"#,
            struct_name
        );
        Err(syn::Error::new(attr.path.span(), msg))
    }
}

pub struct NameValuePair {
    pub name: Ident,
    pub value: LitStr,
}

pub fn name_value_pair(meta: &NestedMeta) -> Result<NameValuePair, syn::Error> {
    if let NestedMeta::Meta(Meta::NameValue(nv)) = meta {
        if let Some(name) = nv.path.get_ident() {
            if let Lit::Str(value) = &nv.lit {
                return Ok(NameValuePair {
                    name: name.clone(),
                    value: value.clone(),
                });
            }
        }
    }
    Err(syn::Error::new(
        meta.span(),
        "Expected a `name = \"value\"` pair.",
    ))
}
