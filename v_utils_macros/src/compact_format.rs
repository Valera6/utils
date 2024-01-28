extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident};

//TODO!: get rid of the dependency \
pub use anyhow::{Error, Result};
// here to be used when interacting with the library from outside, so if I change it, nothing outside of library breaks
pub const COMPACT_FORMAT_DELIMITER: char = ':';

// need to make a `const fn` that will be exported right before the macros, which produces an array of all acceptable names for referencing the struct.

/// A brain-dead child format of mine. Idea is to make parameter specification as compact as possible. Very similar to how you would pass arguments to `clap`, but here all the args are [arg(short)] by default, and instead of spaces, equal signs, and separating names from values, we write `named_argument: my_value` as `-nmy_value`. Entries are separated by ':' char.
///```rust
///use v_utils::init_compact_format;
///use v_utils::trades::{Timeframe, TimeframeDesignator};
///
///init_compact_format!(SAR, [(start, f64), (increment, f64), (max, f64), (timeframe, Timeframe)]);
///
///fn main() {
///	let sar = SAR { start: 0.07, increment: 0.02, max: 0.15, timeframe: Timeframe { designator: TimeframeDesignator::Minutes, n: 5 } };
///	let params_string = "sar:s0.07:i0.02:m0.15:t5m";
/// let without_name = params_string.splitn(2, ':').collect::<Vec<_>>()[1];
///	assert_eq!(sar, without_name.parse::<SAR>().unwrap());
///}
///```
#[macro_export]
macro_rules! init_compact_format {
	($name:ident, [ $(($field:ident, $field_type:ty)),* ]) => {
#[derive(Clone, Debug, PartialEq)]
pub struct $name {
$(
pub $field: $field_type,
)*
}
///NB: Note that FromStr takes string withot $name, while Display prints it with $name
/// Not sure if that's a good idea, but no clue how to fix.
impl std::str::FromStr for $name {
	type Err = v_utils::data::compact_format::Error;

	fn from_str(s: &str) -> v_utils::data::compact_format::Result<Self> {
		let parts = s.split(':').collect::<Vec<_>>();
		let mut fields = Vec::new();
		$(
		fields.push(stringify!($field));
		)*
		assert_eq!(parts.len(), fields.len(), "Incorrect number of parameters provided");

		let mut provided_params: std::collections::HashMap<char, &str> = std::collections::HashMap::new();
		//- instead of discarding [0], which is the name, want to assert it is either of: [$name, {name but as if it was a field}, or {name but only capitalized letters; eithre all capitalized or all lowercase}]
		for param in s.split(':') {
			if let Some(first_char) = param.chars().next() {
				let value = &param[1..];
				provided_params.insert(first_char, value);
			}
		}

		Ok($name {
		$(
		$field: {
			let first_char = stringify!($field).chars().next().unwrap();
			let value = provided_params.get(&first_char).unwrap().parse::<$field_type>()?;
			value
		},
		)*
		})
	}
}

impl std::fmt::Display for $name {
	fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
		let struct_name = stringify!($name).to_lowercase();
		write!(f, "{}", struct_name)?;

		$(
			write!(f, "-{}{}", stringify!($field).chars().next().unwrap(), self.$field)?;
		)*

		Ok(())
	}
}
};}

/////BUG: will not work if any of the child structs share the same accronym.
//// must end with 's'
////- umbrella_compact_optional!(Protocol, [SAR, TrailingStop, TpSl, LeadingCrosses]);
//// and then if need a wrapper
////- umbrella_compact_optional_wrapped!(Protocol, ProtocolWrapper::new([SAR, TrailingStop, TpSl, LeadingCrosses]);
////TODO!: assert that first split on "::" is followed by "new("
////TODO!!!!!!!!!!!!!: implement Umbrella struct constructor

//#[proc_macro]
//pub fn pascal_to_snake(input: TokenStream) -> TokenStream {
//	let input = parse_macro_input!(input as DeriveInput);
//	let name = input.ident;
//	let snake_case_name = to_snake_case(&name.to_string());
//	let snake_case_ident = Ident::new(&snake_case_name, name.span());
//	TokenStream::from(snake_case_ident)
//}
//
//fn to_snake_case(s: &str) -> String {
//	let mut snake_case = String::new();
//	for (i, char) in s.chars().enumerate() {
//		if char.is_uppercase() && i != 0 {
//			snake_case.push('_');
//		}
//		snake_case.push(char.to_lowercase().next().unwrap());
//	}
//	snake_case
//}
//
//#[macro_export]
//macro_rules! umbrella_compact_optional {
//	($name:ident, [ $struct: ty, * ]) => {
//#[derive(Clone, Debug)]
//pub enum concat_idents!($name, s) {
//	$(
//		pascal_to_snake_case!($struct): $struct,
//	)*
//}
//};}
////- umbrella_compact_optional!(Protocol, [SAR, TrailingStop, TpSl, LeadingCrosses]);
//
//#[cfg(test)]
//mod tests {
//	use super::*;
//	use proc_macro::TokenStream;
//	use quote::quote;
//
//	#[test]
//	fn test_pascal_to_snake() {
//		let input = TokenStream::from(quote! {
//			struct TestStruct;
//		});
//
//		let expected_output = "test_struct".to_string();
//		let actual_output = pascal_to_snake(input).to_string();
//
//		assert!(
//			actual_output.contains(&expected_output),
//			"Expected '{}', found '{}'",
//			expected_output,
//			actual_output
//		);
//	}
//}
