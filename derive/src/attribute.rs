use virtue::prelude::*;
use virtue::utils::*;


fn parse_value_string(value: &Literal) -> Result<String> {
    let val_string = value.to_string();

    if val_string.starts_with("\"") && val_string.ends_with("\"") {
        return Ok(val_string[1..val_string.len() - 1].to_string());
    }

    Ok(val_string)
}


#[derive(Debug)]
pub struct ContainerAttributes {
    pub crate_name: String,
    pub lowercase: bool,
    pub uppercase: bool,
}


impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            crate_name: "::".to_string(),
            lowercase: false,
            uppercase: false,
        }
    }
}


impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "jenum")? {
            Some(body) => body,
            None => return Ok(None),
        };

        let mut result = Self::default();

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // // #xxx[xxx]
                    match i.to_string().as_str() {
                        "lowercase" => result.lowercase = true,
                        "uppercase" => result.uppercase = true,
                        _ => return Err(Error::custom_at("Unknown field attribute", i.span())),
                    }
                }
                ParsedAttribute::Property(key, _val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        _ => {
                            return Err(Error::custom_at("Unknown field attribute", key.span()));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}


#[derive(Debug, Default)]
pub struct FieldAttributes {
    pub rename: Option<String>,
    pub alias: Vec<String>,
    pub range: Option<String>,
    pub default: bool,
}


impl FromAttribute for FieldAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let attributes = match parse_tagged_attribute(group, "jenum")? {
            Some(body) => body,
            None => return Ok(None),
        };

        let mut result = Self::default();

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // #xxx[xxx]
                    match i.to_string().as_str() {
                        "default" => result.default = true,
                        _ => return Err(Error::custom_at("Unknown field attribute", i.span())),
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "rename" => result.rename = Some(parse_value_string(&val)?),
                        "alias" => result.alias.push(val.to_string()),
                        "range" => result.range = Some(parse_value_string(&val)?),
                        _ => {
                            return Err(Error::custom_at("Unknown field attribute", key.span()));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}
