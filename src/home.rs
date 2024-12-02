pub fn home() -> String{
	let content: String = String::from("
		<!DOCTYPE html>
		<html lang='en'>
			<head>
				<meta charset='utf-8'>
				<title>Hello!</title>
			</head>
			<body>
				<h1>Hello!</h1>
				<p>Hi from Rust</p>
			</body>
		</html>");

	let length: usize = content.len();

	return format!("HTTP/1.1 200 OK\nContent-Length: {length}\n\n{content}");
}
