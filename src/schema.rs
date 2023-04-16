use serde_derive::{Deserialize, Serialize};

/// A table schema. Top level of the table structure.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct TableSchema {
    pub table_name: String,
    pub columns: Vec<ColumnSchema>,
}

/// Describes a column in a table.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ColumnSchema {
    pub name: String,
    pub data_type: DataType,
    pub length: Option<u32>,
    pub constraints: ColumnConstraints,
}

/// The data type of a column.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DataType {
    Integer,
    Varchar,
}

/// The constraints which apply to a column.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ColumnConstraints {
    #[serde(default)]
    pub primary_key: bool,
    #[serde(default)]
    pub auto_increment: bool,
    #[serde(default)]
    pub not_null: bool,
    #[serde(default)]
    pub unique: bool,
    pub foreign_key: Option<ForeignKey>,
}

/// A foreign key constraint, belongs to a column and names another table/column.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ForeignKey {
    pub reference_table: String,
    pub reference_column: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_table_schema() {
        let table_schema: TableSchema = serde_json::from_str(SAMPLE).unwrap();

        assert_eq!(table_schema.table_name, "employees");
        assert_eq!(table_schema.columns.len(), 5);

        assert_eq!(
            table_schema.columns[0],
            ColumnSchema {
                name: "employee_id".to_string(),
                data_type: DataType::Integer,
                length: None,
                constraints: ColumnConstraints {
                    primary_key: true,
                    auto_increment: true,
                    not_null: false,
                    unique: false,
                    foreign_key: None,
                }
            }
        );

        assert_eq!(
            table_schema.columns[1],
            ColumnSchema {
                name: "first_name".to_string(),
                data_type: DataType::Varchar,
                length: Some(50),
                constraints: ColumnConstraints {
                    primary_key: false,
                    auto_increment: false,
                    not_null: true,
                    unique: false,
                    foreign_key: None,
                }
            }
        );

        assert_eq!(
            table_schema.columns[2],
            ColumnSchema {
                name: "last_name".to_string(),
                data_type: DataType::Varchar,
                length: Some(50),
                constraints: ColumnConstraints {
                    primary_key: false,
                    auto_increment: false,
                    not_null: true,
                    unique: false,
                    foreign_key: None,
                }
            }
        );

        assert_eq!(
            table_schema.columns[3],
            ColumnSchema {
                name: "email".to_string(),
                data_type: DataType::Varchar,
                length: Some(100),
                constraints: ColumnConstraints {
                    primary_key: false,
                    auto_increment: false,
                    not_null: true,
                    unique: true,
                    foreign_key: None,
                }
            }
        );

        assert_eq!(
            table_schema.columns[4],
            ColumnSchema {
                name: "department_id".to_string(),
                data_type: DataType::Integer,
                length: None,
                constraints: ColumnConstraints {
                    primary_key: false,
                    auto_increment: false,
                    not_null: false,
                    unique: false,
                    foreign_key: Some(ForeignKey {
                        reference_table: "departments".to_string(),
                        reference_column: "id".to_string(),
                    }),
                }
            }
        );
    }

    const SAMPLE: &str = r#"
        {
            "table_name": "employees",
            "columns": [
                {
                    "name": "employee_id",
                    "data_type": "integer",
                    "constraints": {
                        "primary_key": true,
                        "auto_increment": true
                    }
                },
                {
                    "name": "first_name",
                    "data_type": "varchar",
                    "length": 50,
                    "constraints": {
                        "not_null": true
                    }
                },
                {
                    "name": "last_name",
                    "data_type": "varchar",
                    "length": 50,
                    "constraints": {
                        "not_null": true
                    }
                },
                {
                    "name": "email",
                    "data_type": "varchar",
                    "length": 100,
                    "constraints": {
                        "not_null": true,
                        "unique": true
                    }
                },
                {
                    "name": "department_id",
                    "data_type": "integer",
                    "constraints": {
                        "foreign_key": {
                            "reference_table": "departments",
                            "reference_column": "id"
                        }
                    }
                }
            ]
        }
    "#;
}
