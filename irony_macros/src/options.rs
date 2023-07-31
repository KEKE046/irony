use std::marker::PhantomData;

use syn::{ext::IdentExt, spanned::Spanned};

/// "Options" are flags that can be supplied to the various irony related
/// macros. They are listed like `(ref, no_eq, foo=bar)` etc. The commas
/// are required and trailing commas are permitted. The options accepted
/// for any particular location are configured via the `AllowedOptions`
/// trait.
pub(crate) struct Options<A: AllowedOptions> {
    // /// The `has_parent` option is used to indicate that the entity has a parent
    // pub has_parent: Option<syn::Ident>,
    /// `entity = <type>`
    pub entity_enum_ty: Option<syn::Type>,

    /// `op = <type>`
    pub op_enum_ty: Option<syn::Type>,

    /// `dtype = <type>`
    pub dtype_ty: Option<syn::Type>,
    
    /// `has_data = <bool>`
    pub has_data: Option<bool>,

    /// `attribute = <type>`
    pub attribute_ty: Option<syn::Type>,

    /// `constraint = <type>`
    pub constraint_ty: Option<syn::Type>,

    phantom: PhantomData<A>,
}

impl<A: AllowedOptions> Default for Options<A> {
    fn default() -> Self {
        Self {
            entity_enum_ty: Default::default(),
            op_enum_ty: Default::default(),
            dtype_ty: Default::default(),
            has_data: Default::default(),
            attribute_ty: Default::default(),
            constraint_ty: Default::default(),
            phantom: PhantomData,
        }
    }
}

/// These flags determine which options are allowed in a given context
pub(crate) trait AllowedOptions {
    const ENTITY: bool;
    const OP: bool;
    const DTYPE: bool;
    const ATTRIBUTE: bool;
    const CONSTRAINT: bool; 

    const HAS_DATA: bool;
}

type Equals = syn::Token![=];
type Comma = syn::Token![,];

impl<A: AllowedOptions> Options<A> {
    pub(crate) fn entity_enum_ty(&self) -> syn::Type {
        if let Some(entity_enum_ty) = &self.entity_enum_ty {
            return entity_enum_ty.clone();
        }

        parse_quote! {crate::EntityEnum}
    }
    pub(crate) fn op_enum_ty(&self) -> syn::Type {
        if let Some(op_enum_ty) = &self.op_enum_ty {
            return op_enum_ty.clone();
        }

        parse_quote! {crate::OpEnum}
    }
    pub(crate) fn dtype_ty(&self) -> syn::Type {
        if let Some(dtype_ty) = &self.dtype_ty {
            return dtype_ty.clone();
        }
        parse_quote! {crate::DataTypeEnum}
    }

    pub(crate) fn attribute_ty(&self) -> syn::Type {
        if let Some(attribute_ty) = &self.attribute_ty {
            return attribute_ty.clone();
        }
        parse_quote! {crate::AttributeEnum}
    }

    pub(crate) fn constraint_ty(&self) -> syn::Type {
        if let Some(constraint_ty) = &self.constraint_ty {
            return constraint_ty.clone();
        }
        parse_quote! {crate::ConstraintEnum}
    }
}

impl<A: AllowedOptions> syn::parse::Parse for Options<A> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut options = Options::default();

        while !input.is_empty() {
            let ident: syn::Ident = syn::Ident::parse_any(input)?;

            if ident == "entity" {
                if A::ENTITY {
                    let _eq = Equals::parse(input)?;
                    let ty = syn::Type::parse(input)?;
                    if let Some(old) = std::mem::replace(&mut options.entity_enum_ty, Some(ty)) {
                        return Err(syn::Error::new(
                            old.span(),
                            "option `entity_enum_ty` provided twice",
                        ));
                    }
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "`entity_enum_ty` option not allowed here",
                    ));
                }
            } else if ident == "op" {
                if A::OP {
                    let _eq = Equals::parse(input)?;
                    let ty = syn::Type::parse(input)?;
                    if let Some(old) = std::mem::replace(&mut options.op_enum_ty, Some(ty)) {
                        return Err(syn::Error::new(
                            old.span(),
                            "option `op_enum_ty` provided twice",
                        ));
                    }
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "`op_enum_ty` option not allowed here",
                    ));
                }
            } else if ident == "dtype" {
                if A::DTYPE {
                    let _eq = Equals::parse(input)?;
                    let ty = syn::Type::parse(input)?;
                    if let Some(old) = std::mem::replace(&mut options.dtype_ty, Some(ty)) {
                        return Err(syn::Error::new(
                            old.span(),
                            "option `dtype_ty` provided twice",
                        ));
                    }
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "`dtype_ty` option not allowed here",
                    ));
                }
            }  else if ident == "attribute" {
                if A::ATTRIBUTE {
                    let _eq = Equals::parse(input)?;
                    let ty = syn::Type::parse(input)?;
                    if let Some(old) = std::mem::replace(&mut options.attribute_ty, Some(ty)) {
                        return Err(syn::Error::new(
                            old.span(),
                            "option `attribute_ty` provided twice",
                        ));
                    }
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "`attribute_ty` option not allowed here",
                    ));
                }
            } else if ident == "constraint" {
                if A::CONSTRAINT {
                    let _eq = Equals::parse(input)?;
                    let ty = syn::Type::parse(input)?;
                    if let Some(old) = std::mem::replace(&mut options.constraint_ty, Some(ty)) {
                        return Err(syn::Error::new(
                            old.span(),
                            "option `constraint_ty` provided twice",
                        ));
                    }
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "`constraint_ty` option not allowed here",
                    ));
                }
            }else if ident == "has_data" {
                if A::DTYPE {
                    let _eq = Equals::parse(input)?;
                    let has = syn::LitBool::parse(input)?;
                    if let Some(old) = std::mem::replace(&mut options.has_data, Some(has.value())) {
                        return Err(syn::Error::new(
                            old.span(),
                            "option `dtype_ty` provided twice",
                        ));
                    }
                } else {
                    return Err(syn::Error::new(
                        ident.span(),
                        "`dtype_ty` option not allowed here",
                    ));
                }
            }

            if input.is_empty() {
                break;
            }

            let _comma = Comma::parse(input)?;
        }

        Ok(options)
    }
}
