use proc_macro::{Group, Literal, Span, TokenStream, TokenTree};

pub struct Error {
	span: Span,
	message: String,
}

impl Error {
	pub fn new(span: Span, message: impl Into<String>) -> Self {
		let message = message.into();
		Self {
			span,
			message,
		}
	}

	pub fn call_site(message: impl Into<String>) -> Self {
		Self::new(Span::call_site(), message)
	}

	pub fn to_compile_error(&self) -> TokenStream {
		let mut message = TokenTree::Literal(Literal::string(&self.message));
		message.set_span(self.span);
		let mut args = TokenStream::new();
		args.extend([message]);
		let mut args =  TokenTree::Group(Group::new(proc_macro::Delimiter::Parenthesis, args));
		args.set_span(self.span);
		let mut compile_error: proc_macro::TokenStream = "::core::compile_error!".parse().unwrap();
		compile_error.extend([args]);
		compile_error
	}
}
