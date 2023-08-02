

use crate::options::AllowedOptions;
use crate::utils::IronyStruct;
use crate::utils::IronyStructKind;


pub fn op(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match IronyStruct::new(IronyStructKind::Op, args, input)
        .and_then(|base| OpStruct(base).generate())
    {
        Ok(s) => s.into(),
        Err(err) => err.into_compile_error().into(),
    }
}  

struct OpStruct(IronyStruct<Self>);

impl std::ops::Deref for OpStruct {
    type Target = IronyStruct<Self>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AllowedOptions for OpStruct {
    const ENTITY: bool = false;

    const OP: bool = false;

    const DTYPE: bool = false;

    const HAS_DATA: bool = false;

    const ATTRIBUTE: bool = true;

    const CONSTRAINT: bool = true;

}

impl OpStruct {
    fn generate(&self) -> syn::Result<proc_macro::TokenStream> {
        unimplemented!()

    }
}
