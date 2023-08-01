use crate::AttributeTrait;

pub fn extract_vec<A: AttributeTrait+Clone+PartialEq>(v: &Vec<(String, A)>, field_name: &str) -> Option<A> {
    v.iter().find_map(|(name, field)| {
        if name == field_name {
            Some(field.to_owned())
        } else {
            None
        }
    })
}