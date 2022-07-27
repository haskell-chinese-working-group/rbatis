use crate::type_info::{PgType, PgTypeKind};
use crate::types::Oid;
use crate::value::{PgValue, PgValueFormat, PgValueRef};
use byteorder::{BigEndian, ByteOrder};
use rbdc::Error;
use rbdc::date::Date;
use rbdc::time::ParseTime;
use rbs::Value;
use std::str::FromStr;
use std::time::Duration;
use rbdc::datetime::DateTime;
use rbdc::types::time::Time;
use crate::types::byte::Bytea;

pub trait Decode: Sized {
    /// Decode a new value of this type using a raw value from the database.
    fn decode(value: PgValue) -> Result<Self, Error>;
}

impl Decode for Value {
    fn decode(arg: PgValue) -> Result<Self, Error> {
        Ok(match arg.type_info.0 {
            PgType::Bool => Value::Bool(Decode::decode(arg)?),
            PgType::Bytea => Value::Ext("Bytea",Box::new(Value::U32(Bytea::decode(arg)?.0 as u32))),
            PgType::Char => Value::String(Decode::decode(arg)?),
            PgType::Name => Value::String(Decode::decode(arg)?),
            PgType::Int8 => Value::I32(Decode::decode(arg)?),
            PgType::Int2 => Value::I32(Decode::decode(arg)?),
            PgType::Int4 => Value::I32(Decode::decode(arg)?),
            PgType::Text => Value::String(Decode::decode(arg)?),
            PgType::Oid => Value::Ext("Oid", Box::new(Value::U32(Decode::decode(arg)?))),
            PgType::Json => Value::Ext("Json", Box::new(Value::String(
                crate::types::json::Json::decode(arg)
                    .unwrap_or_default()
                    .json,
            ))),
            PgType::Point => {
                Value::Ext("Point", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Lseg => {
                Value::Ext("Lseg", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Path => {
                Value::Ext("Path", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Box => {
                Value::Ext("Box", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Polygon => {
                Value::Ext("Polygon", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Line => {
                Value::Ext("Line", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Cidr => {
                Value::Ext("Cidr", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }

            PgType::Float4 => Value::F32(Decode::decode(arg)?),
            PgType::Float8 => Value::F32(Decode::decode(arg)?),
            PgType::Unknown => Value::Null,
            PgType::Circle => {
                Value::Ext("Circle", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Macaddr8 => {
                Value::Ext("Macaddr8", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Macaddr => {
                Value::Ext("Macaddr", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Inet => {
                Value::Ext("Inet", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Bpchar => {
                Value::Ext("Bpchar", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Varchar => Value::String(Decode::decode(arg)?),
            PgType::Date => {
                let date: Date = Decode::decode(arg)?;
                Value::Ext("Date", Box::new(Value::String(date.to_string())))
            }
            PgType::Time => {
                let time: Time = Decode::decode(arg)?;
                Value::Ext("Date", Box::new(Value::String(time.to_string())))
            }
            PgType::Timestamp => {
                let fast_date: DateTime = Decode::decode(arg)?;
                Value::Ext("DateTime", Box::new(Value::String(fast_date.0.to_string())))
            },
            PgType::Timestamptz => {
                let fast_date: DateTime = Decode::decode(arg)?;
                Value::Ext("Timestamptz", Box::new(Value::String(fast_date.0.to_string())))
            },
            PgType::Interval => {
                Value::Ext("Interval", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Timetz => {
                todo!()
            }
            PgType::Bit => {
                todo!()
            }
            PgType::Varbit => {
                todo!()
            }
            PgType::Numeric => {
                todo!()
            }
            PgType::Record => {
                Value::Ext("Record", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Uuid => {
                todo!()
            }
            PgType::Jsonb => {
                todo!()
            }
            PgType::Int4Range => {
                Value::Ext("Int4Range", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::NumRange => {
                Value::Ext("NumRange", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::TsRange => {
                Value::Ext("TsRange", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::TstzRange => {
                Value::Ext("TstzRange", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::DateRange => {
                Value::Ext("DateRange", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Int8Range => {
                Value::Ext("Int8Range", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Jsonpath => {
                Value::Ext("Jsonpath", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::Money => {
                todo!()
            }
            PgType::Void => {
                todo!()
            }
            PgType::Custom(_) => {
                Value::Ext("Custom", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::DeclareWithName(_) => {
                Value::Ext("DeclareWithName", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::DeclareWithOid(_) => {
                Value::Ext("DeclareWithOid", Box::new(Value::Binary({
                    match arg.format() {
                        PgValueFormat::Binary => arg.as_bytes().unwrap_or_default().to_owned(),
                        PgValueFormat::Text => {
                            arg.as_str().unwrap_or_default().as_bytes().to_vec()
                        }
                    }
                })))
            }
            PgType::JsonArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::LineArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::CidrArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::CircleArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Macaddr8Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::BoolArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::ByteaArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::CharArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::NameArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int2Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int4Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TextArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::BpcharArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::VarcharArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int8Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::PointArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::LsegArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::PathArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::BoxArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Float4Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Float8Array => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::PolygonArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::OidArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::MacaddrArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::InetArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TimestampArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::DateArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TimeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TimestamptzArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::IntervalArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::NumericArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TimetzArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::BitArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::VarbitArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::RecordArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::UuidArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::JsonbArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int4RangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::NumRangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TsRangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::TstzRangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::DateRangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::Int8RangeArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::JsonpathArray => {
                Value::Array(Decode::decode(arg)?)
            }
            PgType::MoneyArray => {
                Value::Array(Decode::decode(arg)?)
            }
        })
    }
}

impl From<PgValue> for Value {
    fn from(arg: PgValue) -> Self {
        Decode::decode(arg).unwrap()
    }
}




