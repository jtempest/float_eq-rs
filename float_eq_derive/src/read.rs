use proc_macro2::{Ident, Span, TokenStream};
use quote::ToTokens;
use syn::{
    spanned::Spanned, Attribute, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, Lit,
    LitInt, Meta, NestedMeta, Type,
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
        self.fields.iter().map(|f| func(f)).collect()
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
    }
}

fn unnamed_field_info((n, field): (usize, &syn::Field)) -> FieldInfo {
    FieldInfo {
        name: FieldName::Num(Lit::Int(LitInt::new(&format!("{}", n), Span::call_site()))),
        ty: &field.ty,
    }
}

#[derive(Default)]
pub struct FloatEqAttr {
    struct_name: String,
    ulps_epsilon_type_name: Option<Ident>,
    debug_ulps_diff_type_name: Option<Ident>,
    all_epsilon_type_name: Option<Ident>,
}

impl FloatEqAttr {
    pub fn ulps_epsilon_type(&self) -> Result<&Ident, syn::Error> {
        self.ulps_epsilon_type_name.as_ref().ok_or({
            let msg = format!(
                r#"Missing epsilon ULPs type name required to derive trait.

help: try adding `#[float_eq(ulps_epsilon = "{}Ulps")]` to your type."#,
                self.struct_name
            );
            syn::Error::new(Span::call_site(), msg)
        })
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

    pub fn all_epsilon_type(&self) -> Result<&Ident, syn::Error> {
        self.all_epsilon_type_name.as_ref().ok_or({
            let msg = format!(
                r#"Missing Epsilon type name required to derive trait.

help: try adding `#[float_eq(all_epsilon = "T")]` to your type, where T is commonly `f32` or `f64`."#
            );
            syn::Error::new(Span::call_site(), msg)
        })
    }
}

pub fn float_eq_attr(input: &DeriveInput) -> Result<FloatEqAttr, syn::Error> {
    let nv_pair_lists: Vec<Vec<NameTypePair>> = input
        .attrs
        .iter()
        .filter(|a| a.path.is_ident("float_eq"))
        .map(|a| name_type_pair_list(&input.ident, a))
        .collect::<Result<_, _>>()?;

    let mut attr_values = FloatEqAttr {
        struct_name: input.ident.to_string(),
        ..Default::default()
    };
    for nv in nv_pair_lists.into_iter().flatten() {
        let name = nv.name.to_string();
        if name == "ulps_epsilon" {
            if attr_values.ulps_epsilon_type_name.is_none() {
                attr_values.ulps_epsilon_type_name = Some(nv.value);
            } else {
                let msg = format!(
                    r#"Expected only one epsilon ULPs type name, previously saw `ulps_epsilon = "{}"`."#,
                    attr_values.ulps_epsilon_type_name.unwrap().to_string()
                );
                return Err(syn::Error::new(nv.value.span(), msg));
            }
        } else if name == "debug_ulps_diff" {
            if attr_values.debug_ulps_diff_type_name.is_none() {
                attr_values.debug_ulps_diff_type_name = Some(nv.value);
            } else {
                let msg = format!(
                    r#"Expected only one debug ULPs diff type name, previously saw `debug_ulps_diff = "{}"`."#,
                    attr_values.debug_ulps_diff_type_name.unwrap().to_string()
                );
                return Err(syn::Error::new(nv.value.span(), msg));
            }
        } else if name == "all_epsilon" {
            if attr_values.all_epsilon_type_name.is_none() {
                attr_values.all_epsilon_type_name = Some(nv.value);
            } else {
                let msg = format!(
                    r#"Expected only one Epsilon type name, previously saw `all_epsilon = "{}"`."#,
                    attr_values.all_epsilon_type_name.unwrap().to_string()
                );
                return Err(syn::Error::new(nv.value.span(), msg));
            }
        } else {
            let msg = r"Not a valid float_eq derive option.";
            return Err(syn::Error::new(nv.name.span(), msg));
        }
    }

    Ok(attr_values)
}

fn name_type_pair_list(
    struct_name: &Ident,
    attr: &Attribute,
) -> Result<Vec<NameTypePair>, syn::Error> {
    if let Meta::List(list) = attr.parse_meta()? {
        list.nested.iter().map(name_type_pair).collect()
    } else {
        let msg = format!(
            r#"float_eq attribute must be a list of options, for example `#[float_eq(ulps_epsilon = "{}Ulps")]`"#,
            struct_name.to_string()
        );
        Err(syn::Error::new(attr.span(), msg))
    }
}

pub struct NameTypePair {
    pub name: Ident,
    pub value: Ident,
}

pub fn name_type_pair(meta: &NestedMeta) -> Result<NameTypePair, syn::Error> {
    if let NestedMeta::Meta(Meta::NameValue(nv)) = meta {
        if let Some(name) = nv.path.get_ident() {
            if let Lit::Str(value) = &nv.lit {
                if let Ok(value) = value.parse::<Ident>() {
                    return Ok(NameTypePair {
                        name: name.clone(),
                        value: value.clone(),
                    });
                }
            }
        }
    }
    Err(syn::Error::new(
        meta.span(),
        "Expected a `name = value` pair.",
    ))
}
