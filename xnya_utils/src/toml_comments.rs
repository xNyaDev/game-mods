use std::error::Error;

use toml_edit::DocumentMut;

pub trait TypedFields {
    fn get_field_type(field: &str) -> Option<&'static str>;
}

pub trait ReflectionFields {
    fn get_field_value(&self, field: &str) -> Option<String>;
}

pub trait DocumentedStruct {
    fn get_doc_string(location: &str, field: &str) -> Option<&'static str>;
    fn get_type_string(location: &str, field: &str) -> Option<&'static str>;
    fn get_default_string(location: &str, field: &str) -> Option<String>;
}

pub fn serialize_with_comments<T: serde::Serialize + DocumentedStruct>(
    data: &T,
) -> Result<String, Box<dyn Error>> {
    let mut document = toml::to_string_pretty(data)?.parse::<DocumentMut>()?;

    comment_table::<T>(document.as_table_mut(), "");

    Ok(document.to_string())
}

fn comment_table<T: DocumentedStruct>(table: &mut toml_edit::Table, location: &str) {
    for (mut key, value) in table.iter_mut() {
        let key_name = key.get().to_owned();

        let doc_string = T::get_doc_string(location, &key_name).unwrap_or_default();
        let type_string = T::get_type_string(location, &key_name)
            .unwrap_or_default()
            .replace(" ", "");
        let default_string = T::get_default_string(location, &key_name).unwrap_or_default();

        match value.as_table_mut() {
            None => {
                let decor = key.leaf_decor_mut();
                decor.set_prefix(
                    doc_string_to_comment("\n", doc_string)
                        + &format!(
                            "# Type: {}\n# Default value: {}\n",
                            type_string, default_string
                        ),
                );
            }
            Some(table) => {
                let decor = table.decor_mut();
                decor.set_prefix(doc_string_to_comment("\n\n", doc_string));
                comment_table::<T>(table, &key_name);
            }
        }
    }
}

fn doc_string_to_comment(prefix: &str, doc_string: &str) -> String {
    let mut comment = String::from(prefix);
    for line in doc_string.lines() {
        let line = if line.is_empty() {
            String::from("##\n")
        } else {
            format!("## {line}\n")
        };
        comment.push_str(&line);
    }
    comment
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ReflectionFields, TypedFields};
    use documented::DocumentedFields;
    use serde::Serialize;

    impl DocumentedStruct for TestStruct1 {
        fn get_doc_string(location: &str, field: &str) -> Option<&'static str> {
            match location {
                "" => TestStruct1::get_field_docs(field).ok(),
                "table" => TestStruct2::get_field_docs(field).ok(),
                _ => None,
            }
        }

        fn get_type_string(location: &str, field: &str) -> Option<&'static str> {
            match location {
                "" => TestStruct1::get_field_type(field),
                "table" => TestStruct2::get_field_type(field),
                _ => None,
            }
        }
        fn get_default_string(location: &str, field: &str) -> Option<String> {
            match location {
                "" => TestStruct1::default().get_field_value(field),
                "table" => TestStruct2::default().get_field_value(field),
                _ => None,
            }
        }
    }

    #[derive(Debug, Default, DocumentedFields, Serialize, TypedFields, ReflectionFields)]
    pub struct TestStruct1 {
        /// Some comment
        pub value: bool,
        /// Some comment on a table
        pub table: TestStruct2,
    }

    #[derive(Debug, Default, DocumentedFields, Serialize, TypedFields, ReflectionFields)]
    pub struct TestStruct2 {
        /// Some other comment
        ///
        /// Multiline too
        pub value: bool,
        /// More values
        pub value2: bool,
    }

    #[test]
    fn serializing_test() {
        let test_struct = TestStruct1::default();

        let actual = serialize_with_comments(&test_struct).unwrap();

        let expected = r#"## Some comment
# Type: bool
# Default value: false
value = false


## Some comment on a table
[table]

## Some other comment
##
## Multiline too
# Type: bool
# Default value: false
value = false

## More values
# Type: bool
# Default value: false
value2 = false"#;

        assert_eq!(actual.trim(), expected);
    }
}
