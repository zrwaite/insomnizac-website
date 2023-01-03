pub fn alert_str(msg: &str) {
	web_sys::window().unwrap().alert_with_message(msg).unwrap();
}
pub fn alert(msg: String) {
	alert_str(&msg);
}