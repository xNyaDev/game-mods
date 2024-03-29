use std::error::Error;

use toml_edit::DocumentMut;

pub trait DocumentedStruct {
    fn get_doc_string(location: &str, field: &str) -> Option<&'static str>;
}

pub fn serialize_with_comments<T: serde::Serialize + DocumentedStruct>(data: &T) -> Result<String, Box<dyn Error>> {
    let mut document = toml::to_string_pretty(data)?.parse::<DocumentMut>()?;

    comment_table::<T>(document.as_table_mut(), "", T::get_doc_string);

    Ok(document.to_string())
}

fn comment_table<T: DocumentedStruct>(table: &mut toml_edit::Table, location: &str, get_doc_string: fn(&str, &str) -> Option<&'static str>) {
    for (mut key, value) in table.iter_mut() {
        let key_name = key.get().to_owned();

        let doc_string = get_doc_string(location, &key_name).unwrap_or_default();

        match value.as_table_mut() {
            None => {
                let decor = key.leaf_decor_mut();
                decor.set_prefix(doc_string_to_comment("\n", doc_string));
            }
            Some(table) => {
                let decor = table.decor_mut();
                decor.set_prefix(doc_string_to_comment("\n\n", doc_string));
                comment_table::<T>(table, &key_name, get_doc_string);
            }
        }
    }
}

fn doc_string_to_comment(prefix: &str, doc_string: &str) -> String {
    let mut comment = String::from(prefix);
    for line in doc_string.lines() {
        let line = if line.is_empty() {
            String::from("#\n")
        } else {
            format!("# {line}\n")
        };
        comment.push_str(&line);
    }
    comment
}

#[cfg(test)]
mod tests {
    use documented::DocumentedFields;
    use serde::Serialize;

    use super::*;

    impl DocumentedStruct for TestStruct1 {
        fn get_doc_string(location: &str, field: &str) -> Option<&'static str> {
            match location {
                "" => TestStruct1::get_field_docs(field).ok(),
                "table" => TestStruct2::get_field_docs(field).ok(),
                _ => None,
            }
        }
    }

    #[derive(Default, DocumentedFields, Serialize)]
    pub struct TestStruct1 {
        /// Some comment
        pub value: bool,
        /// Some comment on a table
        pub table: TestStruct2,
    }

    #[derive(Default, DocumentedFields, Serialize)]
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

        let expected = r#"# Some comment
value = false


# Some comment on a table
[table]

# Some other comment
#
# Multiline too
value = false

# More values
value2 = false"#;

        assert_eq!(actual.trim(), expected);
    }
}