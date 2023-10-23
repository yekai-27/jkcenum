#[allow(unused_imports)]
use virtue::{generate::Generator, parse::IdentOrIndex};
use virtue::prelude::*;
use crate::attribute::{ContainerAttributes, FieldAttributes};


/// `SnakeCase` to `Snake Case`
/// 
/// # Args:
/// 
/// - `value`: &str
/// 
/// # Returns:
/// 
/// - `String`
/// 
pub fn snakecase_to_desc(value: &str) -> String {
    let mut first = 0;
    let mut vlist = vec![];

    for (i, v) in value.chars().enumerate() {
        if v >= 'A' && v <= 'Z' {
            if i > 0 {
                vlist.push(&value[first..i]);
            }

            first = i;
        }
    }

    vlist.push(&value[first..]);

    vlist.join(" ")
}


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

    fn get_field_name(&self, attributes: &FieldAttributes, name: &Ident) -> String {
        let mut field_name = name.to_string();

        if let Some(name) = &attributes.rename {
            field_name = name.to_string();
        }
        else if self.attributes.lowercase {
            field_name = field_name.to_lowercase();
        }
        else if self.attributes.uppercase {
            field_name = field_name.to_uppercase();
        }
        else if self.attributes.desc {
            field_name = snakecase_to_desc(&field_name);
        }

        field_name
    }

    pub fn generate_jkcenum(&self, generator: &mut Generator) -> Result<()> {
        self.generate_enum_to_string(generator)?;
        self.generate_enum_from_str(generator)?;
        self.generate_enum_to_vec(generator)?;
        self.generate_enum_from_int(generator)?;

        Ok(())
    }

    pub fn generate_enum_to_vec(&self, generator: &mut Generator) -> Result<()> {
        let enum_name = generator.target_name();

        generator
            .r#impl()
            .generate_fn("to_vec")
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

                        variant_case.push_parsed(format!("\"{}\".to_string(),", &self.get_field_name(&attributes, &variant.name)))?;
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

                        variant_case.push_parsed(format!("\"{}\"", &self.get_field_name(&attributes, &variant.name)))?;

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

    pub fn generate_enum_from_int(&self, generator: &mut Generator) -> Result<()> {
        let mut generator = generator.impl_for("jkcenum::FromInt");
            generator.impl_type("Err", "jkcenum::errors::FromIntParseError")?;
            generator.generate_fn("from_int")
            .with_arg("v", "isize")
            .with_return_type("Result<Self, Self::Err>")
            .body(|fn_builder| {
                fn_builder.push_parsed("match v")?;

                fn_builder.group(Delimiter::Brace, |variant_case| {
                    let mut default = false;

                    for (variant_index, variant) in self.iter_fields() {
                        let attributes = variant.attributes.get_attribute::<FieldAttributes>()?.unwrap_or_default();

                        if attributes.default {
                            variant_case.push_parsed("_")?;
                            default = true;
                        }
                        else if let Some(range) = attributes.range {
                            variant_case.push_parsed(format!("{range}"))?;
                        }
                        else {
                            variant_case.push_parsed(format!("{}", variant_index.to_string().replace("isize", "")))?;
                        }

                        variant_case.puncts("=>");

                        variant_case.push_parsed(format!("Ok(Self::{}),", &variant.name))?;
                    }

                    if !default {
                        variant_case.push_parsed("_ => Err(Self::Err::InvalidInt(v))")?;
                    }

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
