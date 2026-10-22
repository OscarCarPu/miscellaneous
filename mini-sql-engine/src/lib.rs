// src/lib.rs
mod core;
mod run;
mod tokenizer;

pub use crate::core::{Column, DataType, Table, Value};
pub use crate::run::run;
pub use crate::tokenizer::Token;

pub fn execute(sql: &str, table: &Table) -> Result<String, String> {
    let tokens = Token::tokenize(sql)?;
    let (columns, rows) = run(tokens, table)?;
    Ok(format!("{:?}, {:?}", columns, rows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn select_all_query() {
        let mut table = Table::create(
            "users".to_string(),
            vec![Column {
                name: "id".to_string(),
                data_type: DataType::Int,
            }],
        );
        table.insert(vec![Value::Int(1)]).unwrap();

        let output = execute("select *", &table).unwrap();

        assert_eq!(
            output,
            "[Column { name: \"id\", data_type: Int }], [[Int(1)]]"
        );
    }
}
