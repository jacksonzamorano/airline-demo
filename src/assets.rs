pub struct Assets {}
impl Assets {
	const K_INDEX:&'static str = "<!DOCTYPE html>
<!DOCTYPE html>
<html>
<head>
	<meta charset=\"utf-8\">
	<meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">
	<title>Test</title>
</head>
<body>
	<h1>Thiis is my live server!</h1>
</body>
</html>";

	pub fn index() -> &'static str{
		return Assets::K_INDEX;
	}

}