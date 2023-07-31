//! Common code for irony macros on structs
//!
//! Example of usage:
//!
//! ```rust,ignore
//! #[irony::entity]
//! pub struct Wire;
//! ```

use syn::Attribute;

use crate::options::{AllowedOptions, Options};

pub(crate) enum IronyStructKind {
    Entity,
    Op,
}

pub(crate) struct IronyStruct<A: AllowedOptions> {
    kind: IronyStructKind,
    args: Options<A>,
    struct_item: syn::ItemStruct,
    fields: Vec<IronyField>,
}

impl<A: AllowedOptions> IronyStruct<A> {
    pub(crate) fn new(
        kind: IronyStructKind,
        args: proc_macro::TokenStream,
        input: proc_macro::TokenStream,
    ) -> syn::Result<Self> {
        let struct_item = syn::parse(input)?;
        Self::with_struct(kind, args, struct_item)
    }

    pub(crate) fn with_struct(
        kind: IronyStructKind,
        args: proc_macro::TokenStream,
        struct_item: syn::ItemStruct,
    ) -> syn::Result<Self> {
        let args: Options<A> = syn::parse(args)?;
        let fields = Self::extract_options(&struct_item)?;
        Ok(Self {
            kind,
            args,
            struct_item,
            fields,
        })
    }

    /// Extract out the fields and their options:
    /// If this is a struct, it must use named fields or no fields, so we can define field accessors.
    /// If it is an enum, then this is not necessary.
    pub(crate) fn extract_options(struct_item: &syn::ItemStruct) -> syn::Result<Vec<IronyField>> {
        match &struct_item.fields {
            syn::Fields::Named(n) => Ok(n
                .named
                .iter()
                .map(IronyField::new)
                .collect::<syn::Result<Vec<_>>>()?),
            syn::Fields::Unit => Ok(vec![]),
            f => Err(syn::Error::new_spanned(
                f,
                "must have named fields or no fields for a struct",
            )),
        }
    }

    /// Iterator over all named fields.
    ///
    /// If this is an enum, empty iterator.
    pub(crate) fn all_fields(&self) -> impl Iterator<Item = &IronyField> {
        self.fields.iter()
    }

    /// all fields
    pub(crate) fn all_fields_as_syn(&self) -> Vec<&syn::Field> {
        self.all_fields().map(|ef| &ef.field).collect()
    }

    /// Names of all fields (id and value).
    /// If this is an enum, empty vec.
    pub(crate) fn all_field_names(&self) -> Vec<&syn::Ident> {
        self.all_fields().map(|ef| ef.name()).collect()
    }

    /// Visibilities of all fields
    pub(crate) fn all_field_vises(&self) -> Vec<&syn::Visibility> {
        self.all_fields().map(|ef| ef.vis()).collect()
    }

    /// Types of all fields (id and value).
    /// If this is an enum, empty vec.
    pub(crate) fn all_field_tys(&self) -> Vec<&syn::Type> {
        self.all_fields().map(|ef| ef.ty()).collect()
    }

    /// The name of the "identity" struct (this is the name the user gave, e.g., `Foo`).
    pub(crate) fn ident(&self) -> &syn::Ident {
        &self.struct_item.ident
    }

    /// Type of the entity_enum for this struct
    pub(crate) fn entity_enum_ty(&self) -> syn::Type {
        self.args.entity_enum_ty()
    }

    /// Type of the op_enum for this struct
    pub(crate) fn op_enum_ty(&self) -> syn::Type {
        self.args.op_enum_ty()
    }

    /// Type of the dtype for this struct
    pub(crate) fn dtype_ty(&self) -> syn::Type {
        self.args.dtype_ty()
    }

    pub(crate) fn has_data(&self) -> Option<bool> {
        self.args.has_data
    }

    /// Returns the visibility of this item
    pub(crate) fn visibility(&self) -> &syn::Visibility {
        &self.struct_item.vis
    }

    pub(crate) fn attributes(&self) -> Vec<&Attribute> {
        self.struct_item.attrs.iter().collect()
    }
}

pub(crate) struct IronyField {
    field: syn::Field,
}

impl IronyField {
    pub(crate) fn new(field: &syn::Field) -> syn::Result<Self> {
        Ok(Self {
            field: field.clone(),
        })
    }

    /// The name of this field (all `IronyField` instances are named).
    pub(crate) fn name(&self) -> &syn::Ident {
        self.field.ident.as_ref().unwrap()
    }

    /// The visibility of this field.
    pub(crate) fn vis(&self) -> &syn::Visibility {
        &self.field.vis
    }

    /// The type of this field (all `IronyField` instances are named).
    pub(crate) fn ty(&self) -> &syn::Type {
        &self.field.ty
    }
}

pub(crate) enum IronyEnumKind {
    EntityEnum,
    OpEnum,
    DataEnum,
}

pub(crate) struct IronyEnum<A: AllowedOptions> {
    kind: IronyEnumKind,
    args: Options<A>,
    enum_item: syn::ItemEnum,
    variants: Vec<IronyVariant>,
}

impl<A: AllowedOptions> IronyEnum<A> {
    pub(crate) fn new(
        kind: IronyEnumKind,
        args: proc_macro::TokenStream,
        input: proc_macro::TokenStream,
    ) -> syn::Result<Self> {
        let enum_item = syn::parse(input)?;
        Self::with_enum(kind, args, enum_item)
    }

    pub(crate) fn with_enum(
        kind: IronyEnumKind,
        args: proc_macro::TokenStream,
        enum_item: syn::ItemEnum,
    ) -> syn::Result<Self> {
        let args: Options<A> = syn::parse(args)?;
        let variants = Self::extract_options(&enum_item)?;
        Ok(Self {
            kind,
            args,
            enum_item,
            variants,
        })
    }

    /// Extract out the fields and their options:
    /// If this is a struct, it must use named fields or no fields, so we can define field accessors.
    /// If it is an enum, then this is not necessary.
    pub(crate) fn extract_options(enum_item: &syn::ItemEnum) -> syn::Result<Vec<IronyVariant>> {
        Ok(enum_item
            .variants
            .iter()
            .map(IronyVariant::new)
            .collect::<syn::Result<Vec<_>>>()?)
    }

    /// Returns the ident of this item
    pub(crate) fn ident(&self) -> &syn::Ident {
        &self.enum_item.ident
    }

    /// Returns the visibility of this item
    pub(crate) fn visibility(&self) -> &syn::Visibility {
        &self.enum_item.vis
    }

    pub(crate) fn attributes(&self) -> Vec<&Attribute> {
        self.enum_item.attrs.iter().collect()
    }

    pub(crate) fn dtype(&self) -> syn::Type {
        self.args.dtype_ty()
    }

    /// Iterator over all variants.
    pub(crate) fn all_fields(&self) -> impl Iterator<Item = &IronyVariant> {
        self.variants.iter()
    }
    pub(crate) fn all_variant_idents(&self) -> Vec<syn::Ident> {
        self.all_fields().map(|x| x.ident() ).collect()
    }
}

pub(crate) struct IronyVariant {
    variant: syn::Variant,
}

impl IronyVariant {
    pub(crate) fn new(variant: &syn::Variant) -> syn::Result<Self> {
        Ok(Self {
            variant: variant.clone(),
        })
    }

    pub(crate) fn ident(&self) -> syn::Ident {
        self.variant.ident.clone()
    }

}
