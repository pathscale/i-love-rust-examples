use thiserror::Error;
use uuid::Uuid;

use super::Value;

pub trait ParsableValue {
    fn try_i8(&self) -> Result<i8, ParseValueError>;
    fn possible_i8(&self) -> Result<Option<i8>, ParseValueError>;
    fn try_i16(&self) -> Result<i16, ParseValueError>;
    fn possible_i16(&self) -> Result<Option<i16>, ParseValueError>;
    fn try_i32(&self) -> Result<i32, ParseValueError>;
    fn possible_i32(&self) -> Result<Option<i32>, ParseValueError>;
    fn try_i64(&self) -> Result<i64, ParseValueError>;
    fn possible_i64(&self) -> Result<Option<i64>, ParseValueError>;
    fn try_i128(&self) -> Result<i128, ParseValueError>;
    fn possible_i128(&self) -> Result<Option<i128>, ParseValueError>;
    fn try_f64(&self) -> Result<f64, ParseValueError>;
    fn possible_f64(&self) -> Result<Option<f64>, ParseValueError>;
    fn try_bool(&self) -> Result<bool, ParseValueError>;
    fn possible_bool(&self) -> Result<Option<bool>, ParseValueError>;
    fn try_string(&self) -> Result<String, ParseValueError>;
    fn possible_string(&self) -> Result<Option<String>, ParseValueError>;
    fn try_bytea(&self) -> Result<Vec<u8>, ParseValueError>;
    fn possible_bytea(&self) -> Result<Option<Vec<u8>>, ParseValueError>;
    fn try_uuid(&self) -> Result<Uuid, ParseValueError>;
    fn possible_uuid(&self) -> Result<Option<Uuid>, ParseValueError>;
}

impl ParsableValue for Value {
    fn try_i8(&self) -> Result<i8, ParseValueError> {
        match self {
            Value::I8(i) => Ok(*i),
            Value::Null => Err(ParseValueError::I8NullError),
            _ => Err(ParseValueError::I8NotActualValueError),
        }
    }
    fn possible_i8(&self) -> Result<Option<i8>, ParseValueError> {
        match self {
            Value::I8(i) => Ok(Some(*i)),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::I8NotActualValueError),
        }
    }
    fn try_i16(&self) -> Result<i16, ParseValueError> {
        match self {
            Value::I16(i) => Ok(*i),
            Value::Null => Err(ParseValueError::I16NullError),
            _ => Err(ParseValueError::I16NotActualValueError),
        }
    }
    fn possible_i16(&self) -> Result<Option<i16>, ParseValueError> {
        match self {
            Value::I16(i) => Ok(Some(*i)),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::I16NotActualValueError),
        }
    }
    fn try_i32(&self) -> Result<i32, ParseValueError> {
        match self {
            Value::I32(i) => Ok(*i),
            Value::Null => Err(ParseValueError::I32NullError),
            _ => Err(ParseValueError::I32NotActualValueError),
        }
    }
    fn possible_i32(&self) -> Result<Option<i32>, ParseValueError> {
        match self {
            Value::I32(i) => Ok(Some(*i)),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::I32NotActualValueError),
        }
    }
    fn try_i64(&self) -> Result<i64, ParseValueError> {
        match self {
            Value::I64(i) => Ok(*i),
            Value::Null => Err(ParseValueError::I64NullError),
            _ => Err(ParseValueError::I64NotActualValueError),
        }
    }
    fn possible_i64(&self) -> Result<Option<i64>, ParseValueError> {
        match self {
            Value::I64(i) => Ok(Some(*i)),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::I64NotActualValueError),
        }
    }
    fn try_i128(&self) -> Result<i128, ParseValueError> {
        match self {
            Value::I128(i) => Ok(*i),
            Value::Null => Err(ParseValueError::I128NullError),
            _ => Err(ParseValueError::I128NotActualValueError),
        }
    }
    fn possible_i128(&self) -> Result<Option<i128>, ParseValueError> {
        match self {
            Value::I128(i) => Ok(Some(*i)),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::I128NotActualValueError),
        }
    }
    fn try_f64(&self) -> Result<f64, ParseValueError> {
        match self {
            Value::F64(f) => Ok(*f),
            Value::Null => Err(ParseValueError::F64NullError),
            _ => Err(ParseValueError::F64NotActualValueError),
        }
    }
    fn possible_f64(&self) -> Result<Option<f64>, ParseValueError> {
        match self {
            Value::F64(f) => Ok(Some(*f)),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::F64NotActualValueError),
        }
    }
    fn try_bool(&self) -> Result<bool, ParseValueError> {
        match self {
            Value::Bool(b) => Ok(*b),
            Value::Null => Err(ParseValueError::BoolNullError),
            _ => Err(ParseValueError::BoolNotActualValueError),
        }
    }
    fn possible_bool(&self) -> Result<Option<bool>, ParseValueError> {
        match self {
            Value::Bool(b) => Ok(Some(*b)),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::BoolNotActualValueError),
        }
    }
    fn try_string(&self) -> Result<String, ParseValueError> {
        match self {
            Value::Str(s) => Ok(s.to_owned()),
            Value::Null => Err(ParseValueError::StringNullError),
            _ => Err(ParseValueError::StringNotActualValueError),
        }
    }
    fn possible_string(&self) -> Result<Option<String>, ParseValueError> {
        match self {
            Value::Str(s) => Ok(Some(s.to_owned())),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::StringNotActualValueError),
        }
    }
    fn try_bytea(&self) -> Result<Vec<u8>, ParseValueError> {
        match self {
            Value::Bytea(b) => Ok(b.to_vec()),
            Value::Null => Err(ParseValueError::ByteaNullError),
            _ => Err(ParseValueError::ByteaNotActualValueError),
        }
    }
    fn possible_bytea(&self) -> Result<Option<Vec<u8>>, ParseValueError> {
        match self {
            Value::Bytea(b) => Ok(Some(b.to_vec())),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::ByteaNotActualValueError),
        }
    }
    fn try_uuid(&self) -> Result<Uuid, ParseValueError> {
        match self {
            Value::Uuid(u) => Ok(Uuid::from_u128(*u)),
            Value::Null => Err(ParseValueError::UuidNullError),
            _ => Err(ParseValueError::UuidNotActualValueError),
        }
    }
    fn possible_uuid(&self) -> Result<Option<Uuid>, ParseValueError> {
        match self {
            Value::Uuid(u) => Ok(Some(Uuid::from_u128(*u))),
            Value::Null => Ok(None),
            _ => Err(ParseValueError::UuidNotActualValueError),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParseValueError {
    #[error("i8 not actual value")]
    I8NotActualValueError,
    #[error("i8 is null")]
    I8NullError,
    #[error("i16 not actual value")]
    I16NotActualValueError,
    #[error("i16 is null")]
    I16NullError,
    #[error("i32 not actual value")]
    I32NotActualValueError,
    #[error("i32 is null")]
    I32NullError,
    #[error("i64 not actual value")]
    I64NotActualValueError,
    #[error("i64 is null")]
    I64NullError,
    #[error("i128 not actual value")]
    I128NotActualValueError,
    #[error("i128 is null")]
    I128NullError,
    #[error("f64 not actual value")]
    F64NotActualValueError,
    #[error("f64 is null")]
    F64NullError,
    #[error("bool not actual value")]
    BoolNotActualValueError,
    #[error("bool is null")]
    BoolNullError,
    #[error("string not actual value")]
    StringNotActualValueError,
    #[error("string is null")]
    StringNullError,
    #[error("bytea not actual value")]
    ByteaNotActualValueError,
    #[error("bytea is null")]
    ByteaNullError,
    #[error("uuid not actual value")]
    UuidNotActualValueError,
    #[error("uuid is null")]
    UuidNullError,
}
