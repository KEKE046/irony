
#[derive(Clone, Debug, PartialEq)]
pub struct Symbol {
    name: Option<String>,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Self { name: Some(name) }
    }
}

pub trait Id {
    fn id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}

impl Id for usize {
    fn id(&self) -> usize {
        *self
    }
    fn set_id(&mut self, _id: usize) {
        panic!("cannot set id to usize")
    }
}

pub trait AttributeTrait<DT> {
    fn dtype(&self) -> DT;
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstValueI32<D:Clone> {
    pub value: i32,
    pub dtype: D,
}

impl<D:Clone> AttributeTrait<D> for ConstValueI32<D> {
    fn dtype(&self) -> D {
        self.dtype.to_owned()
    }
}

#[macro_export]
macro_rules! data_type_enum {
    ($enum_name:ident = $($variant:ident$(($($inner:ident),*))?),*) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $enum_name {
            $($variant$(($($inner),*))?),*
        }
    };
}

#[macro_export]
macro_rules! attribute_enum {
    ($enum_name:ident : $data_type:ident = $($variant:ident($variant_ty:ty)),*) => {
        $attribute_enum! {
            [data_type = $data_type]
            $enum_name = $($variant:ident($variant_ty:ty)),*
        }
    };

    ([data_type = $dtype: ty] $name:ident = $($variant:ident($variant_ty:ty)),*) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $name {
            $($variant($variant_ty)),*
        }
        impl irony::AttributeTrait<$dtype> for $name {
            fn dtype(&self) -> $dtype {
                match self {
                    $($name::$variant(inner) => inner.dtype()),*
                }
            }
        }

        $(
            impl Into<$name> for $variant_ty {
                fn into(self) -> $name {
                    $name::$variant(self.to_owned())
                }
            }
        )*
    };


}