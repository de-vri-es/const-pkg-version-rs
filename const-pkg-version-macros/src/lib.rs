use proc_macro::{Group, Ident, Literal, Span, TokenStream, TokenTree};

mod error;
use error::Error;

#[proc_macro]
pub fn major(input: TokenStream) -> TokenStream {
	let tokens = match impl_u32_component(input, "CARGO_PKG_VERSION_MAJOR") {
		Ok(x) => x,
		Err(e) => return e.to_compile_error(),
	};
	surround_braces(tokens)
}

#[proc_macro]
pub fn minor(input: TokenStream) -> TokenStream {
	let tokens = match impl_u32_component(input, "CARGO_PKG_VERSION_MINOR") {
		Ok(x) => x,
		Err(e) => return e.to_compile_error(),
	};
	surround_braces(tokens)
}

#[proc_macro]
pub fn patch(input: TokenStream) -> TokenStream {
	let tokens = match impl_u32_component(input, "CARGO_PKG_VERSION_PATCH") {
		Ok(x) => x,
		Err(e) => return e.to_compile_error(),
	};
	surround_braces(tokens)
}

fn impl_u32_component(input: TokenStream, name: &str) -> Result<TokenStream, Error> {
	let _ = MacroInput::parse(input)?;
	let value = get_env_u32(name)?;
	Ok([TokenTree::Literal(Literal::u32_unsuffixed(value))].into_iter().collect())
}

#[proc_macro]
pub fn pre_release(input: TokenStream) -> TokenStream {
	let tokens = match impl_string_component(input, "CARGO_PKG_VERSION_PRE") {
		Ok(x) => x,
		Err(e) => return e.to_compile_error(),
	};
	surround_braces(tokens)
}

fn impl_string_component(input: TokenStream, name: &str) -> Result<TokenStream, Error> {
	let _ = MacroInput::parse(input)?;
	let value = get_env_str(name)?;
	let value = match value.as_str() {
		"" => None,
		x => Some(x),
	};
	Ok(option_str(value))
}

#[proc_macro]
pub fn build_metadata(input: TokenStream) -> TokenStream {
	let tokens = match impl_build_metadata(input, "CARGO_PKG_VERSION") {
		Ok(x) => x,
		Err(e) => return e.to_compile_error(),
	};
	surround_braces(tokens)
}

fn impl_build_metadata(input: TokenStream, name: &str) -> Result<TokenStream, Error> {
	let _ = MacroInput::parse(input)?;
	let value = get_env_str(name)?;
	let (_, build_metadata) = split_once_optional(&value, '+');
	Ok(option_str(build_metadata))
}

#[proc_macro]
pub fn full(input: TokenStream) -> TokenStream {
	let tokens = match impl_full(input, "CARGO_PKG_VERSION") {
		Ok(x) => x,
		Err(e) => return e.to_compile_error(),
	};
	surround_braces(tokens)
}

fn impl_full(input: TokenStream, name: &str) -> Result<TokenStream, Error> {
	let input = MacroInput::parse(input)?;
	let value = get_env_str(name)?;
	let version = PkgVersion::from_str(&value).map_err(Error::call_site)?;

	let mut output = StructBuilder::new_crate_local(input.self_crate, "Version");
	output.field("major", [TokenTree::Literal(Literal::u32_suffixed(version.major))]);
	output.field("minor", [TokenTree::Literal(Literal::u32_suffixed(version.minor))]);
	output.field("patch", [TokenTree::Literal(Literal::u32_suffixed(version.patch))]);
	output.field("pre_release", option_str(version.pre_release));
	output.field("build_metadata", option_str(version.build_metadata));

	Ok(output.finish())
}

fn get_env_str(name: &str) -> Result<String, Error> {
	match std::env::var(name) {
		Ok(x) => Ok(x),
		Err(std::env::VarError::NotPresent) => Err(Error::call_site(format!("environment variable {name} not set"))),
		Err(std::env::VarError::NotUnicode(_)) => Err(Error::call_site(format!("environment variable {name} contains non UTF-8 data"))),
	}
}

fn get_env_u32(name: &str) -> Result<u32, Error> {
	get_env_str(name)?
		.parse()
		.map_err(|e| Error::call_site(format!("environment variable {name} is not a valid u32: {e}")))
}

struct PkgVersion<'a> {
	major: u32,
	minor: u32,
	patch: u32,
	pre_release: Option<&'a str>,
	build_metadata: Option<&'a str>,
}

impl<'a> PkgVersion<'a> {
	fn from_str(input: &'a str) -> Result<Self, String> {
		let rest = &input;
		let (rest, build_metadata) = split_once_optional(rest, '+');
		let (rest, pre_release) = split_once_optional(rest, '-');
		let [major, minor, patch] = split_exact(rest, '.')
			.map_err(|()| "invalid version: expected MAJOR.MINOR.PATCH")?;

		let major = major.parse()
			.map_err(|e| format!("invalid major version number: {e}"))?;
		let minor = minor.parse()
			.map_err(|e| format!("invalid minor version number: {e}"))?;
		let patch = patch.parse()
			.map_err(|e| format!("invalid patch version number: {e}"))?;
		Ok(PkgVersion {
			major,
			minor,
			patch,
			pre_release,
			build_metadata,
		})
	}
}

struct MacroInput {
	self_crate: Ident,
}

impl MacroInput {
	fn parse(input: TokenStream) -> Result<MacroInput, Error> {
		let mut input = input.into_iter();

		let self_crate = input.next()
			.ok_or_else(|| Error::call_site("missing argument: `$crate`"))?;
		let self_crate = match self_crate {
			TokenTree::Ident(x) => x,
			other => return Err(Error::new(other.span(), "expected `$crate`")),
		};

		match input.next() {
			None => (),
			Some(TokenTree::Punct(x)) if x.as_char() == ',' => {
				match input.next() {
					None => (),
					Some(TokenTree::Punct(x)) => return Err(Error::new(x.span(), "unexpected puncutation")),
					Some(other) => return Err(Error::new(other.span(), "unexpected argument")),
				}
			},
			Some(x) => return Err(Error::new(x.span(), "unexpected token")),
		};

		Ok(MacroInput { self_crate })
	}
}


fn surround_braces(tokens: TokenStream) -> TokenStream {
	[TokenTree::Group(Group::new(proc_macro::Delimiter::Brace, tokens))].into_iter().collect()
}

fn surround_parens(tokens: TokenStream) -> TokenStream {
	let expr = TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, tokens));
	[expr].into_iter().collect()
}

fn split_once_optional(input: &str, delimiter: char) -> (&str, Option<&str>) {
	match input.split_once(delimiter) {
		Some((left, right)) => (left, Some(right)),
		None => (input, None),
	}
}

fn split_exact<const N: usize>(input: &str, delimiter: char) -> Result<[&str; N], ()> {
	let mut output = [""; N];
	let mut fields = input.split(delimiter);
	for output in &mut output {
		*output = fields.next().ok_or(())?;
	}

	if fields.next().is_some() {
		return Err(());
	}

	Ok(output)
}

fn tokens(input: &str) -> TokenStream {
	input.parse().unwrap()
}

struct StructBuilder {
	name: TokenStream,
	fields: TokenStream,
}

impl StructBuilder {
	fn new(name: TokenStream) -> Self {
		Self {
			name,
			fields: TokenStream::new(),
		}
	}

	fn new_crate_local(crate_ident: Ident, local_path: &str) -> Self {
		use proc_macro::{Punct, Spacing};

		let mut name = TokenStream::new();
		name.extend([
			TokenTree::Ident(crate_ident),
			TokenTree::Punct(Punct::new(':', Spacing::Joint)),
			TokenTree::Punct(Punct::new(':', Spacing::Alone)),
		]);
		name.extend(tokens(local_path));

		Self::new(name)
	}

	fn field(&mut self, name: &str, data: impl IntoIterator<Item = TokenTree>) {
		use proc_macro::{Punct, Spacing};

		let name = Ident::new(name, Span::call_site());
		self.fields.extend([
			TokenTree::Ident(name),
			TokenTree::Punct(Punct::new(':', Spacing::Alone)),
		]);
		self.fields.extend(data);
		self.fields.extend([
			TokenTree::Punct(Punct::new(',', Spacing::Alone)),
		]);
	}

	fn finish(self) -> TokenStream {
		let mut output = self.name;
		output.extend(surround_braces(self.fields));
		output
	}
}

fn option_str(input: Option<&str>) -> TokenStream {
	match input {
		None => tokens("::core::option::Option::None::<&::core::primitive::str>"),
		Some(x) => {
			let mut output = tokens("::core::option::Option::Some");
			output.extend(surround_parens([TokenTree::Literal(Literal::string(x))].into_iter().collect()));
			output
		}
	}
}
