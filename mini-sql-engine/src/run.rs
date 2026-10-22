use crate::core::{Column, Table, Value};
use crate::tokenizer::Token;

pub fn run(tokens: Vec<Token>, table: &Table) -> Result<(&Vec<Column>, &Vec<Vec<Value>>), String> {
    match tokens.as_slice() {
        [Token::Select, Token::Star] => Ok(table.select_all()),
        _ => Err("unsopported query".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::DataType;

    fn users_table() -> Table {
        let mut table = Table::create(
            "users".to_string(),
            vec![
                Column {
                    name: "id".to_string(),
                    data_type: DataType::Int,
                },
                Column {
                    name: "name".to_string(),
                    data_type: DataType::Text,
                },
            ],
        );
        table
            .insert(vec![Value::Int(1), Value::Text("Ana".to_string())])
            .unwrap();
        table
            .insert(vec![Value::Int(2), Value::Text("Pedro".to_string())])
            .unwrap();
        table
    }

    #[test]
    fn run() {
        let table = users_table();
        let (columns, rows) = super::run(vec![Token::Select, Token::Star], &table).unwrap();

        assert_eq!(columns.len(), 2);
        assert_eq!(columns[0].name, "id");
        assert_eq!(columns[1].name, "name");

        assert_eq!(
            rows,
            &vec![
                vec![Value::Int(1), Value::Text("Ana".to_string())],
                vec![Value::Int(2), Value::Text("Pedro".to_string())]
            ]
        );
    }

    #[test]
    fn run_unsupported() {
        let table = users_table();
        let result = super::run(vec![Token::Star], &table);
        assert!(result.is_err());
    }
}
