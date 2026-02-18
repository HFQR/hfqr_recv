use core::fmt;

use serde_core::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(i8)]
pub enum Direction {
    LONG = 1,
    SHORT = -1,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LONG => f.write_str("LONG"),
            Self::SHORT => f.write_str("SHORT"),
        }
    }
}

impl Serialize for Direction {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        const NAME: &str = "Direction";
        match *self {
            Direction::LONG => ser.serialize_unit_variant(NAME, 0, "LONG"),
            Direction::SHORT => ser.serialize_unit_variant(NAME, 1, "SHORT"),
        }
    }
}

impl<'de> Deserialize<'de> for Direction {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const VARIANTS: &[&str] = &["LONG", "SHORT"];

        enum Field {
            F0,
            F1,
        }

        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("variant identifier")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match val {
                    0 => Ok(Field::F0),
                    1 => Ok(Field::F1),
                    _ => Err(de::Error::invalid_value(
                        de::Unexpected::Unsigned(val),
                        &"variant index 0 <= i < 2",
                    )),
                }
            }

            #[inline]
            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_bytes(val.as_bytes())
            }

            fn visit_bytes<E>(self, val: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match val {
                    b"LONG" => Ok(Field::F0),
                    b"SHORT" => Ok(Field::F1),
                    _ => {
                        let str = str::from_utf8(val).unwrap_or("\u{fffd}\u{fffd}\u{fffd}");
                        Err(de::Error::unknown_variant(str, VARIANTS))
                    }
                }
            }
        }

        impl<'de> Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D>(de: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                de.deserialize_identifier(FieldVisitor)
            }
        }

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Direction;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("enum Direction")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: de::EnumAccess<'de>,
            {
                match de::EnumAccess::variant(data)? {
                    (Field::F0, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Direction::LONG)
                    }
                    (Field::F1, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Direction::SHORT)
                    }
                }
            }
        }

        de.deserialize_enum("Direction", VARIANTS, Visitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Offset {
    OPEN = 1,
    CLOSE,
    CLOSETODAY,
    CLOSEYESTERDAY,
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::OPEN => f.write_str("OPEN"),
            Self::CLOSE => f.write_str("CLOSE"),
            Self::CLOSETODAY => f.write_str("CLOSETODAY"),
            Self::CLOSEYESTERDAY => f.write_str("CLOSEYESTERDAY"),
        }
    }
}

impl Serialize for Offset {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        const NAME: &str = "Offset";
        match *self {
            Offset::OPEN => ser.serialize_unit_variant(NAME, 0, "OPEN"),
            Offset::CLOSE => ser.serialize_unit_variant(NAME, 1, "CLOSE"),
            Offset::CLOSETODAY => ser.serialize_unit_variant(NAME, 2, "CLOSETODAY"),
            Offset::CLOSEYESTERDAY => ser.serialize_unit_variant(NAME, 3, "CLOSEYESTERDAY"),
        }
    }
}

impl<'de> Deserialize<'de> for Offset {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const VARIANTS: &[&str] = &["OPEN", "CLOSE", "CLOSETODAY", "CLOSEYESTERDAY"];

        enum Field {
            F0,
            F1,
            F2,
            F3,
        }

        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("variant identifier")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match val {
                    0 => Ok(Field::F0),
                    1 => Ok(Field::F1),
                    2 => Ok(Field::F2),
                    3 => Ok(Field::F3),
                    _ => Err(de::Error::invalid_value(
                        de::Unexpected::Unsigned(val),
                        &"variant index 0 <= i < 4",
                    )),
                }
            }

            #[inline]
            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_bytes(val.as_bytes())
            }

            fn visit_bytes<E>(self, val: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match val {
                    b"OPEN" => Ok(Field::F0),
                    b"CLOSE" => Ok(Field::F1),
                    b"CLOSETODAY" => Ok(Field::F2),
                    b"CLOSEYESTERDAY" => Ok(Field::F3),
                    _ => {
                        let str = str::from_utf8(val).unwrap_or("\u{fffd}\u{fffd}\u{fffd}");
                        Err(de::Error::unknown_variant(str, VARIANTS))
                    }
                }
            }
        }

        impl<'de> Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D>(de: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                de.deserialize_identifier(FieldVisitor)
            }
        }

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Offset;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("enum Offset")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: de::EnumAccess<'de>,
            {
                match de::EnumAccess::variant(data)? {
                    (Field::F0, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Offset::OPEN)
                    }
                    (Field::F1, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Offset::CLOSE)
                    }
                    (Field::F2, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Offset::CLOSETODAY)
                    }
                    (Field::F3, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Offset::CLOSEYESTERDAY)
                    }
                }
            }
        }

        de.deserialize_enum("Offset", VARIANTS, Visitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum Status {
    ERROR = 0,
    INITIAL,
    SUBMITTING,
    NOTTRADED,
    ALLTRADED,
    CANCELLED,
    CANCELFAILED,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::ERROR => f.write_str("ERROR"),
            Self::INITIAL => f.write_str("INITIAL"),
            Self::SUBMITTING => f.write_str("SUBMITTING"),
            Self::NOTTRADED => f.write_str("NOTTRADED"),
            Self::ALLTRADED => f.write_str("ALLTRADED"),
            Self::CANCELLED => f.write_str("CANCELLED"),
            Self::CANCELFAILED => f.write_str("CANCELFAILED"),
        }
    }
}

impl Serialize for Status {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        const NAME: &str = "Status";
        match *self {
            Status::ERROR => ser.serialize_unit_variant(NAME, 0, "ERROR"),
            Status::INITIAL => ser.serialize_unit_variant(NAME, 1, "INITIAL"),
            Status::SUBMITTING => ser.serialize_unit_variant(NAME, 2, "SUBMITTING"),
            Status::NOTTRADED => ser.serialize_unit_variant(NAME, 3, "NOTTRADED"),
            Status::ALLTRADED => ser.serialize_unit_variant(NAME, 4, "ALLTRADED"),
            Status::CANCELLED => ser.serialize_unit_variant(NAME, 5, "CANCELLED"),
            Status::CANCELFAILED => ser.serialize_unit_variant(NAME, 6, "CANCELFAILED"),
        }
    }
}

impl<'de> Deserialize<'de> for Status {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        const VARIANTS: &[&str] = &[
            "ERROR",
            "INITIAL",
            "SUBMITTING",
            "NOTTRADED",
            "ALLTRADED",
            "CANCELLED",
            "CANCELFAILED",
        ];

        enum Field {
            F0,
            F1,
            F2,
            F3,
            F4,
            F5,
            F6,
        }

        struct FieldVisitor;

        impl<'de> de::Visitor<'de> for FieldVisitor {
            type Value = Field;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("variant identifier")
            }

            fn visit_u64<E>(self, val: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match val {
                    0 => Ok(Field::F0),
                    1 => Ok(Field::F1),
                    2 => Ok(Field::F2),
                    3 => Ok(Field::F3),
                    4 => Ok(Field::F4),
                    5 => Ok(Field::F5),
                    6 => Ok(Field::F6),
                    _ => Err(de::Error::invalid_value(
                        de::Unexpected::Unsigned(val),
                        &"variant index 0 <= i < 7",
                    )),
                }
            }

            #[inline]
            fn visit_str<E>(self, val: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_bytes(val.as_bytes())
            }

            fn visit_bytes<E>(self, val: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                match val {
                    b"ERROR" => Ok(Field::F0),
                    b"INITIAL" => Ok(Field::F1),
                    b"SUBMITTING" => Ok(Field::F2),
                    b"NOTTRADED" => Ok(Field::F3),
                    b"ALLTRADED" => Ok(Field::F4),
                    b"CANCELLED" => Ok(Field::F5),
                    b"CANCELFAILED" => Ok(Field::F6),
                    _ => {
                        let str = str::from_utf8(val).unwrap_or("\u{fffd}\u{fffd}\u{fffd}");
                        Err(de::Error::unknown_variant(str, VARIANTS))
                    }
                }
            }
        }

        impl<'de> Deserialize<'de> for Field {
            #[inline]
            fn deserialize<D>(de: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                de.deserialize_identifier(FieldVisitor)
            }
        }

        struct Visitor;

        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Status;

            fn expecting(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.write_str("enum Status")
            }

            fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
            where
                A: de::EnumAccess<'de>,
            {
                match de::EnumAccess::variant(data)? {
                    (Field::F0, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Status::ERROR)
                    }
                    (Field::F1, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Status::INITIAL)
                    }
                    (Field::F2, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Status::SUBMITTING)
                    }
                    (Field::F3, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Status::NOTTRADED)
                    }
                    (Field::F4, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Status::ALLTRADED)
                    }
                    (Field::F5, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Status::CANCELLED)
                    }
                    (Field::F6, v) => {
                        de::VariantAccess::unit_variant(v)?;
                        Ok(Status::CANCELFAILED)
                    }
                }
            }
        }

        de.deserialize_enum("Status", VARIANTS, Visitor)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum OrderType {
    LIMIT = 0,
    MARKET,
    STOP,
    FAK,
    FOK,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::LIMIT => f.write_str("LIMIT"),
            Self::MARKET => f.write_str("MARKET"),
            Self::STOP => f.write_str("STOP"),
            Self::FAK => f.write_str("FAK"),
            Self::FOK => f.write_str("FOK"),
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(u8)]
pub enum Exchange {
    #[default]
    SHFE = 0,
    CFFEX,
    CZCE,
    DCE,
    INE,
    COMEX,
}

impl fmt::Display for Exchange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::SHFE => f.write_str("SHFE"),
            Self::CFFEX => f.write_str("CFFEX"),
            Self::CZCE => f.write_str("CZCE"),
            Self::DCE => f.write_str("DCE"),
            Self::INE => f.write_str("INE"),
            Self::COMEX => f.write_str("COMEX"),
        }
    }
}

impl Serialize for Exchange {
    #[inline]
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self as u8).serialize(ser)
    }
}

impl<'de> Deserialize<'de> for Exchange {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let res = match Deserialize::deserialize(de)? {
            0u8 => Exchange::SHFE,
            1 => Exchange::CFFEX,
            2 => Exchange::CZCE,
            3 => Exchange::DCE,
            4 => Exchange::INE,
            5 => Exchange::COMEX,
            other => {
                return Err(de::Error::custom(format_args!("invalid value: {other}")));
            }
        };

        Ok(res)
    }
}
