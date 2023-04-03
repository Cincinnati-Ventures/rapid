use syn::{File as SynFile, Item, Type, Generics};

const GENERATED_TS_FILE_MESSAGE: &str = "@generated automatically by Rapid-web (https://rapid.cincinnati.ventures). DO NOT CHANGE";

#[derive(Debug)]
pub enum TypeClass {
	InputBody,
	QueryParam,
	Path,
	Return,
	Invalid,
}

#[derive(Debug, Clone)]
pub enum HandlerRequestType {
	Get,
	Post,
	Delete,
	Put,
	Patch,
}

#[derive(Debug)]
pub struct HandlerType {
	pub type_value: Type,
	pub class: Option<TypeClass>,
	pub handler_type: HandlerRequestType,
}

pub fn extract_handler_types(route_source: &str) -> Option<Vec<Option<HandlerType>>> {
	let parsed_file: SynFile = syn::parse_str(route_source).unwrap();
	for item in parsed_file.items {
		// The first route handler that we find we want to break out
		// Any valid handler functions found after the first one are ignored (in rapid, only one handler is allowed per file)
		if let Item::Fn(function) = item {
			if is_valid_handler("rapid_handler", function.attrs) {
				let mut function_types: Vec<Option<HandlerType>> = Vec::new();
				let arg_types = function.sig.inputs.iter();
				let function_name = function.sig.ident;

				for type_value in arg_types {
					if let syn::FnArg::Typed(typed) = type_value {
						let rust_type = *typed.ty.clone();
						let type_class = get_type_class(rust_type.clone());

						function_types.push(Some(HandlerType {
							type_value: rust_type,
							class: type_class,
							handler_type: match function_name.to_string().as_str() {
								"get" => HandlerRequestType::Get,
								"post" => HandlerRequestType::Post,
								"delete" => HandlerRequestType::Delete,
								"put" => HandlerRequestType::Put,
								"patch" => HandlerRequestType::Patch,
								_ => HandlerRequestType::Get,
							},
						}));
					}
				}

				return Some(function_types);
			}
		}
	}

	None
}

pub fn get_type_class(rust_type: Type) -> Option<TypeClass> {
	match rust_type {
		Type::Reference(path) => get_type_class(*path.elem),
		Type::Path(path) => {
			let segment = path.path.segments.last().unwrap();
			let tokens = &segment.ident;

			Some(match tokens.to_string().as_str() {
				"RapidPath" => TypeClass::Path,
				"RapidQuery" => TypeClass::QueryParam,
				"RapidJson" => TypeClass::InputBody, // TODO: support return statements here as well (right now we are defaulting to invalid until implemented)
				_ => TypeClass::Invalid,
			})
		}
		_ => None,
	}
}

/// Method for checking if a handler function is valid
/// Handlers are only valid if they have a "#[rapid_handler]" macro on them
pub fn is_valid_handler(macro_name: &str, attributes: Vec<syn::Attribute>) -> bool {
	attributes
		.iter()
		.any(|attr| attr.path().segments.iter().any(|segment| segment.ident == macro_name))
}

/// Method for creating spacing for the generated typescript file by rapid
pub fn space(space_amount: u32) -> String {
	let mut space_string = "".to_string();
	for _ in 0..space_amount {
		space_string.push(' ');
	}
	space_string
}

/// Method for providing indentions within strings (mainly used in generated typescript file that is generated by rapid)
pub fn indent(amount: u32) -> String {
	let mut new_amount = String::new();

	for _ in 0..amount {
		new_amount.push('\n');
	}
	new_amount
}

/// Function for extracting generics from a rust struct
pub fn get_struct_generics(type_generics: Generics) -> String {
    let mut generic_params: Vec<String> = Vec::new();

    for generic_param in type_generics.params {
        if let syn::GenericParam::Type(rust_type) = generic_param {
            generic_params.push(rust_type.ident.to_string());
        }
    }

    if generic_params.is_empty() {
        "".to_string()
    } else {
        format!("<{}>", generic_params.join(", "))
    }
}
