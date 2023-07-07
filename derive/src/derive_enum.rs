#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::prelude::*;
use crate::attribute::{ContainerAttributes, FieldAttributes};


#[allow(unused)]
pub(crate) struct DeriveEnum {
    pub variants: Vec<EnumVariant>,
    pub attributes: ContainerAttributes,
}


impl DeriveEnum {
    fn iter_fields(&self) -> EnumVariantIterator {
        EnumVariantIterator {
            idx: 0,
            variants: &self.variants,
            curruent_idx: 0,
        }
    }

    pub fn generate_jkcenum(&self, generator: &mut Generator) -> Result<()> {
        self.generate_enum_to_string(generator)?;
        self.generate_enum_from_str(generator)?;
        self.generate_enum_to_vec(generator)?;

        Ok(())
    }

    pub fn generate_enum_to_vec(&self, generator: &mut Generator) -> Result<()> {
        let enum_name = generator.target_name();

        generator
            .r#impl()
            .generate_fn("to_vec")
            // .with_self_arg(FnSelfArg::RefSelf)
            // .with_return_type("String")
            .with_return_type(format!("Vec<{enum_name}>"))
            .make_pub()
            .body(|fn_builder| {
                fn_builder.push_parsed("vec!")?;

                fn_builder.group(Delimiter::Bracket, |variant_case| {
                    for (mut _variant_index, variant) in self.iter_fields() {
                        variant_case.push_parsed(format!("{}::{},", &enum_name, &variant.name))?;
                    }
    
                    Ok(())
                })?;

                Ok(())
            })?;

        Ok(())
    }

    pub fn generate_enum_to_string(&self, generator: &mut Generator) -> Result<()> {
        generator
            .impl_for("ToString")
            .generate_fn("to_string")
            .with_self_arg(FnSelfArg::RefSelf)
            .with_return_type("String")
            .body(|fn_builder| {
                fn_builder.push_parsed("match self")?;

                fn_builder.group(Delimiter::Brace, |variant_case| {
                    for (mut _variant_index, variant) in self.iter_fields() {
                        variant_case.push_parsed(format!("Self::{}", &variant.name))?;
                        variant_case.puncts("=>");

                        let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                        if let Some(name) = attributes.rename {
                            variant_case.push_parsed(format!("\"{}\".to_string(),", &name))?;
                        }
                        else {
                            variant_case.push_parsed(format!("\"{}\".to_string(),", &variant.name))?;
                        }
                    }

                    Ok(())
                })?;

                Ok(())
            })?;

        Ok(())
    }

    pub fn generate_enum_from_str(&self, generator: &mut Generator) -> Result<()> {
        let mut generator = generator.impl_for("std::str::FromStr");
            generator.impl_type("Err", "jkcenum::errors::FromStrParseError")?;
            generator.generate_fn("from_str")
            .with_arg("s", "&str")
            .with_return_type("Result<Self, Self::Err>")
            .body(|fn_builder| {
                fn_builder.push_parsed("match s")?;

                fn_builder.group(Delimiter::Brace, |variant_case| {
                    for (mut _variant_index, variant) in self.iter_fields() {
                        let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                        if let Some(name) = attributes.rename {
                            variant_case.push_parsed(format!("\"{name}\""))?;
                        }
                        else {
                            variant_case.push_parsed(format!("\"{}\"", &variant.name))?;
                        }

                        if !attributes.alias.is_empty() {
                            variant_case.push_parsed(format!(" | {}", attributes.alias.join(" | ")))?;
                        }

                        variant_case.puncts("=>");

                        variant_case.push_parsed(format!("Ok(Self::{}),", &variant.name))?;
                    }

                    variant_case.push_parsed("_ => Err(Self::Err::InvalidStr(s.to_string()))")?;

                    Ok(())
                })?;

                Ok(())
            })?;

        Ok(())
    }
}


struct EnumVariantIterator<'a> {
    variants: &'a [EnumVariant],
    idx: usize,
    curruent_idx: isize,
}


impl<'a> Iterator for EnumVariantIterator<'a> {
    type Item = (TokenTree, &'a EnumVariant);

    fn next(&mut self) -> Option<Self::Item> {
        // let mut idx = self.idx;
        let variant = self.variants.get(self.idx)?;

        if let Some(value) = &variant.value {
            // Literal
            let val_string = value.to_string();

            if val_string.starts_with("0x") {
                self.curruent_idx = isize::from_str_radix(&val_string[2..], 16).unwrap();
            }
            else {
                self.curruent_idx = val_string.parse::<isize>().unwrap();
            }
        }

        let tokens = TokenTree::Literal(Literal::isize_suffixed(self.curruent_idx));

        self.curruent_idx += 1;
        self.idx += 1;

        Some((tokens, variant))
    }
}
