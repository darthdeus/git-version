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
	let version = describe_cwd().unwrap();

	TokenTree::Literal(Literal::string(&version)).into()
}
