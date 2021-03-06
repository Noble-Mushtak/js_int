#[cfg(test)]
mod tests {
    use serde_json::{from_str as from_json_str, to_string as to_json_string};

    use js_int::{int, uint, Int, UInt};

    #[test]
    fn serialize_int() {
        assert_eq!(to_json_string(&int!(100)).unwrap(), "100");
        assert_eq!(to_json_string(&int!(0)).unwrap(), "0");
        assert_eq!(to_json_string(&int!(-100)).unwrap(), "-100");
    }

    #[test]
    fn deserialize_int() {
        assert_eq!(from_json_str::<Int>("100").unwrap(), int!(100));
        assert_eq!(from_json_str::<Int>("0").unwrap(), int!(0));
        assert_eq!(from_json_str::<Int>("-100").unwrap(), int!(-100));
        assert_eq!(from_json_str::<Int>("-9007199254740991").unwrap(), Int::MIN);
        assert_eq!(from_json_str::<Int>("9007199254740991").unwrap(), Int::MAX);
        assert!(from_json_str::<Int>("9007199254740992").is_err());
        assert!(from_json_str::<Int>("-9007199254740992").is_err());
    }

    #[test]
    fn serialize_uint() {
        assert_eq!(to_json_string(&uint!(100)).unwrap(), "100");
        assert_eq!(to_json_string(&uint!(0)).unwrap(), "0");
    }

    #[test]
    fn deserialize_uint() {
        assert_eq!(from_json_str::<UInt>("100").unwrap(), uint!(100));
        assert_eq!(from_json_str::<UInt>("0").unwrap(), uint!(0));
        assert_eq!(from_json_str::<UInt>("9007199254740991").unwrap(), UInt::MAX);
        assert!(from_json_str::<UInt>("9007199254740992").is_err());
    }

    #[test]
    #[cfg_attr(feature = "lax_deserialize", ignore)]
    fn strict_deserialize_int() {
        assert!(from_json_str::<Int>("-10.0").is_err());
        assert!(from_json_str::<Int>("-0.0").is_err());
        assert!(from_json_str::<Int>("0.5").is_err());
        assert!(from_json_str::<Int>("1.0").is_err());
        assert!(from_json_str::<Int>("9007199254740991.0").is_err());
        assert!(from_json_str::<Int>("9007199254740991.49").is_err());
        assert!(from_json_str::<Int>("9007199254740992.0").is_err());
    }

    #[test]
    #[cfg_attr(feature = "lax_deserialize", ignore)]
    fn strict_deserialize_uint() {
        assert!(from_json_str::<UInt>("0.5").is_err());
        assert!(from_json_str::<UInt>("1.0").is_err());
        assert!(from_json_str::<UInt>("9007199254740991.0").is_err());
        assert!(from_json_str::<UInt>("9007199254740991.49").is_err());
        assert!(from_json_str::<UInt>("9007199254740992.0").is_err());
    }

    #[test]
    #[cfg_attr(not(feature = "lax_deserialize"), ignore)]
    fn lax_deserialize_int() {
        assert_eq!(from_json_str::<Int>("-10.0").unwrap(), int!(-10));
        assert_eq!(from_json_str::<Int>("-0.0").unwrap(), int!(0));
        assert_eq!(from_json_str::<Int>("0.5").unwrap(), int!(0));
        assert_eq!(from_json_str::<Int>("1.0").unwrap(), int!(1));
        assert_eq!(from_json_str::<Int>("9007199254740991.0").unwrap(), Int::MAX);
        assert_eq!(from_json_str::<Int>("9007199254740991.49").unwrap(), Int::MAX);
        assert!(from_json_str::<Int>("9007199254740992.0").is_err());
    }

    #[test]
    #[cfg_attr(not(feature = "lax_deserialize"), ignore)]
    fn lax_deserialize_uint() {
        assert_eq!(from_json_str::<UInt>("0.5").unwrap(), uint!(0));
        assert_eq!(from_json_str::<UInt>("1.0").unwrap(), uint!(1));
        assert_eq!(from_json_str::<UInt>("9007199254740991.0").unwrap(), UInt::MAX);
        assert_eq!(from_json_str::<UInt>("9007199254740991.49").unwrap(), UInt::MAX);
        assert!(from_json_str::<UInt>("9007199254740992.0").is_err());
    }
}
