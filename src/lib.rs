use std::any::type_name;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use zbus::Result;
use zvariant::{ObjectPath, Signature, Structure, StructureBuilder};

// Parse a string to a value of type T.
fn from_str<T>(v: &str) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Display,
{
    T::from_str(v)
        .map_err(|e| zbus::Error::Failure(format!("Invalid {} '{}': {}", type_name::<T>(), v, e)))
}

// Build a dictionary from a list of key-value pairs.
fn build_dict<K, V>(pairs: Vec<&str>) -> Result<HashMap<K, V>>
where
    K: FromStr + Eq + Hash,
    V: FromStr,
    <K as FromStr>::Err: std::fmt::Display,
    <V as FromStr>::Err: std::fmt::Display,
{
    let mut dict = HashMap::new();
    for chunk in pairs.chunks(2) {
        let k = from_str::<K>(chunk[0])?;
        let v = from_str::<V>(&chunk[1])?;
        dict.insert(k, v);
    }
    Ok(dict)
}

// Build a message body, parsing values from the arguments. Arguments encode the
// type of the value into the string itself in the format "type:value". All
// basic types are supported, as well as arrays of basic types.
pub fn build_body(args: Vec<&str>) -> Result<Structure<'static>> {
    let mut builder = StructureBuilder::new();

    for arg in args {
        let (type_name, value) = {
            let splits = arg.splitn(2, ':').collect::<Vec<&str>>();
            (splits[0], splits[1])
        };

        match type_name {
            // Basic types
            "int32" => {
                builder = builder.add_field(from_str::<i32>(value)?);
            }
            "uint32" => {
                builder = builder.add_field(from_str::<u32>(value)?);
            }
            "int64" => {
                builder = builder.add_field(from_str::<i64>(value)?);
            }
            "uint64" => {
                builder = builder.add_field(from_str::<u64>(value)?);
            }
            "int16" => {
                builder = builder.add_field(from_str::<i16>(value)?);
            }
            "uint16" => {
                builder = builder.add_field(from_str::<u16>(value)?);
            }
            "byte" => {
                builder = builder.add_field(from_str::<u8>(value)?);
            }
            "double" => {
                builder = builder.add_field(from_str::<f64>(value)?);
            }
            "boolean" | "bool" => {
                builder = builder.add_field(from_str::<bool>(value)?);
            }
            "signature" => {
                builder = builder.add_field(Signature::try_from(value).map_err(|e| {
                    zbus::Error::Failure(format!("Invalid signature '{}': {}", value, e))
                })?)
            }
            "objpath" => {
                builder =
                    builder.add_field(ObjectPath::try_from(value.to_string()).map_err(|e| {
                        zbus::Error::Failure(format!("Invalid object path '{}': {}", value, e))
                    })?)
            }
            "string" => {
                builder = builder.add_field(value.to_string());
            }
            "array" => {
                let (element_type, values) = {
                    let splits = value.splitn(2, ':').collect::<Vec<&str>>();
                    if splits.len() != 2 {
                        return Err(zbus::Error::Failure(format!(
                            "Invalid array type '{}': expected format: array:<element_type>:<comma_separated_values>",
                            value
                        )));
                    }
                    (splits[0], splits[1].split(',').collect::<Vec<&str>>())
                };

                match element_type {
                    "int32" => {
                        let array: Result<Vec<i32>> =
                            values.iter().map(|v| from_str::<i32>(v.trim())).collect();
                        builder = builder.add_field(array?);
                    }
                    "uint32" => {
                        let array: Result<Vec<u32>> =
                            values.iter().map(|v| from_str::<u32>(v.trim())).collect();
                        builder = builder.add_field(array?);
                    }
                    "int64" => {
                        let array: Result<Vec<i64>> =
                            values.iter().map(|v| from_str::<i64>(v.trim())).collect();
                        builder = builder.add_field(array?);
                    }
                    "uint64" => {
                        let array: Result<Vec<u64>> =
                            values.iter().map(|v| from_str::<u64>(v.trim())).collect();
                        builder = builder.add_field(array?);
                    }
                    "int16" => {
                        let array: Result<Vec<i16>> =
                            values.iter().map(|v| from_str::<i16>(v.trim())).collect();
                        builder = builder.add_field(array?);
                    }
                    "uint16" => {
                        let array: Result<Vec<u16>> =
                            values.iter().map(|v| from_str::<u16>(v.trim())).collect();
                        builder = builder.add_field(array?);
                    }
                    "byte" => {
                        let array: Result<Vec<u8>> =
                            values.iter().map(|v| from_str::<u8>(v.trim())).collect();
                        builder = builder.add_field(array?);
                    }
                    "double" => {
                        let array: Result<Vec<f64>> =
                            values.iter().map(|v| from_str::<f64>(v.trim())).collect();
                        builder = builder.add_field(array?);
                    }
                    "boolean" | "bool" => {
                        let array: Result<Vec<bool>> =
                            values.iter().map(|v| from_str::<bool>(v.trim())).collect();
                        builder = builder.add_field(array?);
                    }
                    "string" => {
                        let array: Vec<String> =
                            values.iter().map(|v| v.trim().to_string()).collect();
                        builder = builder.add_field(array);
                    }
                    "objpath" => {
                        let array: Result<Vec<ObjectPath>> = values
                            .iter()
                            .map(|v| {
                                ObjectPath::try_from(v.trim().to_string()).map_err(|e| {
                                    zbus::Error::Failure(format!(
                                        "Invalid object path array value: {}",
                                        e
                                    ))
                                })
                            })
                            .collect();
                        builder = builder.add_field(array.map_err(|e| {
                            zbus::Error::Failure(format!("Invalid object path array value: {}", e))
                        })?);
                    }
                    "signature" => {
                        let array: Result<Vec<Signature>> = values
                            .iter()
                            .map(|v| from_str::<Signature>(v.trim()))
                            .collect();
                        builder = builder.add_field(array?);
                    }
                    _ => {
                        return Err(zbus::Error::Failure(format!(
                            "Unsupported array element type: {}",
                            element_type
                        )));
                    }
                }
            }
            "dict" => {
                let (key_type, value_type, pairs) = {
                    let splits = value.splitn(3, ':').collect::<Vec<&str>>();
                    if splits.len() != 3 {
                        return Err(zbus::Error::Failure(format!(
                            "Invalid dictionary type '{}': expected format: dict:<key_type>:<value_type>:<comma_separated_pairs>",
                            value
                        )));
                    }
                    (
                        splits[0],
                        splits[1],
                        splits[2].split(',').collect::<Vec<&str>>(),
                    )
                };

                // Length of pairs should be even; an odd number of pairs
                // indicates a malformed dictionary.
                if pairs.len() % 2 != 0 {
                    return Err(zbus::Error::Failure(format!(
                        "Invalid dictionary type '{}': expected even number of pairs",
                        value
                    )));
                }

                // Build the dictionary based on key and value types
                match (key_type, value_type) {
                    ("string", "int32") => {
                        builder = builder.add_field(build_dict::<String, i32>(pairs)?);
                    }
                    ("string", "uint32") => {
                        builder = builder.add_field(build_dict::<String, u32>(pairs)?);
                    }
                    ("string", "int64") => {
                        builder = builder.add_field(build_dict::<String, i64>(pairs)?);
                    }
                    ("string", "uint64") => {
                        builder = builder.add_field(build_dict::<String, u64>(pairs)?);
                    }
                    ("string", "int16") => {
                        builder = builder.add_field(build_dict::<String, i16>(pairs)?);
                    }
                    ("string", "uint16") => {
                        builder = builder.add_field(build_dict::<String, u16>(pairs)?);
                    }
                    ("string", "byte") => {
                        builder = builder.add_field(build_dict::<String, u8>(pairs)?);
                    }
                    ("string", "double") => {
                        builder = builder.add_field(build_dict::<String, f64>(pairs)?);
                    }
                    ("string", "boolean") | ("string", "bool") => {
                        builder = builder.add_field(build_dict::<String, bool>(pairs)?);
                    }
                    ("string", "string") => {
                        builder = builder.add_field(build_dict::<String, String>(pairs)?);
                    }
                    _ => {
                        return Err(zbus::Error::Failure(format!(
                            "Unsupported dictionary key-value type combination: {}:{}",
                            key_type, value_type
                        )));
                    }
                }
            }
            _ => {
                return Err(zbus::Error::Failure(format!(
                    "Unsupported type: {}",
                    type_name
                )));
            }
        };
    }

    Ok(builder.build()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_string_int32() {
        let args = vec!["dict:string:int32:\"one\",1,\"two\",2,\"three\",3"];
        let result = build_body(args);
        assert!(
            result.is_ok(),
            "Dictionary parsing should succeed: {:?}",
            result
        );
    }

    #[test]
    fn test_dictionary_string_string() {
        let args = vec!["dict:string:string:\"name\",\"John\",\"city\",\"NYC\""];
        let result = build_body(args);
        assert!(
            result.is_ok(),
            "String-string dictionary parsing should succeed: {:?}",
            result
        );
    }

    #[test]
    fn test_dictionary_invalid_pairs() {
        let args = vec!["dict:string:int32:\"one\",1,\"two\""];
        let result = build_body(args);
        assert!(
            result.is_err(),
            "Dictionary with odd number of pairs should fail"
        );
    }

    #[test]
    fn test_dictionary_unsupported_types() {
        let args = vec!["dict:float:int32:1.0,1"];
        let result = build_body(args);
        assert!(
            result.is_err(),
            "Dictionary with unsupported key type should fail"
        );
    }
}
