# microjson

This is a simple dependency-free Rust JSON library, optimized not for speed but for conciseness. It's self-contained within a single file and can be copy-pasted directly into your application. The number of lines currently breaks down as:

- Parsing: 198 lines
- Serialization: 129 lines
- Everything else: 208 lines

For a total of 535 lines. Let me know if you can get this down even more -- but keep in mind that this isn't code golf: the code should still be (relatively) readable and idiomatic.

Despite its small size, it can do everything you'd expect from a typical JSON library.

## Example

```rs
use microjson::{HashMap, JsonValue, json};

fn main() {
	// Create a new JSON value
	let json = json!({
		"hello": "world!",
		"a number": 34.4,
		"a list": [false, "with new\nlines", 34.3, null, [{}]],
		"empty object": {}
	});

	// JSON values can be serialized with or without indentation
	println!("Printed compactly: {}", json);
	println!("Pretty-printed: {:?}", json);

	// Parse the serialized string
	let mut parsed: JsonValue = json.to_string().parse().unwrap();
	assert!(parsed == json);

	// Read the number
	let num = f64::try_from(&parsed["a list"][2]).unwrap();
	println!("The number in the list is: {}", num);

	// If you try to read a value as the wrong type, it won't work
	let is_it_an_object: Result<&HashMap<_, _>, _> = (&parsed["a list"][2]).try_into();
	assert!(is_it_an_object.is_err());

	// Overwrite the list
	// Note: all expressions, including negative numbers, must be surrounded by parentheses
	// Or you will get cryptic errors
	parsed["a list"] = json!([(-93), "hello", (-1.15 * 2.)]);
	let new_num = f64::try_from(&parsed["a list"][2]).unwrap();
	println!("Now the number is: {}", new_num);

	// We can work with absolutely massive objects
	let massive_json: JsonValue = ("{\"insanely huge JSON\": ".repeat(100000)
		+ &"[".repeat(100000)
		+ &"]".repeat(100000)
		+ &"}".repeat(100000))
		.parse()
		.unwrap();
	println!("Massive object length: {}", massive_json.to_string().len());
}
```

### A few implementation details to keep in mind

- Numbers are internally stored as finite f64s. NaN, infinity, and negative infinity will be automatically converted to `null`.
- JSON objects are implemented using a HashMap which is unordered. Therefore, converting `obj.to_string()` results in keys being output in a nondeterministic order.
- If a JSON string has duplicate keys, all but the last value will be thrown out during parsing.
- Creating a `&mut obj["key"]` will automatically insert `null` if the key doesn't exist. The reason for this unintuitive behaviour is so that you can write `obj["key"] = some_val` and have that always work without panicking. Ideally we would be able to use the (not yet implemented) [IndexSet](https://github.com/rust-lang/rfcs/issues/997) trait for this.
- `JsonValue::clone()` is not very efficient:

```rs
impl Clone for JsonValue {
	fn clone(&self) -> Self {
		self.to_string().parse().unwrap()
	}
}
```