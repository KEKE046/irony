#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    name: Option<String>,
}

impl Symbol {
    pub fn new(name: String) -> Self { Self { name: Some(name) } }
}

pub trait Id {
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}

impl Id for usize {
    fn id(&self) -> usize { *self }

    fn set_id(&mut self, _id: usize) { panic!("cannot set id to usize") }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstValueU32<D: Clone> {
    pub value: u32,
    pub dtype: D,
}

impl<D: Clone> std::fmt::Display for ConstValueU32<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[macro_export]
macro_rules! data_type_enum {
    (
        $enum_name:ident = {
            $($variant:ident($variant_ty:ident)),*
            $(,)?
        }
    ) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $enum_name {
            $($variant($variant_ty)),*
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($enum_name::$variant(inner) => write!(f, "{}", inner)),*
                }
            }
        }
    };
}

#[macro_export]
macro_rules! attribute_enum {

    (
        [data_type = $dtype: ty]
        $name:ident = {
            $($variant:ident($variant_ty:ty)),*
            $(,)?
        }
    ) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $($variant($variant_ty)),*
        }

        $(
            impl Into<$name> for $variant_ty {
                fn into(self) -> $name {
                    $name::$variant(self.to_owned())
                }
            }

            impl Into<$variant_ty> for $name {
                fn into(self) -> $variant_ty {
                    match self {
                        $name::$variant(x) => x,
                        _ => panic!("cannot convert {:?} into {:?}", stringify!($name), stringify!($variant_ty)),
                    }
                }
            }
        )*

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $($name::$variant(x) => write!(f, "{}", x)),*
                }
            }
        }
    };


}
