pub fn format_html(_css: &str, _js: &str) -> String {
	let html_code = format!(
"<!DOCTYPE html>
<html>
<head>
	<meta charset='utf-8'>
	<meta name='viewport' content='width=device-width, initial-scale=1'>
	<link rel='stylesheet' type='text/css' href='{}'>
	<title></title>
</head>
<body>
<script type='text/javascript' src='{}'></script>
</body>
</html>"
, _css, _js);
html_code
}

pub fn return_css() -> String {
	let css_code = 
"* {
	margin: 0;
	padding: 0;
}

body {
	background-color: gray;
}";

css_code.to_string()
}

pub fn return_php() -> String {
	let php_code = "<?php echo 'placeholder'; ?>";
	php_code.to_string()
}