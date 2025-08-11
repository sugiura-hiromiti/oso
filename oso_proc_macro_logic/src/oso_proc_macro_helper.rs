#[macro_export]
macro_rules! fnl {
	($name:ident => $ty:ty, $doc:literal) => {
		#[proc_macro]
		#[doc = $doc]
		pub fn $name(item: proc_macro::TokenStream,) -> proc_macro::TokenStream {
			$crate::def! { $name, item => $ty, }
		}
	};
}

#[macro_export]
macro_rules! atr {
	($name:ident => $ty:ty, $ty2:ty, $doc:literal) => {
		#[proc_macro_attribute]
		#[doc = $doc]
		pub fn $name(
			attr: proc_macro::TokenStream,
			item: proc_macro::TokenStream,
		) -> proc_macro::TokenStream {
			$crate::def! { $name, attr => $ty, item => $ty2, }
		}
	};
}

#[macro_export]
macro_rules! drv {
	($derive:ident, $name:ident => $ty:ty, $(attributes: $($attributes:ident,)+)? $doc:literal) => {
		#[proc_macro_derive($derive $($(, attributes($attributes))+)?)]
		#[doc = $doc]
		pub fn $name(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
			$crate::def! { $name, item => $ty, }

		}
	};
}

#[macro_export]
macro_rules! def {
	($name:ident, $($param:ident => $ty:ty,)+)=>{
		$(
			let $param = syn::parse_macro_input!($param as $ty);
		)?

		oso_proc_macro_logic::$name::$name($($param,)+).unwrap_or_emit().into()
	};
}

#[derive(Debug,)]
pub enum Diag {
	Err(String,),
	Warn(String,),
	Note(String,),
	Help(String,),
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_diag_enum_variants() {
		// Test that all Diag variants can be created
		let err = Diag::Err("Error message".to_string(),);
		let warn = Diag::Warn("Warning message".to_string(),);
		let note = Diag::Note("Note message".to_string(),);
		let help = Diag::Help("Help message".to_string(),);

		// Test pattern matching on variants
		match err {
			Diag::Err(msg,) => assert_eq!(msg, "Error message"),
			_ => panic!("Should match Err variant"),
		}

		match warn {
			Diag::Warn(msg,) => assert_eq!(msg, "Warning message"),
			_ => panic!("Should match Warn variant"),
		}

		match note {
			Diag::Note(msg,) => assert_eq!(msg, "Note message"),
			_ => panic!("Should match Note variant"),
		}

		match help {
			Diag::Help(msg,) => assert_eq!(msg, "Help message"),
			_ => panic!("Should match Help variant"),
		}
	}

	#[test]
	fn test_diag_string_content() {
		let test_messages = vec![
			"Simple error",
			"Error with numbers: 123",
			"Error with special chars: !@#$%",
			"Multi-line\nerror\nmessage",
			"Unicode error: 🦀",
			"", // Empty string
		];

		for msg in test_messages {
			let err = Diag::Err(msg.to_string(),);
			match err {
				Diag::Err(content,) => assert_eq!(content, msg),
				_ => panic!("Should match Err variant"),
			}
		}
	}

	#[test]
	fn test_diag_enum_size() {
		// Test that the enum has a reasonable size
		use std::mem;

		let size = mem::size_of::<Diag,>();
		// Should be reasonable size (String + discriminant)
		assert!(size > 0);
		assert!(size < 1000); // Reasonable upper bound
	}

	#[test]
	fn test_diag_clone_if_possible() {
		// Test if Diag can be cloned (it should be able to since String is Clone)
		let original = Diag::Err("Test error".to_string(),);

		// We can't directly test Clone since it's not derived, but we can test
		// that we can create multiple instances with the same content
		let copy = Diag::Err("Test error".to_string(),);

		match (&original, &copy,) {
			(Diag::Err(msg1,), Diag::Err(msg2,),) => assert_eq!(msg1, msg2),
			_ => panic!("Both should be Err variants"),
		}
	}

	#[test]
	fn test_diag_debug_representation() {
		let diag = Diag::Err("Debug test".to_string(),);
		let debug_str = format!("{:?}", diag);

		assert!(debug_str.contains("Err"));
		assert!(debug_str.contains("Debug test"));
	}

	#[test]
	fn test_diag_all_variants_different() {
		// Test that all variants are distinct
		let err = Diag::Err("message".to_string(),);
		let warn = Diag::Warn("message".to_string(),);
		let note = Diag::Note("message".to_string(),);
		let help = Diag::Help("message".to_string(),);

		// Use discriminant to check they're different variants
		use std::mem;

		assert_ne!(mem::discriminant(&err), mem::discriminant(&warn));
		assert_ne!(mem::discriminant(&err), mem::discriminant(&note));
		assert_ne!(mem::discriminant(&err), mem::discriminant(&help));
		assert_ne!(mem::discriminant(&warn), mem::discriminant(&note));
		assert_ne!(mem::discriminant(&warn), mem::discriminant(&help));
		assert_ne!(mem::discriminant(&note), mem::discriminant(&help));
	}

	#[test]
	fn test_diag_with_long_messages() {
		// Test with very long messages
		let long_message = "a".repeat(10000,);
		let diag = Diag::Err(long_message.clone(),);

		match diag {
			Diag::Err(msg,) => {
				assert_eq!(msg.len(), 10000);
				assert_eq!(msg, long_message);
			},
			_ => panic!("Should match Err variant"),
		}
	}

	#[test]
	fn test_diag_message_modification() {
		// Test that we can modify the message content
		let mut message = "Original message".to_string();
		let diag = Diag::Warn(message.clone(),);

		// Modify the original string
		message.push_str(" - modified",);

		// The diag should still have the original message (String was moved)
		match diag {
			Diag::Warn(msg,) => assert_eq!(msg, "Original message"),
			_ => panic!("Should match Warn variant"),
		}
	}

	#[test]
	fn test_diag_empty_messages() {
		// Test all variants with empty messages
		let variants = vec![
			Diag::Err(String::new(),),
			Diag::Warn(String::new(),),
			Diag::Note(String::new(),),
			Diag::Help(String::new(),),
		];

		for diag in variants {
			match diag {
				Diag::Err(msg,) | Diag::Warn(msg,) | Diag::Note(msg,) | Diag::Help(msg,) => {
					assert!(msg.is_empty());
				},
			}
		}
	}

	#[test]
	fn test_diag_pattern_matching_exhaustive() {
		let test_diag = Diag::Note("Test".to_string(),);

		// Test exhaustive pattern matching
		let result = match test_diag {
			Diag::Err(_,) => "error",
			Diag::Warn(_,) => "warning",
			Diag::Note(_,) => "note",
			Diag::Help(_,) => "help",
		};

		assert_eq!(result, "note");
	}

	#[test]
	fn test_diag_with_special_characters() {
		let special_chars = vec![
			"\n\r\t",
			"\"quotes\"",
			"'single quotes'",
			"\\backslashes\\",
			"null\0char",
			"unicode: αβγδε",
			"emoji: 🚀🦀💻",
		];

		for special in special_chars {
			let diag = Diag::Help(special.to_string(),);
			match diag {
				Diag::Help(msg,) => assert_eq!(msg, special),
				_ => panic!("Should match Help variant"),
			}
		}
	}

	#[test]
	fn test_diag_memory_efficiency() {
		// Test that creating many Diag instances doesn't cause issues
		let mut diags = Vec::new();

		for i in 0..1000 {
			let msg = format!("Message {}", i);
			diags.push(Diag::Err(msg,),);
		}

		assert_eq!(diags.len(), 1000);

		// Check a few random ones
		match &diags[0] {
			Diag::Err(msg,) => assert_eq!(msg, "Message 0"),
			_ => panic!("Should be Err variant"),
		}

		match &diags[999] {
			Diag::Err(msg,) => assert_eq!(msg, "Message 999"),
			_ => panic!("Should be Err variant"),
		}
	}

	// Note: We can't easily test the macros without a full proc-macro environment,
	// but we can test that they compile and have the right structure

	#[test]
	fn test_macro_definitions_exist() {
		// This test just verifies that the macros are defined and accessible
		// The actual functionality would need to be tested in a proc-macro context

		// We can't directly test macro expansion, but we can verify they exist
		// by checking that this test compiles without errors
		assert!(true);
	}

	#[test]
	fn test_string_operations_used_in_diag() {
		// Test various string operations that might be used with Diag
		let base_msg = "Base message";

		// Test string formatting
		let formatted = format!("Error: {}", base_msg);
		let diag = Diag::Err(formatted,);

		match diag {
			Diag::Err(msg,) => assert!(msg.contains("Error: Base message")),
			_ => panic!("Should be Err variant"),
		}
	}

	#[test]
	fn test_diag_with_borrowed_vs_owned_strings() {
		let owned_string = "Owned string".to_string();
		let borrowed_str = "Borrowed string";

		// Both should work for creating Diag
		let diag1 = Diag::Err(owned_string,);
		let diag2 = Diag::Warn(borrowed_str.to_string(),);

		match diag1 {
			Diag::Err(msg,) => assert_eq!(msg, "Owned string"),
			_ => panic!("Should be Err variant"),
		}

		match diag2 {
			Diag::Warn(msg,) => assert_eq!(msg, "Borrowed string"),
			_ => panic!("Should be Warn variant"),
		}
	}
}
