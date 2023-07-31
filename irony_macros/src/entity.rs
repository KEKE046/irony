
use crate::options::AllowedOptions;
use crate::utils::IronyEnum;
use crate::utils::IronyEnumKind;
use crate::utils::IronyStruct;

use crate::utils::IronyStructKind;
pub(crate) fn entity(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match IronyStruct::new(IronyStructKind::Entity, args, input)
        .and_then(|base| EntityStruct(base).generate())
    {
        Ok(s) => s.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

struct EntityStruct(IronyStruct<Self>);

impl std::ops::Deref for EntityStruct {
    type Target = IronyStruct<Self>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AllowedOptions for EntityStruct {
    const ENTITY: bool = false;

    const OP: bool = false;

    const DTYPE: bool = true;

    const HAS_DATA: bool = true;

    const ATTRIBUTE: bool = false;

    const CONSTRAINT: bool = false;
}

impl EntityStruct {
    fn generate(&self) -> syn::Result<proc_macro2::TokenStream> {
        let expanded_struct = self.gen_struct();
        let entity_impl = self.gen_impl_entity();
        let id_impl = self.gen_impl_id();
        let self_impl = self.gen_impl_self();

        Ok(quote! {
            #expanded_struct
            #entity_impl
            #id_impl
            #self_impl
        })

    }


    fn gen_struct(&self) -> syn::ItemStruct {
        let ident = self.ident();
        let vis = self.visibility();

        let attrs = self.attributes();

        let field_idents = self.all_field_names();
        let field_tys = self.all_field_tys();

        let dtype = self.dtype_ty();

        parse_quote! {
            #(#attrs)*
            #[derive(Clone, Debug, PartialEq)]
            #vis struct #ident {
                pub id: usize,
                pub sym: irony::Symbol, 
                pub parent: Option<irony::RegionId>,
                pub dtype: Option<#dtype>,

                #(pub #field_idents: #field_tys)*
            }

        }
    }

    fn gen_impl_entity(&self) -> syn::ItemImpl {

        let ident = self.ident();

        let dtype_ty = self.dtype_ty();

        parse_quote! {
            impl irony::Entity for #ident {
                type DataTypeT = #dtype_ty;

                fn get_def<E: irony::Environ>(&self, env: &E) -> Option<irony::OpId> {
                    env.get_def(self)
                }
        
                fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
                    env.get_uses(self)
                }
        
                fn get_dtype(&self) -> Option<Self::DataTypeT> {
                    self.dtype
                }
        
                fn as_id(&self) -> irony::EntityId {
                    irony::EntityId(self.id)
                }
        
                fn get_parent(&self) -> Option<irony::RegionId> {
                    self.parent
                }
        
                fn set_parent(&mut self, parent: irony::RegionId) {
                    self.parent = Some(parent)
                }
            }
        }

    }
    
    fn gen_impl_id(&self) -> syn::ItemImpl {
        let ident = self.ident();
        parse_quote! {
            impl irony::Id for #ident {
                fn id(&self) -> usize {
                    self.id
                }
                fn set_id(&mut self, id: usize) {
                    self.id = id
                }
            }
        }
    }

    fn gen_impl_self(&self) -> syn::ItemImpl {
        let ident = self.ident();
        if let Some(true) = self.has_data() {
            let dtype = self.dtype_ty();
            parse_quote! {
                impl #ident {
                    pub fn new(name: &str, dtype: #dtype) -> Self {
                        Self {
                            id: 0,
                            sym: irony::Symbol::new(String::from(name)),
                            dtype: Some(dtype),
                            parent: None
                        }

                    }
                }
            }
        } else {
            parse_quote!{
                impl #ident {
                    pub fn new(name: &str) -> Self {
                        Self {
                            id: 0,
                            sym: irony::Symbol::new(String::from(name)),
                            dtype: None,
                            parent: None
                        }

                    }
                }
            }
        }

    }
    
}


pub(crate) fn entity_enum(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match IronyEnum::new(IronyEnumKind::EntityEnum, args, input).and_then(|base| EntityEnumStruct(base).generate()) {
        Ok(s) => s.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

struct EntityEnumStruct(IronyEnum<Self>);

impl std::ops::Deref for EntityEnumStruct {
    type Target = IronyEnum<Self>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AllowedOptions for EntityEnumStruct {
    const ENTITY: bool = false;

    const OP: bool = false;

    const DTYPE: bool = true;

    const HAS_DATA: bool = false;

    const ATTRIBUTE: bool = false;

    const CONSTRAINT: bool = false;
}

impl EntityEnumStruct {
    pub(crate) fn generate(&self) -> syn::Result<proc_macro2::TokenStream> {
        let expanded_enum = self.gen_enum();
        let gen_impl_entity = self.gen_impl_entity();
        let gen_impl_id = self.gen_impl_id();
        let gen_variant_into_enum = self.gen_variant_into_enum();
        let gen_enum_into_variant = self.gen_enum_into_variant();

        Ok(quote!{
            #expanded_enum
            #gen_impl_entity
            #gen_impl_id
            #(#gen_variant_into_enum)*
            #(#gen_enum_into_variant)*
        })
    }

    pub(crate) fn gen_enum(&self) -> syn::ItemEnum {

        let ident = self.ident();
        let attributes = self.attributes();
        let vis = self.visibility();
        let variant_idents = self.all_variant_idents();

        parse_quote! {
            #(#attributes)*
            #[derive(Clone, Debug, PartialEq)]
            #vis enum #ident {
                #(#variant_idents(#variant_idents)),*
            }
        }
    }

    pub(crate) fn gen_impl_entity(&self) -> syn::ItemImpl {
        
        let ident = self.ident();
        let variant_idents = self.all_variant_idents();
        let dtype_ty = self.dtype();

        parse_quote! {
            impl irony::Entity for #ident {
                type DataTypeT = #dtype_ty;

                fn get_def<E: irony::Environ>(&self, env: &E) -> Option<irony::OpId> {
                    match self {
                        #(#ident::#variant_idents(inner) => inner.get_def(env),)*
                    }
                }
        
                fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
                    match self {
                        #(#ident::#variant_idents(inner) => inner.get_uses(env),)*
                    }
                }
        
                fn get_dtype(&self) -> Option<Self::DataTypeT> {
                    match self {
                        #(#ident::#variant_idents(inner) => inner.get_dtype(),)*
                    }
                }
        
                fn as_id(&self) -> irony::EntityId {
                    match self {
                        #(#ident::#variant_idents(inner) => inner.as_id(), )*
                    }
                }
        
                fn get_parent(&self) -> Option<irony::RegionId> {
                    match self {
                        #(#ident::#variant_idents(inner) => inner.get_parent(), )*
                    }
                }
        
                fn set_parent(&mut self, parent: irony::RegionId) {
                    match self {
                        #(#ident::#variant_idents(inner) => inner.set_parent(parent), )*
                    }
                }
            }
        }

    }

    pub(crate) fn gen_impl_id(&self) -> syn::ItemImpl {
        let ident = self.ident();
        let variant_idents = self.all_variant_idents();

        parse_quote!(
            impl irony::Id for #ident {
                fn id(&self) -> usize {
                    match self {
                        #(#ident::#variant_idents(inner) => inner.id(), )*
                    }
                }
                fn set_id(&mut self, id: usize) {
                    match self {
                        #(#ident::#variant_idents(inner) => inner.set_id(id), )*
                    }
                }
            }

        )

    }

    pub(crate) fn gen_variant_into_enum(&self) -> Vec<syn::ItemImpl> {
        let ident = self.ident();
        let variant_idents = self.all_variant_idents();

        let mut v = Vec::new();

        for variant_ident in variant_idents {
            v.push(parse_quote! {
                impl Into<#ident> for #variant_ident {
                    fn into(self) -> #ident {
                        #ident::#variant_ident(self)
                    }
                }
            });
        }
        v
    }

    pub(crate) fn gen_enum_into_variant(&self) -> Vec<syn::ItemImpl> {
        let ident = self.ident() ;
        let variant_idents = self.all_variant_idents();
        let ident_str = ident.to_owned().to_string();

        let mut v = Vec::new();

        for variant_ident in variant_idents {
            let variant_ident_str = variant_ident.to_owned().to_string();
            v.push( parse_quote! {
                impl Into<#variant_ident> for #ident {
                    fn into(self) -> #variant_ident {
                        match self {
                            #ident::#variant_ident(inner) => inner,
                            _ => panic!("match fails, check variant {} and enum {}", #ident_str, #variant_ident_str)
                        }
                    }
                }
            });
        }
        v
    }
}
