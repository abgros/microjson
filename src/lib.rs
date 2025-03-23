pub use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter, Write};
use std::iter::repeat_with;
use std::mem::{forget, replace, take};
use std::ops::{Index, IndexMut};
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FiniteF64(f64);

pub enum JsonValue {
	Null,
	Boolean(bool),
	Number(FiniteF64),
	String(String),
	List(Vec<JsonValue>),
	Object(HashMap<String, JsonValue>),
}

#[macro_export]
macro_rules! json {
	(null) => {
		JsonValue::Null
	};
	([$($elem:tt),*]) => {
		JsonValue::List(vec![$(json!($elem)),*])
	};
	({$($key:tt : $val:tt),*}) => {
		JsonValue::Object(vec![
			$(($key.to_string(), json!($val))),*
		].into_iter().collect())
	};
	($e:expr) => {
		JsonValue::from($e)
	}
}

impl Drop for JsonValue {
	fn drop(&mut self) {
		if matches!(self, JsonValue::List(_) | JsonValue::Object(_)) {
			let mut stack = vec![replace(self, JsonValue::Null)];
			while let Some(mut last) = stack.pop() {
				match &mut last {
					JsonValue::List(lst) => stack.extend(take(lst)),
					JsonValue::Object(obj) => stack.extend(take(obj).into_values()),
					_ => continue,
				};
				forget(last);
			}
		}
	}
}

impl PartialEq for JsonValue {
	fn eq(&self, rhs: &Self) -> bool {
		let mut lhs_stack = vec![self];
		let mut rhs_stack = vec![rhs];

		while let (Some(lhs), Some(rhs)) = (lhs_stack.pop(), rhs_stack.pop()) {
			match (lhs, rhs) {
				(JsonValue::List(l), JsonValue::List(r)) if l.len() == r.len() => {
					lhs_stack.extend(l);
					rhs_stack.extend(r);
				}
				(JsonValue::Object(l), JsonValue::Object(r)) if l.len() == r.len() => {
					for (key, val1) in l {
						let Some(val2) = r.get(key) else {
							return false;
						};
						lhs_stack.push(val1);
						rhs_stack.push(val2);
					}
				}
				(JsonValue::Boolean(l), JsonValue::Boolean(r)) if l == r => {}
				(JsonValue::Number(l), JsonValue::Number(r)) if l == r => {}
				(JsonValue::String(l), JsonValue::String(r)) if l == r => {}
				(JsonValue::Null, JsonValue::Null) => {}
				_ => return false,
			}
		}
		true
	}
}

impl Eq for FiniteF64 {}
impl Eq for JsonValue {}

impl Clone for JsonValue {
	fn clone(&self) -> Self {
		self.to_string().parse().unwrap()
	}
}

impl TryFrom<f64> for FiniteF64 {
	type Error = &'static str;

	fn try_from(value: f64) -> Result<Self, Self::Error> {
		value
			.is_finite()
			.then_some(FiniteF64(value))
			.ok_or("value is not a finite number")
	}
}

macro_rules! impl_from {
	($($from:ty => $into:ty: $in:ident => $out:expr),*) => { $(
		impl<'a> From<$from> for $into {
			fn from(value: $from) -> Self {
				(|$in: $from| $out)(value)
			}
		}
	)* }
}

impl_from!(
	bool => JsonValue: val => JsonValue::Boolean(val),
	f64 => JsonValue: val => FiniteF64::try_from(val).map_or(JsonValue::Null, JsonValue::from),
	u32 => JsonValue: val => FiniteF64::try_from(val as f64).map(JsonValue::from).unwrap(),
	i32 => JsonValue: val => FiniteF64::try_from(val as f64).map(JsonValue::from).unwrap(),
	FiniteF64 => JsonValue: val => JsonValue::Number(val),
	&str => JsonValue: val => JsonValue::String(val.to_owned()),
	String => JsonValue: val => JsonValue::String(val),
	Vec<JsonValue> => JsonValue: val => JsonValue::List(val),
	HashMap<String, JsonValue> => JsonValue: val => JsonValue::Object(val),
	FiniteF64 => f64: val => val.0,
	&'a FiniteF64 => f64: val => val.0,
	&'a mut FiniteF64 => f64: val => val.0
);

macro_rules! impl_try_from {
	($($kind:ident: $type:ty),*) => { $(
		impl TryFrom<JsonValue> for $type {
			type Error = &'static str;

			fn try_from(mut value: JsonValue) -> Result<Self, Self::Error> {
				match &mut value {
					JsonValue::$kind(val) => Ok(take(val)),
					_ => Err(concat!("provided value is not a JSON ", stringify!($kind))),
				}
			}
		}
	)* }
}

impl_try_from!(Boolean: bool, String: String, List: Vec<JsonValue>, Object: HashMap<String, JsonValue>);

macro_rules! impl_try_from_ref {
	($($in:ty: $kind:ident => $out:ty),*) => { $(
		impl<'a> TryFrom<$in> for $out {
			type Error = &'static str;

			fn try_from(value: $in) -> Result<Self, Self::Error> {
				match value {
					JsonValue::$kind(val) => Ok(val.into()),
					_ => Err(concat!("provided value is not a JSON ", stringify!($kind))),
				}
			}
		}
	)* }
}

impl_try_from_ref!(
	JsonValue: Number => f64,
	&'a JsonValue: Number => f64,
	&'a mut JsonValue: Number => f64,

	&'a JsonValue: Boolean => &'a bool,
	&'a mut JsonValue: Boolean => &'a mut bool,

	&'a JsonValue: String => &'a String,
	&'a mut JsonValue: String => &'a mut String,

	&'a JsonValue: List => &'a Vec<JsonValue>,
	&'a mut JsonValue: List => &'a mut Vec<JsonValue>,

	&'a JsonValue: Object => &'a HashMap<String, JsonValue>,
	&'a mut JsonValue: Object => &'a mut HashMap<String, JsonValue>
);

impl Index<usize> for JsonValue {
	type Output = JsonValue;

	fn index(&self, idx: usize) -> &Self::Output {
		&(<&Vec<_>>::try_from(self).unwrap())[idx]
	}
}

impl Index<&str> for JsonValue {
	type Output = JsonValue;

	fn index(&self, key: &str) -> &Self::Output {
		&(<&HashMap<_, _>>::try_from(self).unwrap())[key]
	}
}

impl IndexMut<usize> for JsonValue {
	fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
		&mut (<&mut Vec<_>>::try_from(self).unwrap())[idx]
	}
}

impl IndexMut<&str> for JsonValue {
	fn index_mut(&mut self, key: &str) -> &mut Self::Output {
		let inner: &mut HashMap<_, _> = self.try_into().unwrap();
		inner.entry(key.into()).or_insert(JsonValue::Null)
	}
}

impl JsonValue {
	fn write_escaped(f: &mut Formatter, s: &str) -> Result<(), fmt::Error> {
		f.write_char('"')?;
		for c in s.chars() {
			match c {
				'"' => f.write_str("\\\"")?,
				'\n' => f.write_str("\\n")?,
				'\r' => f.write_str("\\r")?,
				'\t' => f.write_str("\\t")?,
				'\\' => f.write_str("\\\\")?,
				c if c == 8 as char => f.write_str("\\b")?,
				c if c == 12 as char => f.write_str("\\f")?,
				c if c.is_ascii_control() => write!(f, "\\u{:0>4x}", c as u32)?,
				c => f.write_char(c)?,
			}
		}
		f.write_char('"')
	}

	fn maybe_newline(f: &mut Formatter, flag: bool, depth: usize) -> Result<(), fmt::Error> {
		if !flag {
			return Ok(());
		}
		f.write_char('\n')?;
		repeat_with(|| f.write_char('\t')).take(depth).collect()
	}

	fn serialize(&self, f: &mut Formatter, pretty: bool) -> Result<(), fmt::Error> {
		enum StackItem<'a> {
			TopLevel,
			List(std::slice::Iter<'a, JsonValue>),
			Object(std::collections::hash_map::Iter<'a, String, JsonValue>),
		}

		let mut stack = vec![StackItem::TopLevel];
		let mut write_comma = false;
		let mut write_nl_before_val = false;
		let mut write_nl_after_val = true;

		loop {
			let next = match stack.last_mut() {
				Some(StackItem::TopLevel) => stack.pop().map(|_| self).unwrap(),
				Some(StackItem::List(iter)) => {
					let Some(val) = iter.next() else {
						stack.pop();
						JsonValue::maybe_newline(f, write_nl_after_val && pretty, stack.len())?;
						f.write_char(']')?;
						write_nl_after_val = true;
						write_comma = true;
						continue;
					};

					val
				}
				Some(StackItem::Object(iter)) => {
					let Some((key, val)) = iter.next() else {
						stack.pop();
						JsonValue::maybe_newline(f, write_nl_after_val && pretty, stack.len())?;
						f.write_char('}')?;
						write_nl_after_val = true;
						write_comma = true;
						continue;
					};

					if write_comma {
						f.write_char(',')?;
						write_comma = false;
					}

					JsonValue::maybe_newline(f, pretty, stack.len())?;
					JsonValue::write_escaped(f, key)?;
					f.write_char(':')?;
					write_nl_before_val = false;

					if pretty {
						f.write_char(' ')?;
					}

					val
				}
				None => return Ok(()),
			};

			if write_comma {
				f.write_char(',')?;
			}

			JsonValue::maybe_newline(f, write_nl_before_val && pretty, stack.len())?;
			write_nl_before_val = true;
			write_nl_after_val = true;
			write_comma = true;

			match next {
				JsonValue::List(ls) => {
					f.write_char('[')?;
					stack.push(StackItem::List(ls.iter()));
					write_comma = false;
					write_nl_after_val = !ls.is_empty();
				}
				JsonValue::Object(obj) => {
					f.write_char('{')?;
					stack.push(StackItem::Object(obj.iter()));
					write_comma = false;
					write_nl_after_val = !obj.is_empty();
				}
				JsonValue::Null => f.write_str("null")?,
				JsonValue::Number(num) => write!(f, "{}", f64::from(num))?,
				JsonValue::Boolean(b) => write!(f, "{b}")?,
				JsonValue::String(s) => JsonValue::write_escaped(f, s)?,
			};
		}
	}
}

impl Display for JsonValue {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		self.serialize(f, false)
	}
}

impl Debug for JsonValue {
	fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
		self.serialize(f, true)
	}
}

impl FromStr for JsonValue {
	type Err = String;

	fn from_str(input: &str) -> Result<Self, Self::Err> {
		#[derive(Clone, Copy, PartialEq)]
		enum Expecting {
			CommaOrBrace,
			CommaOrBracket,
			Key,
			KeyOrBrace,
			Value,
			ValueOrBracket,
		}
		use Expecting::*;

		let bytes = input.as_bytes();
		let mut stack = vec![];
		let mut key_stack = vec![];
		let mut i = 0;
		let mut expect = Value;

		loop {
			let next = match (bytes.get(i).ok_or("unexpected end of input")?, expect) {
				(b' ' | b'\t' | b'\n' | b'\r', _) if !stack.is_empty() => {
					i += 1;
					continue;
				}
				(b'{', Value | ValueOrBracket) => {
					i += 1;
					expect = KeyOrBrace;
					stack.push(HashMap::new().into());
					continue;
				}
				(b'}', CommaOrBrace | KeyOrBrace) => {
					i += 1;
					stack.pop().ok_or("unexpected closing brace")?
				}
				(b',', CommaOrBracket | CommaOrBrace) => {
					i += 1;
					expect = if expect == CommaOrBracket { Value } else { Key };
					continue;
				}
				(b'[', Value | ValueOrBracket) => {
					i += 1;
					expect = ValueOrBracket;
					stack.push(Vec::new().into());
					continue;
				}
				(b']', CommaOrBracket | ValueOrBracket) => {
					i += 1;
					stack.pop().ok_or("unexpected closing bracket")?
				}
				(b'"', Value | ValueOrBracket | Key | KeyOrBrace) => {
					i += 1;
					let mut s = String::new();

					loop {
						let end = i + bytes
							.get(i..)
							.unwrap_or_default()
							.iter()
							.position(|&c| c == b'"' || c == b'\\' || c.is_ascii_control())
							.ok_or("missing end quote")?;

						s.push_str(&input[i..end]);
						i = end;

						s.push(match (bytes[i], bytes.get(i + 1)) {
							(b'"', _) => break,
							(b'\\', Some(b'"')) => '"',
							(b'\\', Some(b'\\')) => '\\',
							(b'\\', Some(b'/')) => '/',
							(b'\\', Some(b'b')) => 8 as char,
							(b'\\', Some(b'f')) => 12 as char,
							(b'\\', Some(b'n')) => '\n',
							(b'\\', Some(b'r')) => '\r',
							(b'\\', Some(b't')) => '\t',
							(b'\\', Some(b'u')) => {
								let mut codepoint = input
									.get(i + 2..i + 6)
									.and_then(|s| u32::from_str_radix(s, 16).ok())
									.ok_or("invalid hex string")?;
								i += 4;

								let is_surrogate = matches!(codepoint, 0xd800..0xdc00);
								if is_surrogate && matches!(bytes.get(i + 2..i + 4), Some(b"\\u")) {
									codepoint = input
										.get(i + 4..i + 8)
										.and_then(|s| u32::from_str_radix(s, 16).ok())
										.ok_or("invalid hex string")?
										.checked_sub(0xdc00)
										.filter(|&num| num < 0xe000 - 0xdc00)
										.map(|num| 0x10000 + num + (codepoint - 0xd800) * 1024)
										.inspect(|_| i += 6)
										.unwrap_or(codepoint);
								}
								char::from_u32(codepoint).unwrap_or('�')
							}
							(b'\\', Some(c)) => Err(format!("invalid escape sequence: {c}"))?,
							(b'\\', None) => Err("missing escape sequence")?,
							(c, _) => Err(format!("illegal control character: 0x{c:x}"))?,
						});

						i += 2;
					}
					i += 1;

					if matches!(expect, Key | KeyOrBrace) {
						i += bytes
							.get(i..)
							.unwrap_or_default()
							.iter()
							.position(|&c| !matches!(c, b' ' | b'\t' | b'\n' | b'\r'))
							.and_then(|pos| (bytes[i + pos] == b':').then_some(pos + 1))
							.ok_or("missing colon")?;
						key_stack.push(s);
						expect = Value;
						continue;
					}

					JsonValue::from(s)
				}
				(b'-' | b'0'..=b'9', Value | ValueOrBracket) => {
					let mut decimal_places = 0;
					let is_negative = bytes[i] == b'-';
					i += if is_negative { 1 } else { 0 };

					let mut num = match (bytes.get(i), bytes.get(i + 1)) {
						(Some(b'0'), Some(b'0'..=b'9')) => Err("illegal leading zero")?,
						(Some(c @ b'0'..=b'9'), _) => (c - b'0') as f64,
						(Some(c), _) => Err(format!("unexpected character: {}", *c as char))?,
						(None, _) => Err("unexpected end of input")?,
					};

					loop {
						i += 1;
						match bytes.get(i) {
							Some(c @ b'0'..=b'9') if decimal_places > 0 => {
								num += (c - b'0') as f64 / 10_f64.powi(decimal_places);
								decimal_places += 1;
							}
							Some(c @ b'0'..=b'9') => num = num * 10. + (c - b'0') as f64,
							Some(b'.') if decimal_places == 0 => decimal_places = 1,
							Some(b'e' | b'E') => {
								i += 1;

								let mut exp = match bytes.get(i).ok_or("unexpected end of input")? {
									c @ b'0'..=b'9' => (c - b'0') as f64,
									b'-' | b'+' => 0.,
									c => Err(format!("unexpected character: {}", *c as char))?,
								};
								let exp_is_negative = bytes[i] == b'-';

								i += 1;
								while let Some(c @ b'0'..=b'9') = bytes.get(i) {
									exp = exp * 10. + (c - b'0') as f64;
									i += 1;
								}

								num *= 10_f64.powf(if exp_is_negative { -exp } else { exp });
								break;
							}
							_ => break,
						};
					}

					JsonValue::from(if is_negative { -num } else { num })
				}
				(b't', Value | ValueOrBracket) if bytes.get(i..i + 4) == Some(b"true") => {
					i += 4;
					JsonValue::from(true)
				}
				(b'f', Value | ValueOrBracket) if bytes.get(i..i + 5) == Some(b"false") => {
					i += 5;
					JsonValue::from(false)
				}
				(b'n', Value | ValueOrBracket) if bytes.get(i..i + 4) == Some(b"null") => {
					i += 4;
					JsonValue::Null
				}
				(&c, _) => Err(format!("unexpected character: {}", c as char))?,
			};

			match stack.last_mut() {
				Some(JsonValue::List(ls)) => {
					ls.push(next);
					expect = CommaOrBracket;
				}
				Some(JsonValue::Object(obj)) => {
					obj.insert(key_stack.pop().unwrap(), next);
					expect = CommaOrBrace;
				}
				_ if i == bytes.len() => return Ok(next),
				_ => {}
			};
		}
	}
}
