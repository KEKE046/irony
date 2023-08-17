use super::common::Id;
use super::environ::Environ;
use super::operation::OpId;

pub trait Entity: Id {
    type DataTypeT;
    type AttributeT: Clone + PartialEq + std::fmt::Display;
    fn get_dtype(&self) -> Option<Self::DataTypeT>;

    fn get_defs<E: Environ>(&self, env: &E) -> Vec<OpId>;
    fn get_uses<E: Environ>(&self, env: &E) -> Vec<OpId>;
    fn as_id(&self) -> EntityId;
    fn get_parent(&self) -> Option<RegionId>;
    fn set_parent(&mut self, parent: Option<RegionId>);
    fn get_attrs(&self) -> Vec<(String, Self::AttributeT)>;
    fn set_attrs(&mut self, attrs: Vec<(String, Self::AttributeT)>);

    fn update_attrs<F>(&mut self, name: &str, f: F) -> ()
    where F: Fn(Self::AttributeT) -> Self::AttributeT {
        let updated_attrs: Vec<_> = self
            .get_attrs()
            .iter()
            .map(|(attr_name, attr)| {
                if attr_name == name {
                    (attr_name.to_owned(), f(attr.to_owned()))
                } else {
                    (attr_name.to_owned(), attr.to_owned())
                }
            })
            .collect();
        self.set_attrs(updated_attrs)
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct EntityId(pub usize);

impl From<usize> for EntityId {
    fn from(value: usize) -> Self { Self(value) }
}
impl Id for EntityId {
    fn id(&self) -> usize { self.0 }

    fn set_id(&mut self, id: usize) { self.0 = id }
}

impl EntityId {
    pub fn get<'env: 't, 't, E>(&'t self, env: &'env E) -> &'t E::EntityT
    where E: Environ {
        env.get_entity(self.to_owned())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Region {
    pub id: usize,
    pub isolated: bool,
    pub op_children: Vec<OpId>,
    pub entity_children: Vec<EntityId>,
}

impl Id for Region {
    fn id(&self) -> usize { self.id }

    fn set_id(&mut self, id: usize) { self.id = id }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RegionId(pub usize);
impl Id for RegionId {
    fn id(&self) -> usize { self.0 }

    fn set_id(&mut self, id: usize) { self.0 = id }
}

impl Region {
    pub fn get_use<E: Environ>(&self, env: &E) -> Option<OpId> {
        env.get_region_use(self.as_id())
    }

    pub fn as_id(&self) -> RegionId { RegionId(self.id) }

    pub fn new(isolated: bool) -> Self {
        Self {
            id: 0,
            isolated,
            op_children: vec![],
            entity_children: vec![],
        }
    }

    pub fn add_op_child(&mut self, op: OpId) {
        if let Some(_) =
            self.op_children.iter().find(|&op_exist| op_exist.id() == op.id())
        {
            panic!("{} has already been in the op_children of {}\n", op.id(), self.id())
        } else {
            self.op_children.push(op)
        }
    }

    pub fn add_entity_child(&mut self, entity: EntityId) {
        if let Some(_) = self
            .entity_children
            .iter()
            .find(|&entity_exist| entity_exist.id() == entity.id())
        {
            panic!(
                "{} has already been in the entity_children of {}",
                entity.id(),
                self.id()
            )
        } else {
            self.entity_children.push(entity)
        }
    }
}

#[macro_export]
macro_rules! entity_def {
    (
        [data_type = $data_type:ty, attr = $attr_ty:ty]
        $name_enum:ident = {
            $($name:ident $(: [$($attr:ident : $attr_variant:ident($attr_inner_ty:ty)),*])?),+
            $(,)?
        }
    ) => {
        $(irony::entity_def_one! {
            $name : ($(attrs = [$($attr: $attr_variant($attr_inner_ty))*],)? data_type = $data_type, attr = $attr_ty)
        })*

        irony::entity_enum! {
            [data_type = $data_type, attr = $attr_ty]
            $name_enum = $($name),*
        }
    };
}

#[macro_export]
macro_rules! entity_def_one {


    ($name:ident : ($(attrs = [$($attr:ident: $attr_variant:ident($attr_inner_ty:ty))*],)? data_type = $data_type:ty, attr = $attr_ty:ty)) => {
        #[derive(Clone, Debug, PartialEq)]
        pub struct $name {
            pub id: usize,
            pub parent: Option<irony::RegionId>,
            pub dtype: Option<$data_type>,
            $($(pub $attr: Option<$attr_inner_ty>,)*)?
        }

        impl irony::Entity for $name {
            type DataTypeT = $data_type;
            type AttributeT = $attr_ty;

            fn get_defs<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
                env.get_defs(self.as_id())
            }

            fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
                env.get_uses(self.as_id())
            }

            fn get_dtype(&self) -> Option<Self::DataTypeT> {
                self.dtype.to_owned()
            }

            fn as_id(&self) -> irony::EntityId {
                irony::EntityId(self.id)
            }

            fn get_parent(&self) -> Option<irony::RegionId> {
                self.parent
            }

            fn set_parent(&mut self, parent: Option<irony::RegionId>) {
                self.parent = parent
            }
            fn get_attrs(&self) -> Vec<(String, Self::AttributeT)> {
                let mut attrs = vec![];
                $(
                    $(
                        if let Some(attr) = self.$attr.to_owned() {
                            attrs.push((String::from(stringify!($attr)), attr.into()))
                        }
                    )*
                )?
                attrs
            }

            fn set_attrs(&mut self, attrs: Vec<(String, Self::AttributeT)>) -> () {
                $(
                    $(
                        if let Some((name, attr)) = attrs.iter().find(|(name, _)| name == &String::from(stringify!($attr))) {
                            self.$attr = Some(attr.to_owned().into())
                        }
                    )*
                )?
            }
        }

        impl irony::Id for $name {
            fn id(&self) -> usize {
                self.id
            }
            fn set_id(&mut self, id: usize) {
                self.id = id
            }
        }

        impl $name {
            pub const fn const_new (dtype: Option<$data_type>) -> Self {
                Self {
                    id: 0,
                    dtype: dtype,
                    parent: None,
                    $($($attr: None),*)?
                }
            }
            pub fn new(dtype: Option<$data_type>, $($($attr: Option<$attr_inner_ty>),*)?) -> Self {
                Self {
                    id: 0,
                    dtype: dtype,
                    parent: None,
                    $($($attr: $attr.map(|x| x.into())),*)?
                }
            }
        }
    };


}

#[macro_export]
macro_rules! entity_enum {
    ([data_type = $dtype:ty, attr = $attr_ty: ty] $name:ident= $($variant:ident),*) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $($variant($variant)),*
        }

        impl irony::Entity for $name {
            type DataTypeT = $dtype;
            type AttributeT = $attr_ty;

            fn get_defs<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
                match self {
                    $($name::$variant(inner) => inner.get_defs(env),)*
                }
            }

            fn get_uses<E: irony::Environ>(&self, env: &E) -> Vec<irony::OpId> {
                match self {
                    $($name::$variant(inner) => inner.get_uses(env),)*
                }
            }

            fn get_dtype(&self) -> Option<Self::DataTypeT> {
                match self {
                    $($name::$variant(inner) => inner.get_dtype(),)*
                }
            }

            fn as_id(&self) -> irony::EntityId {
                match self {
                    $($name::$variant(inner) => inner.as_id(), )*
                }
            }

            fn get_parent(&self) -> Option<irony::RegionId> {
                match self {
                    $($name::$variant(inner) => inner.get_parent(), )*
                }
            }

            fn set_parent(&mut self, parent: Option<irony::RegionId>) {
                match self {
                    $($name::$variant(inner) => inner.set_parent(parent), )*
                }
            }

            fn get_attrs(&self) -> Vec<(String, Self::AttributeT)> {
                match self {
                    $($name::$variant(inner) => inner.get_attrs(), )*
                }
            }
            fn set_attrs(&mut self, attrs: Vec<(String, Self::AttributeT)>) -> () {
                match self {
                    $($name::$variant(inner) => inner.set_attrs(attrs), )*
                }
            }
        }

        impl irony::Id for $name {
            fn id(&self) -> usize {
                match self {
                    $($name::$variant(inner) => inner.id(), )*
                }
            }
            fn set_id(&mut self, id: usize) {
                match self {
                    $($name::$variant(inner) => inner.set_id(id), )*
                }
            }
        }

        $(
            impl Into<$name> for $variant {
                fn into(self) -> $name {
                    $name::$variant(self)
                }
            }
        )*

        $(
            impl Into<$variant> for $name {
                fn into(self) -> $variant {
                    match self {
                        $name::$variant(inner) => inner,
                        _ => panic!("match fails, check variant {} and enum {}", stringify!($variant), stringify!($name))
                    }
                }
            }
        )*


    };
}
