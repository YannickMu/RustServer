pub fn hello(path: &str) -> String {
	let mut name: &str;
	if path.starts_with("/hello?name=") {
		let name_start = path.find('=').unwrap_or(path.len()) + 1;
		name = &path[name_start..];
	} else {
		name = "Stranger!\nI think you forgot the name parameter: /name?name=<your Name>";
	}
	let content = format!("Hello, {name}!");
	let length: usize = content.len();
	return format!("HTTP/1.1 200 OK\nContent-Length: {length}\n\n{content}");
}
