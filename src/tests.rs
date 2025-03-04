use crate::{HashMap, JsonValue, json};

#[test]
fn test_null() {
	assert_eq!("null".parse::<JsonValue>(), Ok(JsonValue::Null));
}

#[test]
fn test_true() {
	assert_eq!("true".parse::<JsonValue>(), Ok(true.into()));
}

#[test]
fn test_false() {
	assert_eq!("false".parse::<JsonValue>(), Ok(false.into()));
}

#[test]
fn test_error() {
	assert!("   hello  ".parse::<JsonValue>().is_err());
}

#[test]
fn test_one() {
	assert_eq!(
		"1.0".parse::<JsonValue>(),
		Ok(JsonValue::Number(1_f64.try_into().unwrap()))
	);
}

#[test]
fn test_decimal() {
	assert_eq!(
		"-234.43".parse::<JsonValue>(),
		Ok(JsonValue::try_from(-234.43_f64).unwrap())
	);
}

#[test]
fn test_complicated() {
	assert_eq!(
		"-0.00933e+5".parse::<JsonValue>(),
		Ok(JsonValue::try_from(-933.).unwrap())
	);
}

#[test]
fn test_complicated2() {
	assert_eq!(
		"18.4e-2".parse::<JsonValue>(),
		Ok(0.184_f64.try_into().unwrap())
	);
}

#[test]
fn test_inf() {
	assert_eq!(
		"21e99999999999999999999999999999999999".parse::<JsonValue>(),
		Ok(JsonValue::Null)
	);
}

#[test]
fn test_empty_string() {
	assert_eq!(
		"\"\"".parse::<JsonValue>(),
		Ok(JsonValue::String("".into()))
	);
}

#[test]
fn test_string() {
	assert_eq!(
		"\"cool\\\" \\t\\t \\\\string\"".parse::<JsonValue>(),
		Ok(JsonValue::String("cool\" \t\t \\string".into()))
	);
}

#[test]
fn test_codepoint() {
	assert_eq!(
		"\"\\u0045 \\u0046 \\u0047\"".parse::<JsonValue>(),
		Ok(JsonValue::String("E F G".into()))
	);
}

#[test]
fn test_bad_escape_seq() {
	assert!("\"\\a\"".parse::<JsonValue>().is_err());
}

#[test]
fn test_list() {
	assert_eq!(
		"[1,null,4]".parse::<JsonValue>(),
		Ok(JsonValue::List(vec![
			1_f64.try_into().unwrap(),
			JsonValue::Null,
			4_f64.try_into().unwrap(),
		]))
	);
}

#[test]
fn test_nested_list() {
	assert_eq!(
		"[[2],\"hi\"]".parse::<JsonValue>(),
		Ok(JsonValue::List(vec![
			JsonValue::List(vec![JsonValue::Number(2_f64.try_into().unwrap())]),
			JsonValue::String("hi".into()),
		]))
	)
}

#[test]
fn test_complex_list() {
	assert_eq!(
		"[1,[2,null],[4,[5]]]".parse::<JsonValue>(),
		Ok(JsonValue::List(vec![
			JsonValue::Number(1_f64.try_into().unwrap()),
			JsonValue::List(vec![
				JsonValue::Number(2_f64.try_into().unwrap()),
				JsonValue::Null
			]),
			JsonValue::List(vec![
				JsonValue::Number(4_f64.try_into().unwrap()),
				JsonValue::List(vec![JsonValue::Number(5_f64.try_into().unwrap())])
			])
		]))
	)
}

#[test]
fn test_obj() {
	assert_eq!(
		"{\"hi\": 5.1}".parse::<JsonValue>(),
		Ok(JsonValue::Object(HashMap::from([(
			"hi".into(),
			JsonValue::Number(5.1_f64.try_into().unwrap())
		)])))
	)
}

#[test]
fn test_nested_obj() {
	assert_eq!(
		"{\"outer\": {\"inner\": 42}}".parse::<JsonValue>(),
		Ok(JsonValue::Object(HashMap::from([(
			"outer".into(),
			JsonValue::Object(HashMap::from([(
				"inner".into(),
				JsonValue::Number(42.0_f64.try_into().unwrap())
			)]))
		)])))
	);
}

#[test]
fn test_large_obj() {
	assert_eq!(
		"{\"first\": 1, \"second\": 2}".parse::<JsonValue>(),
		Ok(JsonValue::Object(HashMap::from([
			("first".into(), JsonValue::Number(1_f64.try_into().unwrap())),
			(
				"second".into(),
				JsonValue::Number(2_f64.try_into().unwrap())
			)
		])))
	);
}

#[test]
fn test_complex_nested_json() {
	assert_eq!(
		"{\"level1\": {\"level2\": {\"level3\": {\"num\": 123, \"text\": \"hello\", \"array\": [true, false, null, 3.15], \"obj\": {\"key\": \"value\"}}}}}".parse::<JsonValue>(),
		Ok(JsonValue::Object(HashMap::from([
			("level1".into(), JsonValue::Object(HashMap::from([
				("level2".into(), JsonValue::Object(HashMap::from([
					("level3".into(), JsonValue::Object(HashMap::from([
						("num".into(), JsonValue::Number(123.0_f64.try_into().unwrap())),
						("text".into(), JsonValue::String("hello".into())),
						("array".into(), JsonValue::List(vec![
							JsonValue::Boolean(true),
							JsonValue::Boolean(false),
							JsonValue::Null,
							JsonValue::Number(3.15_f64.try_into().unwrap())
						])),
						("obj".into(), JsonValue::Object(HashMap::from([
						("key".into(), JsonValue::String("value".into()))
						])))
					])))
				])))
			])))
		])))
	);
}

#[test]
fn test_deeply_nested_json() {
	assert_eq!(
		"{\"a\": {\"b\": {\"c\": {\"d\": {\"e\": {\"f\": 42}}}}}}".parse::<JsonValue>(),
		Ok(JsonValue::Object(HashMap::from([(
			"a".into(),
			JsonValue::Object(HashMap::from([(
				"b".into(),
				JsonValue::Object(HashMap::from([(
					"c".into(),
					JsonValue::Object(HashMap::from([(
						"d".into(),
						JsonValue::Object(HashMap::from([(
							"e".into(),
							JsonValue::Object(HashMap::from([(
								"f".into(),
								JsonValue::Number(42.0_f64.try_into().unwrap())
							)]))
						)]))
					)]))
				)]))
			)]))
		)])))
	);
}

#[test]
fn test_mixed_data_types_json() {
	assert_eq!(
		"{\"string\": \"example\", \"boolean\": true, \"null_value\": null, \"number\": 99.98, \"list\": [1, \"two\", false, {\"nested\": \"yes\"}]}".parse::<JsonValue>(),
		Ok(JsonValue::Object(HashMap::from([
			("string".into(), JsonValue::String("example".into())),
			("boolean".into(), JsonValue::Boolean(true)),
			("null_value".into(), JsonValue::Null),
			("number".into(), JsonValue::Number(99.98_f64.try_into().unwrap())),
			("list".into(), JsonValue::List(vec![
				JsonValue::Number(1.0_f64.try_into().unwrap()),
				JsonValue::String("two".into()),
				JsonValue::Boolean(false),
				JsonValue::Object(HashMap::from([
					("nested".into(), JsonValue::String("yes".into()))
				]))
			]))
		])))
	);
}

#[test]
fn test_empty_json_object() {
	assert_eq!(
		"{}".parse::<JsonValue>(),
		Ok(JsonValue::Object(HashMap::new()))
	);
}

#[test]
fn test_empty_json_array() {
	assert_eq!("[]".parse::<JsonValue>(), Ok(JsonValue::List(vec![])));
}

#[test]
fn test_nested_empty_structures() {
	assert_eq!(
		"{\"a\": {}, \"b\": [], \"c\": {\"d\": []}}".parse::<JsonValue>(),
		Ok(JsonValue::Object(HashMap::from([
			("a".into(), JsonValue::Object(HashMap::new())),
			("b".into(), JsonValue::List(vec![])),
			(
				"c".into(),
				JsonValue::Object(HashMap::from([("d".into(), JsonValue::List(vec![]))]))
			)
		])))
	);
}

#[test]
fn test_error_missing_quote() {
	assert!("\"unclosed string".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_invalid_number() {
	assert!("123abc".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_leading_zeros() {
	assert!("024".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_leading_zeros_negative() {
	assert!("-024".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_unexpected_character() {
	assert!("#invalid".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_incomplete_object() {
	assert!("{\"key\": \"value\"".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_incomplete_array() {
	assert!("[1, 2, 3".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_single_bracket() {
	assert!("[".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_leading_ws() {
	assert!(" []".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_extra_closing_bracket() {
	assert!("[]]".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_mismatched_brackets_array_object() {
	assert!("[ }".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_mismatched_brackets_object_array() {
	assert!("{ ]".parse::<JsonValue>().is_err());
}

#[test]
fn test_error_missing_comma() {
	assert!(r#"{"one":1 "two":2}"#.parse::<JsonValue>().is_err());
}

#[test]
fn test_super_complex_nested_json() {
	let json_str = r#"{
		"level1": {
			"array": [
				{"key1": "value1"},
				{"key2": 2},
				[
					{"deep": true},
					{"deeper": [null, {"even_deeper": "end"}]}
				]
			],
			"object": {
				"nested1": {
					"nested2": {
						"nested3": {
							"num": 123.456,
							"bool": false,
							"str": "nested text",
							"inner_array": [
								{"a": 1},
								{"b": 2},
								{"c": {"d": "deepest"}}
							]
						}
					}
				}
			},
			"simple": "test"
		},
		"root_array": [
			1,
			2,
			3,
			{"final": "object"}
		]
	}"#;

	let mut inner_most_object = HashMap::new();
	inner_most_object.insert("even_deeper".into(), JsonValue::String("end".into()));

	let mut deeper_object: HashMap<String, JsonValue> = HashMap::new();
	deeper_object.insert(
		"deeper".into(),
		JsonValue::List(vec![JsonValue::Null, JsonValue::Object(inner_most_object)]),
	);

	let deep_array = JsonValue::List(vec![
		JsonValue::Object(HashMap::from([("deep".into(), JsonValue::Boolean(true))])),
		JsonValue::Object(HashMap::from([(
			"deeper".into(),
			JsonValue::List(vec![
				JsonValue::Null,
				JsonValue::Object(HashMap::from([(
					"even_deeper".into(),
					JsonValue::String("end".into()),
				)])),
			]),
		)])),
	]);

	let level1_array = JsonValue::List(vec![
		JsonValue::Object(HashMap::from([(
			"key1".into(),
			JsonValue::String("value1".into()),
		)])),
		JsonValue::Object(HashMap::from([(
			"key2".into(),
			JsonValue::Number(2.0_f64.try_into().unwrap()),
		)])),
		deep_array,
	]);

	let mut inner_array_object = HashMap::new();
	inner_array_object.insert("a".into(), JsonValue::Number(1.0_f64.try_into().unwrap()));
	let mut inner_array_object2 = HashMap::new();
	inner_array_object2.insert("b".into(), JsonValue::Number(2.0_f64.try_into().unwrap()));
	let mut inner_obj = HashMap::new();
	inner_obj.insert("d".into(), JsonValue::String("deepest".into()));
	let mut inner_array_object3 = HashMap::new();
	inner_array_object3.insert("c".into(), JsonValue::Object(inner_obj));

	let inner_array = JsonValue::List(vec![
		JsonValue::Object(inner_array_object),
		JsonValue::Object(inner_array_object2),
		JsonValue::Object(inner_array_object3),
	]);

	let mut nested3 = HashMap::new();
	nested3.insert(
		"num".into(),
		JsonValue::Number(123.456_f64.try_into().unwrap()),
	);
	nested3.insert("bool".into(), JsonValue::Boolean(false));
	nested3.insert("str".into(), JsonValue::String("nested text".into()));
	nested3.insert("inner_array".into(), inner_array);

	let mut nested2 = HashMap::new();
	nested2.insert("nested3".into(), JsonValue::Object(nested3));

	let mut nested1 = HashMap::new();
	nested1.insert("nested2".into(), JsonValue::Object(nested2));

	let mut object = HashMap::new();
	object.insert("nested1".into(), JsonValue::Object(nested1));

	let mut level1_object = HashMap::new();
	level1_object.insert("array".into(), level1_array);
	level1_object.insert("object".into(), JsonValue::Object(object));
	level1_object.insert("simple".into(), JsonValue::String("test".into()));

	let root_array = JsonValue::List(vec![
		JsonValue::Number(1.0_f64.try_into().unwrap()),
		JsonValue::Number(2.0_f64.try_into().unwrap()),
		JsonValue::Number(3.0_f64.try_into().unwrap()),
		JsonValue::Object(HashMap::from([(
			"final".into(),
			JsonValue::String("object".into()),
		)])),
	]);

	let expected = JsonValue::Object(HashMap::from([
		("level1".into(), JsonValue::Object(level1_object)),
		("root_array".into(), root_array),
	]));

	assert_eq!(json_str.parse::<JsonValue>(), Ok(expected));
}
#[test]
fn serialize_simple() {
	let json = json!([45, 1, 45]);
	dbg!(json.to_string());
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_complex_object() {
	let json = json!({
		"name": "John",
		"address": { "street": "123 Main St", "city": "Anytown" },
		"phoneNumbers": ["123-4567", "987-6543"],
		"isActive": true,
		"age": 28,
		"preferences": { "notifications": true, "theme": "dark" }
	});
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_object_with_mixed_types() {
	let json = json!({
		"stringField": "Hello",
		"intField": 100,
		"floatField": 3.14,
		"boolField": false,
		"arrayField": [1, 2, 3],
		"nestedField": { "innerField": "nested" }
	});
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_object_with_special_characters() {
	let json = json!({
		"name": "John\nDoe",
		"description": "A string with special characters: \t\r\\",
		"quote": "\"Hello World!\"",
		"new\nline": "Huh?"
	});
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_large_number() {
	let json = json!({
		"bigNumber": 1234567890123456789012345678901234567890_f64
	});
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_array_with_objects() {
	let json = json!([
		{ "name": "Alice", "age": 30 },
		{ "name": "Bob", "age": 25 },
		{ "name": "Charlie", "age": 35 }
	]);
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_mixed_nested() {
	let json = json!({
		"id": 123,
		"details": {
			"name": "Sample",
			"meta": { "version": "1.0", "status": "active" }
		},
		"tags": ["tag1", "tag2"],
		"values": [100, 200, 300]
	});
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_array_with_different_data_types() {
	let json = json!([
		0.5,
		"hello",
		true,
		null,
		{ "key": "value" },
		[1, 2, 3]
	]);
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_object_with_date_string() {
	let json = json!({
		"event": "Conference",
		"date": "2025-03-15T14:30:00Z"
	});
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_object() {
	let json = json!({
		"hello": 1,
		"world": 2
	});
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn serialize_empty_object() {
	let json = json!({
		"empty": {},
		"also empty": {},
		"as well": []
	});
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}

#[test]
fn test_massive_object() {
	let json = ("{\"ABSOLUTELY MASSIVE BOI\": ".repeat(100000)
		+ &"[".repeat(100000)
		+ &"]".repeat(100000)
		+ &"}".repeat(100000))
		.parse::<JsonValue>()
		.unwrap();

	assert_eq!(json, json.clone());
	assert_eq!(json, json.to_string().parse::<JsonValue>().unwrap());
}
