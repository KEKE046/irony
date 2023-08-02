
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

pub trait AttributeTrait: Clone {
    type DataTypeT;
    fn dtype(&self) -> Self::DataTypeT;
}


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ConstValueI32<D:Clone> {
    pub value: i32,
    pub dtype: D,
}

impl<D:Clone> AttributeTrait for ConstValueI32<D> {
    type DataTypeT = D;
    fn dtype(&self) -> D {
        self.dtype.to_owned()
    }
}

#[macro_export]
macro_rules! data_type_enum {
    (
        $enum_name:ident = {
            $($variant:ident$(($($inner:ident),*))?),*
            $(,)?
        }
    ) => {
        #[derive(Clone, Debug, PartialEq)]
        pub enum $enum_name {
            $($variant$(($($inner),*))?),*
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
        impl irony::AttributeTrait for $name {
            type DataTypeT = $dtype;
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