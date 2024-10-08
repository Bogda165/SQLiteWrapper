use std::error::Error;
use macros_l::*;
use rusqlite::{Connection, ToSql};
use rusqlite::types::ToSqlOutput;
use rusqlite::types::Value::Null;
use crate::Attributes::{AUTO_I, PK};
use crate::DbTypes::*;

trait INSERTABLE {
    fn for_insert(&self, num: i32) -> String;
}

#[derive(Clone, Debug)]
pub enum OptionalNULL<T> {
    NULL,
    VALUE(T)
}

impl<T> ToSql for OptionalNULL<T>
    where T: ToSql
{
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            OptionalNULL::NULL => {Null.to_sql()}
            OptionalNULL::VALUE(val) => {
                val.to_sql()
            }
        }
    }
}

#[fields_name]
#[derive(Debug)]
pub enum DbTypes {
    INTEGER_N(OptionalNULL<i32>),
    FLOAT_N(OptionalNULL<f32>),
    TEXT_N(OptionalNULL<String>),
    INTEGER(i32),
    FLOAT(f32),
    TEXT(String),
}

impl DbTypes {
    pub fn to_string(&self) -> String {
        match &self {
            INTEGER(_) => {
                "INTEGER".to_string()
            }
            FLOAT(_) => {
                "REAL".to_string()
            }
            TEXT(_) => {
                "TEXT".to_string()
            }
            INTEGER_N(_) => {
                "INTEGER".to_string()
            }
            FLOAT_N(_) => {
                "REAL".to_string()
            }
            TEXT_N(_) => {
                "TEXT".to_string()
            }

            _ => {
                unreachable!("I hae not any others types")
            }
        }
    }
}

impl INSERTABLE for DbTypes {
    fn for_insert(&self, num: i32) -> String {
        match self {
            INTEGER(val) => {
                val.to_string()
            }
            FLOAT(val) => {
                val.to_string()
            }
            TEXT(str) => {
                str.clone()
            }
            INTEGER_N(val) => {
                match val {
                    OptionalNULL::NULL => {"NULL".to_string()}
                    OptionalNULL::VALUE(val) => {val.to_string()}
                }
            }
            FLOAT_N(val) => {
                match val {
                    OptionalNULL::NULL => {"NULL".to_string()}
                    OptionalNULL::VALUE(val) => {val.to_string()}
                }
            }
            TEXT_N(val) => {
                match val {
                    OptionalNULL::NULL => {"NULL".to_string()}
                    OptionalNULL::VALUE(val) => {val.clone()}
                }
            }
        }
    }
}


impl ToSql for DbTypes {
    fn to_sql(&self) -> Result<ToSqlOutput<'_>, rusqlite::Error> {
        match self {
            DbTypes::INTEGER(i) => {Ok(ToSqlOutput::from(*i))},
            DbTypes::FLOAT(f) => Ok(ToSqlOutput::from(*f)),
            DbTypes::TEXT(s) => Ok(ToSqlOutput::from(s.clone())),
            INTEGER_N(i) => {i.to_sql()},
            FLOAT_N(f) => {f.to_sql()},
            TEXT_N(t) => {t.to_sql()},
        }
    }
}

#[derive(Debug)]
#[fields_name]
pub enum Attributes {
    PK,
    AUTO_I,
    CONNECT(&'static str)
}

impl Attributes {
    pub fn to_string(&self) -> String {
        match &self {
            PK => {
                "PRIMARY KEY".to_string()
            }
            AUTO_I => {
                "AUTOINCREMENT".to_string()
            }
            Attributes::CONNECT(_) => {
                "CONNECT IDK)".to_string()
            }
        }
    }
}


pub trait TableTrait {

}
//create additional array for many to many connection, add it in the end of the query, if needed, as different query

// a repore represent a list of table types, which must realise method get table(shadow table)
/*
    -CREATE TABLE person (
    ident.to_sring() for field in fields field.to_string()
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  age             INTEGER
                )",
 */
/*

    INSERT INTO person (name, age) VALUES (?1, ?2)
 */

pub trait Entity
{
    fn get_table_name() -> String;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_table() {
        let user_instance = user {
            id: 1,
            name: "Alice".to_string(),
        };

        let table = user_instance.get_table();

        match table.id {
            (DbTypes::INTEGER(val), Attributes::PK, Attributes::AUTO_I) => assert_eq!(val, 1),
            _ => panic!("ID field did not match expected values"),
        }

        match table.name {
            DbTypes::TEXT(val) => assert_eq!(val, "Alice".to_string()),
            _ => panic!("Name field did not match expected values"),
        }
    }
}