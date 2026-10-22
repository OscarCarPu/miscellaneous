#[derive(PartialEq, Clone, Debug)]
enum DataType {
    Int,
    Float,
    Text,
    Bool,
}
enum Value {
    Null,
    Int(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}

#[derive(Clone, PartialEq, Debug)]
struct Column {
    name: String,
    data_type: DataType,
}

struct Table {
    name: String,
    columns: Vec<Column>,
    rows: Vec<Vec<Value>>,
}

impl Table {
    fn new(name: String, columns: Vec<Column>) -> Table {
        Table {
            name,
            columns,
            rows: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_table() {
        let columns = vec![
            Column {
                name: "id".to_string(),
                data_type: DataType::Int,
            },
            Column {
                name: "name".to_string(),
                data_type: DataType::Text,
            },
        ];

        let table = Table::new("users".to_string(), columns.clone());

        assert_eq!(table.name, "users");
        assert_eq!(table.columns, columns);
    }
}
