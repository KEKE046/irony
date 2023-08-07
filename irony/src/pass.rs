use crate::{OpId, Environ};

pub trait PassTrait<T, ERR> {

    fn get_statistics_mut(&self) -> &mut PassStatistics;

    fn get_arguments_str() -> String;
    
    fn get_name_str() -> String;

    fn get_description_str() -> String;

    fn check_op<E: Environ>(&self, env:&E, op: OpId) -> bool;
    fn check_op_static<E:Environ>(env: &E, op: OpId) -> bool;

    fn run_on(&self, op: OpId) -> Result<T, ERR>;
    
}

pub struct PassManager;

pub struct PassPipeline;

pub struct PassStatistics;
