extern crate proc_macro;

use proc_macro::{Literal, TokenStream, TokenTree};

mod utils;
use self::utils::describe_cwd;

/// Get the git version for the source code.
///
/// # Examples
///
/// ```ignore
/// const VERSION: &str = git_version!();
#[proc_macro]
pub fn git_version(_input: TokenStream) -> TokenStream {
	let git_args = vec!["--always".to_string(), "--dirty=-modified".to_string()];

	let version = describe_cwd(&git_args).unwrap();

	TokenTree::Literal(Literal::string(&version)).into()
}
