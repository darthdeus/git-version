use comfy_git_version::git_version;

#[test]
fn git_describe_is_right() {
	let vec = std::process::Command::new("git")
		.args(&["describe", "--always"])
		.output()
		.expect("failed to execute git")
		.stdout;
	let name = std::str::from_utf8(&vec[..vec.len() - 1]).expect("non-utf8 error?!");
	println!("name = {}", name);
	// println!("GIT_VERSION = {}", git_version!(args = ["--always", "--dirty=-modified"]));
	assert_eq!(git_version!(), name);
	// assert_eq!(git_describe!("--always", "--dirty=-modified"), name);
	// assert_eq!(git_version!(prefix = "[", suffix = "]"), format!("[{}]", name));
}
