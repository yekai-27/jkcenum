#![feature(let_chains)]
extern crate proc_macro;

mod attribute;
mod derive_enum;
mod iter;

use proc_macro::TokenStream;
use attribute::ContainerAttributes;
use virtue::prelude::*;


#[proc_macro_derive(JkcEnum, attributes(jenum))]
pub fn derive_jkcenum(input: TokenStream) -> TokenStream {
    derive_jkcenum_inner(input).unwrap_or_else(|e|e.into_token_stream())
}


fn derive_jkcenum_inner(input: TokenStream) -> Result<TokenStream> {
    let parse = Parse::new(input)?;
    let (mut generator, attributes, body) = parse.into_generator();
    let attributes = attributes
        .get_attribute::<ContainerAttributes>()?
        .unwrap_or_default();

    match body {
        Body::Struct(_body) => {
            return Err(Error::custom("not support struct."));
        }
        Body::Enum(body) => {
            derive_enum::DeriveEnum {
                variants: body.variants,
                attributes,
            }
            .generate_jkcenum(&mut generator)?;
        }
    }

    generator.export_to_file("jenum", "JkcEnum");
    generator.finish()
}
