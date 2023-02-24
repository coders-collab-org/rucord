macro_rules! to_value {
    ($map:ident, $key:ident) => {
        serde_json::from_value($map.remove(stringify!($key)).expect(format!("expected `{}` field", stringify!($key)).as_str())).expect("Inavlid field type")
    };
}
