//! Irony's macros and attributes
 
#![allow(dead_code)]
#![allow(unused_macros)]
#![recursion_limit = "256"]

extern crate proc_macro;
extern crate proc_macro2;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

macro_rules! parse_quote {
    ($($inp:tt)*) => {
        syn::parse2(quote!{$($inp)*}).unwrap_or_else(|err| {
            panic!("failed to parse at {}:{}:{}: {}", file!(), line!(), column!(), err)
        })
    }
}

macro_rules! parse_quote_spanned {
    ($($inp:tt)*) => {
        syn::parse2(quote_spanned!{$($inp)*}).unwrap_or_else(|err| {
            panic!("failed to parse at {}:{}:{}: {}", file!(), line!(), column!(), err)
        })
    }
}

mod utils;
mod options;
mod entity;
mod op;


#[proc_macro_attribute]
pub fn entity(args: TokenStream, input: TokenStream) -> TokenStream {
    entity::entity(args, input)
}

#[proc_macro_attribute]
pub fn entity_enum(args: TokenStream, input: TokenStream) -> TokenStream {
    entity::entity_enum(args, input)
}

#[proc_macro_attribute]
pub fn op(args: TokenStream, input: TokenStream) -> TokenStream {
    op::op(args, input)
}