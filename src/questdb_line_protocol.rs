use primitive_types::U256;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::ProtocolError;

#[derive(Copy, Clone, Debug)]
pub struct DataPoint<'a, S, C>
where
    S: IntoIterator<Item = (&'a str, &'a str)>,
    C: IntoIterator<Item = (&'a str, ColumnValue<'a>)>,
{
    pub table: &'a str,
    pub symbol_set: S,
    pub column_set: C,
    pub timestamp: Option<u64>,
}

impl<'a, S, C> TryFrom<DataPoint<'a, S, C>> for String
where
    S: IntoIterator<Item = (&'a str, &'a str)>,
    C: IntoIterator<Item = (&'a str, ColumnValue<'a>)>,
{
    type Error = ProtocolError;

    fn try_from(item: DataPoint<'a, S, C>) -> Result<Self, Self::Error> {
        // Write table name
        check_valid(item.table)?;
        let mut result = escape(item.table);

        // Write symbol set
        for (name, value) in item.symbol_set {
            check_valid(name)?;
            result += &format!(",{}={}", escape(name), escape(value));
        }

        // Write column set
        for (i, (name, value)) in item.column_set.into_iter().enumerate() {
            if i == 0 {
                result += " ";
            } else {
                result += ",";
            }
            check_valid(name)?;
            result += &format!("{}={value}", escape(name));
        }

        // Write timestamp
        if let Some(ts) = item.timestamp {
            result += &format!(" {ts}");
        }

        // Terminate line
        result += "\n";

        Ok(result)
    }
}

const FORBIDDEN_CHARACTERS: &str = ".?,:\\,/\0)(+*~%-";

fn check_valid(s: &str) -> Result<(), ProtocolError> {
    for c in s.chars() {
        if FORBIDDEN_CHARACTERS.contains(c) {
            return Err(ProtocolError::ForbiddenCharacter(c));
        }
    }
    Ok(())
}

const ESCAPE_CHARACTERS: &str = " \\,";

fn escape(s: &str) -> String {
    let mut result = "".to_string();
    result.reserve(s.len());
    for c in s.chars() {
        if ESCAPE_CHARACTERS.contains(c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

#[derive(Clone, Debug)]
pub enum ColumnValue<'a> {
    Integer(i64),
    Long256(U256),
    Float(f64),
    Boolean(bool),
    String(&'a str),
    Timestamp(u64),
}

impl<'a> fmt::Display for ColumnValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Integer(value) => write!(f, "{value}i"),
            Self::Long256(value) => write!(f, "{value:#x}i"),
            Self::Float(value) => write!(f, "{value}"),
            Self::Boolean(true) => write!(f, "true"),
            Self::Boolean(false) => write!(f, "false"),
            Self::String(value) => write!(f, "\"{}\"", Self::escape(value)),
            Self::Timestamp(value) => write!(f, "{value}t"),
        }
    }
}

impl<'a> ColumnValue<'a> {
    fn escape(s: &str) -> String {
        s.replace("\"", "\\\"")
    }
}

pub fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn column_to_string() {
        assert_eq!(ColumnValue::Integer(-1234).to_string(), "-1234i");
        assert_eq!(ColumnValue::Long256(U256::from(1234)).to_string(), "0x4d2i");
        assert_eq!(ColumnValue::Float(1.234).to_string(), "1.234");
        assert_eq!(ColumnValue::Boolean(true).to_string(), "true");
        assert_eq!(ColumnValue::Boolean(false).to_string(), "false");
        assert_eq!(
            ColumnValue::String("Hello World!").to_string(),
            "\"Hello World!\""
        );
        assert_eq!(
            ColumnValue::Timestamp(1234567890).to_string(),
            "1234567890t"
        );
    }

    #[test]
    fn data_point_to_string() {
        let s: String = DataPoint {
            table: r"My measurements",
            symbol_set: vec![(r"symbol name", "Symbol Value")],
            column_set: vec![
                (r"Integer column", ColumnValue::Integer(1234)),
                (r"Long256 column", ColumnValue::Long256(U256::from(1234))),
                (r"Float column", ColumnValue::Float(1.234)),
                (r"Boolean column", ColumnValue::Boolean(true)),
                (r"String column", ColumnValue::String("Hello \"World!\"")),
                (r"Timestamp column", ColumnValue::Timestamp(1234567890)),
            ],
            timestamp: Some(1234567890),
        }
        .try_into()
        .unwrap();
        assert_eq!(
            s,
            r#"My\ measurements,symbol\ name=Symbol\ Value Integer\ column=1234i,"#.to_owned()
                + r#"Long256\ column=0x4d2i,Float\ column=1.234,Boolean\ column=true,"#
                + r#"String\ column="Hello \"World!\"",Timestamp\ column=1234567890t 1234567890"#
        );
    }
}
