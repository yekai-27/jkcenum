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
    pub is_serde: bool,
    pub lowercase: bool,
    pub uppercase: bool,
    pub rename_all: Option<String>,
    pub desc: bool, // description, eg: "SnakeCase" to "Snake Case"
}


impl Default for ContainerAttributes {
    fn default() -> Self {
        Self {
            crate_name: "::".to_string(),
            is_serde: false,
            lowercase: false,
            uppercase: false,
            rename_all: None,
            desc: false,
        }
    }
}


impl FromAttribute for ContainerAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let mut result = Self::default();

        let attributes = match parse_tagged_attribute(group, "jenum")? {
            Some(body) => body,
            None => {
                match parse_tagged_attribute(group, "serde")? {
                    Some(body) => {
                        result.is_serde = true;
                        body
                    },
                    None => return Ok(None),
                }
            },
        };

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // // #xxx[xxx]
                    match i.to_string().as_str() {
                        "lowercase" => result.lowercase = true,
                        "uppercase" => result.uppercase = true,
                        "desc" => result.uppercase = true,
                        _ => {
                            if !result.is_serde {
                                return Err(Error::custom_at("Unknown field attribute", i.span()))
                            }
                        },
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "rename_all" => {
                            match parse_value_string(&val)?.to_string().as_str() {
                                "lowercase" => result.lowercase = true,
                                "uppercase" | "UPPERCASE" => result.uppercase = true,
                                "desc" => result.desc = true,
                                _ => return Err(Error::custom_at("Unknown field attribute", key.span())),
                            }
                        },
                        _ => {
                            if !result.is_serde {
                                return Err(Error::custom_at("Unknown field attribute", key.span()));
                            }
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
    pub is_serde: bool,
    pub rename: Option<String>,
    pub alias: Vec<String>,
    pub range: Option<String>,
    pub default: bool,
}


impl FromAttribute for FieldAttributes {
    fn parse(group: &Group) -> Result<Option<Self>> {
        let mut result = Self::default();

        let attributes = match parse_tagged_attribute(group, "jenum")? {
            Some(body) => body,
            None => {
                match parse_tagged_attribute(group, "serde")? {
                    Some(body) => {
                        result.is_serde = true;
                        body
                    },
                    None => return Ok(None),
                }
            },
        };

        for attribute in attributes {
            match attribute {
                ParsedAttribute::Tag(i) => {
                    // #xxx[xxx]
                    match i.to_string().as_str() {
                        "default" => result.default = true,
                        _ => {
                            if !result.is_serde {
                                return Err(Error::custom_at("Unknown field attribute", i.span()))
                            }
                        },
                    }
                }
                ParsedAttribute::Property(key, val) => {
                    // #xxx[xxx=xxx]
                    match key.to_string().as_str() {
                        "rename" => result.rename = Some(parse_value_string(&val)?),
                        "alias" => result.alias.push(val.to_string()),
                        "range" => result.range = Some(parse_value_string(&val)?),
                        _ => {
                            if !result.is_serde {
                                return Err(Error::custom_at("Unknown field attribute", key.span()));
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(Some(result))
    }
}
