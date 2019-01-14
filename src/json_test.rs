use std::f64;
use std::fs::File;

use crate::json::{Json, Jsons};
use crate::num::{Floating, Integral};
use crate::property::Property;

#[test]
fn test_json_constructor() {
    use self::Json;

    assert_eq!(Json::new(10), Json::Integer(Integral::new("10")));
}

#[test]
fn test_simple_jsons() {
    use self::Json::{Array, Bool, Null, Object, String};

    let jsons = include!("../testdata/test_simple.jsons");
    let refs = include!("../testdata/test_simple.jsons.ref");

    for (i, json) in jsons.iter().enumerate() {
        let mut value: Json = json.parse().unwrap();
        value.compute().unwrap();
        assert_eq!(value, refs[i], "testcase {}", i);
    }
}

#[test]
fn test_simple_jsons_ref() {
    use self::Json::{Array, Bool, Null, Object, String};

    let jsons = include!("../testdata/test_simple.jsons");
    let refs = include!("../testdata/test_simple.jsons.ref");

    let value: Json = jsons[51].parse().unwrap();
    assert_eq!(value, refs[51]);

    let ref_jsons = include!("../testdata/test_simple.jsons.ref.jsons");
    for (i, r) in refs.iter().enumerate() {
        let s = format!("{}", r);
        //println!("{} {}", i, &s);
        assert_eq!(&s, ref_jsons[i], "testcase: {}", i);
    }
}

#[test]
fn test_convert() {
    let js: Json = true.into();
    assert_eq!(js, Json::new(true));

    let js: Json = 1024.into();
    assert_eq!(js, Json::new(1024));

    let js: Json = 1024.2.into();
    assert_eq!(js, Json::new(1024.2));

    let js: Json = "hello world".to_string().into();
    assert_eq!(js, Json::new("hello world"));

    let js: Json = "hello world".into();
    assert_eq!(js, Json::new("hello world"));
}

#[test]
fn test_deferred() {
    let inp = r#" [10123.1231, 1231.123123, 1233.123123, 123.1231231, 12312e10]"#;
    let value: Json = inp.parse().unwrap();
    let refval = Json::Array(vec![
        Json::Float(Floating::new("10123.1231")),
        Json::Float(Floating::new("1231.123123")),
        Json::Float(Floating::new("1233.123123")),
        Json::Float(Floating::new("123.1231231")),
        Json::Float(Floating::new("12312e10")),
    ]);
    assert_eq!(value, refval);
}

#[test]
fn test_validate_sorted() {
    let json = r#"{"z":1,"a":[2, {"x":"y"}, true],"c":[null],"d":3}"#;
    let mut value: Json = json.parse().unwrap();

    assert_eq!(value.validate(), Ok(()));

    let mut props: Vec<Property> = Vec::new();
    let prop = vec![Property::new("x", Json::new("y"))];
    let items = vec![Json::new(2), Json::new(prop), Json::new(true)];
    props.push(Property::new("a", Json::new(items)));
    props.push(Property::new("c", Json::new(vec![Json::Null])));
    props.push(Property::new("d", Json::new(3)));
    props.push(Property::new("z", Json::new(1)));

    assert_eq!(value, Json::new(props));
}

#[test]
fn test_compute() {
    let json = r#"{"z":1,"a":[2, {"x":"y"}, true],"c":[null],"d":3}"#;
    let mut value: Json = json.parse().unwrap();

    assert_eq!(value.compute(), Ok(()));

    let mut props: Vec<Property> = Vec::new();
    let prop = vec![Property::new("x", Json::new("y"))];
    let items = vec![Json::new(2), Json::new(prop), Json::new(true)];
    props.push(Property::new("a", Json::new(items)));
    props.push(Property::new("c", Json::new(vec![Json::Null])));
    props.push(Property::new("d", Json::new(3)));
    props.push(Property::new("z", Json::new(1)));

    assert_eq!(value, Json::new(props));
}

#[test]
fn test_json5_whitespace() {
    let text = "\u{0009} \u{000a} \u{000b} \u{000c} ".to_string()
        + &("\u{00a0} \r \t \n 0x1234".to_string());
    let json: Json = text.parse().unwrap();
    assert_eq!(json.integer(), Json::new(0x1234).integer());
}

#[test]
fn test_json5_num() {
    let mut json: Json = "0x1234".parse().unwrap();
    json.compute().unwrap();
    assert_eq!(json, Json::new(0x1234));

    let mut json: Json = "1234.".parse().unwrap();
    json.compute().unwrap();
    assert_eq!(json.float(), Json::new(1234.0).float());

    let mut json: Json = ".1234".parse().unwrap();
    json.compute().unwrap();
    assert_eq!(json, Json::new(0.1234));

    let mut json: Json = ".1234.".parse().unwrap();
    json.compute().unwrap_err();
    assert_eq!(json.float(), None);

    let mut json: Json = "[Infinity, -Infinity, NaN]".parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![
        Json::new(f64::INFINITY),
        Json::new(f64::NEG_INFINITY),
        Json::new(f64::NAN),
    ]);
    assert_eq!(json, value);

    let mut json: Json = " [ 0xdecaf, -0xC0FFEE ]".parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![Json::new(0xdecaf), Json::new(-0xC0_FFEE)]);
    assert_eq!(json, value);

    let mut json: Json = "[ 123, 123.456, .456, 123e-456 ]".parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![
        Json::new(123),
        Json::new(123.456),
        Json::new(0.456),
        Json::new(123e-456),
    ]);
    assert_eq!(json, value);
}

#[test]
fn test_json5_array() {
    let json: Json = "[]".parse().unwrap();
    let value = Json::new::<Vec<Json>>(vec![]);
    assert_eq!(json, value);

    let mut json: Json = r#"[ 1, true, "three", ]"#.parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![Json::new(1), Json::new(true), Json::new("three")]);
    assert_eq!(json, value);

    let json: Json = r#"[ [1, true, "three"], [4, "five", 0x6], ]"#.parse().unwrap();
    let value = Json::new(vec![
        Json::new(vec![Json::new(1), Json::new(true), Json::new("three")]),
        Json::new(vec![Json::new(4), Json::new("five"), Json::new(0x6)]),
    ]);
    assert_eq!(json, value);
}

#[test]
fn test_json5_object() {
    let json: Json = "{}".parse().unwrap();
    let value = Json::new::<Vec<Property>>(vec![]);
    assert_eq!(json, value);

    let mut json: Json = "{ width: 1920, height: 1080, }".parse().unwrap();
    json.compute().unwrap();
    let value = Json::new(vec![
        Property::new("height", 1080.into()),
        Property::new("width", 1920.into()),
    ]);
    assert_eq!(json, value);

    let mut json: Json = r#"{ image: { width: 1920, height: 1080, "aspect-ratio": "16:9", } }"#
        .parse()
        .unwrap();
    json.compute().unwrap();
    let props = Json::new(vec![
        Property::new("aspect-ratio", "16:9".into()),
        Property::new("height", 1080.into()),
        Property::new("width", 1920.into()),
    ]);
    let value = Json::new(vec![Property::new("image", props)]);
    assert_eq!(json, value);

    let mut json: Json = r#"[ { name: "Joe", age: 27 }, { name: "Jane", age: 32 }, ]"#
        .parse()
        .unwrap();
    json.compute().unwrap();
    let obj1 = Json::new::<Vec<Property>>(vec![
        Property::new("age", 27.into()),
        Property::new("name", "Joe".into()),
    ]);
    let obj2 = Json::new::<Vec<Property>>(vec![
        Property::new("age", 32.into()),
        Property::new("name", "Jane".into()),
    ]);
    let value = Json::new(vec![obj1, obj2]);
    assert_eq!(json, value);
}

#[test]
fn test_stream0() {
    let mut js: Jsons<&[u8]> = b"".as_ref().into();
    assert!(js.next().is_none());

    let mut js: Jsons<&[u8]> = b" \t \r \n ".as_ref().into();
    assert!(js.next().is_none());

    let mut js: Jsons<&[u8]> = b" 1".as_ref().into();
    assert_eq!(js.next().unwrap().unwrap(), Json::new(1));

    let mut js: Jsons<&[u8]> = b" n".as_ref().into();
    let value = js.next().unwrap().unwrap();
    assert!(value.is_error());
    assert_eq!(
        value.error().unwrap(),
        "parse: expected null at offset:0 line:1 col:1".to_string()
    );
}

#[test]
fn test_stream1() {
    let file = File::open("testdata/stream1.jsons").unwrap();
    let mut js: Jsons<File> = file.into();

    assert_eq!(js.next().unwrap().unwrap(), Json::new(1));

    assert_eq!(js.next().unwrap().unwrap(), Json::Null);
    assert_eq!(js.next().unwrap().unwrap(), Json::new(true));
    assert_eq!(js.next().unwrap().unwrap(), Json::new(false));

    assert_eq!(js.next().unwrap().unwrap().integer(), Some(102));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(10.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));

    assert_eq!(js.next().unwrap().unwrap().integer(), Some(0));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(100));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(1));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));

    assert_eq!(js.next().unwrap().unwrap().float(), Some(2.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.02));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(20.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(20.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(200.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));
}

#[test]
fn test_stream11() {
    let file = File::open("testdata/stream11.jsons").unwrap();
    let mut js: Jsons<File> = file.into();

    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(2.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(0.2));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-102));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-10.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.2));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-0));

    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-100));
    assert_eq!(js.next().unwrap().unwrap().integer(), Some(-1));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-00.00));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-2.00));

    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.2));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.02));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-0.0));
    assert_eq!(js.next().unwrap().unwrap().float(), Some(-20.0));
}

#[test]
fn test_stream2() {
    let file = File::open("testdata/stream2.jsons").unwrap();
    let mut js: Jsons<File> = file.into();

    assert_eq!(
        js.next().unwrap().unwrap().string(),
        Some("hello\"  \r\t".to_string())
    );

    assert_eq!(
        js.next().unwrap().unwrap().string(),
        Some("helloȴ\\ 𝄞".to_string())
    );

    assert_eq!(
        js.next().unwrap().unwrap().string(),
        Some("\'é\' character is one Unicode code point é while \'é\' e\u{301} ".to_string())
    );

    assert_eq!(js.next().unwrap().unwrap(), Json::new::<Vec<Json>>(vec![]));
    assert_eq!(js.next().unwrap().unwrap(), Json::new(vec![Json::new(10)]));
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Json::Null,
            true.into(),
            false.into(),
            10.into(),
            "tru\"e".into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        "汉语 / 漢語; Hàn\u{8} \tyǔ ".into()
    );
}

#[test]
fn test_stream3() {
    let file = File::open("testdata/stream3.jsons").unwrap();
    let mut js: Jsons<File> = file.into();

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Json::Null,
            true.into(),
            false.into(),
            "hello\" \\ / \u{8} \u{c}\n\r\t".into()
        ])
    );
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            102.into(),
            10.2.into(),
            0.2.into(),
            0.into(),
            "helloȴ\\ 𝄞".into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            100.into(),
            1.into(),
            0.0.into(),
            2.0.into(),
            "汉语 / 漢語; Hàn\u{8} \tyǔ ".into()
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            0.2.into(),
            0.02.into(),
            0.0.into(),
            0.2.into(),
            0.2.into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Json>>(vec![
            (-102).into(),
            (-100).into(),
            (-0.0).into(),
            (-20.0).into(),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new::<Vec<Property>>(vec![])
    );
    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![Property::new("key1", "value1".into())])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Property::new("key1", "value1".into()),
            Property::new("key2", "value2".into()),
        ])
    );

    assert_eq!(
        js.next().unwrap().unwrap(),
        Json::new(vec![
            Property::new("z", 1.into()),
            Property::new("a", 1.into()),
            Property::new("c", 1.into()),
            Property::new("d", 1.into()),
            Property::new("f", 1.into()),
            Property::new("e", 1.into()),
            Property::new("b", 1.into()),
            Property::new("x", 1.into()),
        ])
    );

    let obj = Json::new(vec![Property::new("key3", 20.into())]);
    let obj = Json::new(vec![Property::new("key2", obj)]);
    let arr = Json::new::<Vec<Json>>(vec!["world".into(), obj]);
    let obj = Json::new(vec![Property::new("key1", arr)]);
    let arr = Json::new::<Vec<Json>>(vec!["hello".into(), obj]);
    assert_eq!(js.next().unwrap().unwrap(), arr);
}

#[test]
fn test_partial_eq() {
    let a = Json::new(f64::INFINITY);
    let b = Json::new(f64::NEG_INFINITY);
    let c = Json::new(f64::NAN);
    let d = Json::new(0.2);

    assert!(a != b);
    assert!(a != c);
    assert!(a != d);
    assert!(b != a);
    assert!(b != c);
    assert!(b != d);
    assert!(c != a);
    assert!(c != b);
    assert!(c != d);
    assert!(d != a);
    assert!(d != b);
    assert!(d != c);

    assert!(Json::minbound() == Json::minbound());
    assert!(Json::maxbound() == Json::maxbound());
    assert!(Json::minbound() != Json::maxbound());
    assert!(Json::maxbound() != Json::minbound());
}

#[test]
fn test_partial_ord1() {
    assert!(Json::Null < Json::new(true));
    assert!(Json::Null < Json::new(false));
    assert!(Json::Null < Json::new(10));
    assert!(Json::Null < Json::new(1.0));
    assert!(Json::Null < Json::new("hello world"));
    assert!(Json::Null < Json::new::<Vec<Json>>(vec![10.into()]));
    assert!(Json::Null < Json::new::<Vec<Property>>(vec![Property::new("key", 10.into())]));

    let value = Json::new(false);
    assert!(value > Json::Null);
    assert!(value == Json::new(false));
    assert!(value < Json::new(true));
    assert!(value < Json::new(10));
    assert!(value < Json::new(1.0));
    assert!(value < Json::new("hello world"));
    assert!(value < Json::new::<Vec<Json>>(vec![10.into()]));
    assert!(value < Json::new::<Vec<Property>>(vec![Property::new("key", 10.into())]));

    let value = Json::new(true);
    assert!(value > Json::Null);
    assert!(value > Json::new(false));
    assert!(value == Json::new(true));
    assert!(value < Json::new(10));
    assert!(value < Json::new(1.0));
    assert!(value < Json::new("hello world"));
    assert!(value < Json::new::<Vec<Json>>(vec![10.into()]));
    assert!(value < Json::new::<Vec<Property>>(vec![Property::new("key", 10.into())]));
}

#[test]
fn test_partial_ord2() {
    let value = Json::new(10);
    assert!(value > Json::Null);
    assert!(value > Json::new(false));
    assert!(value > Json::new(true));
    assert!(value == Json::new(10));
    assert!(value == Json::new(10.0));
    assert!(value < Json::new("hello world"));
    assert!(value < Json::new::<Vec<Json>>(vec![10.into()]));
    assert!(value < Json::new::<Vec<Property>>(vec![Property::new("key", 10.into())]));

    let value = Json::new(10.0);
    assert!(value > Json::Null);
    assert!(value > Json::new(false));
    assert!(value > Json::new(true));
    assert!(value == Json::new(10));
    assert!(value == Json::new(10.0));
    assert!(value < Json::new("hello world"));
    assert!(value < Json::new::<Vec<Json>>(vec![10.into()]));
    assert!(value < Json::new::<Vec<Property>>(vec![Property::new("key", 10.into())]));

    let value = Json::new("hello world");
    assert!(value > Json::Null);
    assert!(value > Json::new(false));
    assert!(value > Json::new(true));
    assert!(value > Json::new(10));
    assert!(value > Json::new(10.0));
    assert!(value == Json::new("hello world"));
    assert!(value < Json::new::<Vec<Json>>(vec![10.into()]));
    assert!(value < Json::new::<Vec<Property>>(vec![Property::new("key", 10.into())]));
}

#[test]
fn test_partial_ord3() {
    let value: Json = "[10,20]".parse().unwrap();
    assert!(value > Json::Null);
    assert!(value > Json::new(false));
    assert!(value > Json::new(true));
    assert!(value > Json::new(10));
    assert!(value > Json::new(10.0));
    assert!(value > Json::new("hello world"));
    assert!(value == Json::new::<Vec<Json>>(vec![10.into(), 20.into()]));
    assert!(value > Json::new::<Vec<Json>>(vec![10.into()]));
    assert!(Json::new::<Vec<Json>>(vec![10.into()]) < value);
    assert!(value < Json::new::<Vec<Property>>(vec![Property::new("key", 10.into())]));

    let value: Json = r#"{"key1": 10, "key2":20}"#.parse().unwrap();
    assert!(value > Json::Null);
    assert!(value > Json::new(false));
    assert!(value > Json::new(true));
    assert!(value > Json::new(10));
    assert!(value > Json::new(10.0));
    assert!(value > Json::new("hello world"));
    assert!(value > Json::new::<Vec<Json>>(vec![10.into()]));
    assert!(
        value
            > Json::new::<Vec<Property>>(vec![
                Property::new("key1", 10.into()),
                Property::new("key2", 10.into())
            ])
    );
    assert!(
        value
            < Json::new::<Vec<Property>>(vec![
                Property::new("key1", 20.into()),
                Property::new("key2", 10.into())
            ])
    );
    assert!(
        value
            > Json::new::<Vec<Property>>(vec![
                Property::new("key1", 5.into()),
                Property::new("key2", 10.into())
            ])
    );
    assert!(value > Json::new::<Vec<Property>>(vec![Property::new("key1", 10.into())]));
    assert!(Json::new::<Vec<Property>>(vec![Property::new("key1", 10.into())]) < value);
}

#[test]
fn test_partial_ord4() {
    let lhs: Json = "[]".parse().unwrap();
    let rhs: Json = "[10]".parse().unwrap();
    assert!(lhs < rhs);
    assert!(rhs > lhs);

    let lhs: Json = r#"{}"#.parse().unwrap();
    let rhs: Json = r#"{"a": 10}"#.parse().unwrap();
    assert!(lhs < rhs);
    assert!(rhs > lhs);

    let lhs: Json = r#"-1.0"#.parse().unwrap();
    let rhs: Json = r#"1.0"#.parse().unwrap();
    assert!(lhs < rhs);
    assert!(rhs > lhs);
    assert!(rhs != lhs);

    let lhs: Json = r#"-0.0"#.parse().unwrap();
    let rhs: Json = r#"0.0"#.parse().unwrap();
    assert!(lhs < rhs);
    assert!(rhs > lhs);
    assert!(lhs == rhs);
}

#[test]
fn test_bounds() {
    assert!(Json::minbound() == Json::minbound());
    assert!(Json::minbound() < Json::Null);
    assert!(Json::minbound() < true.into());
    assert!(Json::minbound() < false.into());
    assert!(Json::minbound() < 10.into());
    assert!(Json::minbound() < 10.2.into());
    assert!(Json::minbound() < "hello world".into());
    assert!(Json::minbound() < "[null]".parse().unwrap());
    assert!(Json::minbound() < r#"{"key":10}"#.parse().unwrap());
    assert!(Json::minbound() < Json::maxbound());

    assert!(Json::maxbound() > Json::minbound());
    assert!(Json::maxbound() > Json::Null);
    assert!(Json::maxbound() > true.into());
    assert!(Json::maxbound() > false.into());
    assert!(Json::maxbound() > 10.into());
    assert!(Json::maxbound() > 10.2.into());
    assert!(Json::maxbound() > "hello world".into());
    assert!(Json::maxbound() > "[null]".parse().unwrap());
    assert!(Json::maxbound() > r#"{"key":10}"#.parse().unwrap());
    assert!(Json::maxbound() == Json::maxbound());
}

#[test]
fn test_boolean_coersion() {
    assert!(!bool::from(Json::Null));
    assert!(!bool::from(Json::new(false)));
    assert!(bool::from(Json::new(true)));
    assert!(!bool::from(Json::new(0)));
    assert!(!bool::from(Json::new(0.0)));
    assert!(bool::from(Json::new(0.1)));
    assert!(bool::from(Json::new(-0.1)));
    assert!(bool::from(Json::new(-1)));
    assert!(!bool::from(Json::new("")));
    assert!(bool::from(Json::new("hello")));
    let value: Vec<Json> = vec![];
    assert!(!bool::from(Json::new(value)));
    let value: Vec<Json> = vec![1.into()];
    assert!(bool::from(Json::new(value)));
    let value: Vec<Json> = vec![];
    assert!(!bool::from(Json::new(value)));
    let value: Vec<Property> = vec![Property::new("a", 10.into())];
    assert!(bool::from(Json::new(value)));
}
