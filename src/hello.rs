pub fn hello(path: &str) -> String {
	let name_start = path.find('=').unwrap_or(path.len()) + 1;
	let name = &path[name_start..];
	let content = format!("Hello, {name}!");
	let length: usize = content.len();
	return format!("HTTP/1.1 200 OK\nContent-Length: {length}\n\n{content}");
}
