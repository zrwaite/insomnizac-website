use wasm_bindgen::JsValue;

pub struct LocalStorage {
	storage: web_sys::Storage,
}	

impl LocalStorage {
	pub fn new() -> Option<LocalStorage> {
		let window = web_sys::window()?;
		let storage = window.local_storage().ok()?;
		let storage = storage?;
		Some(LocalStorage { storage })
	}
	pub fn set(&self, key: String, value: String) -> Result<(), JsValue> {
		self.storage.set_item(&key, &value)?;
		Ok(())
	}
	pub fn get(&self, key: String) -> Option<String> {
		self.storage.get_item(&key).ok()?
	}
	pub fn remove(&self, key: String) -> Result<(), JsValue> {
		self.storage.remove_item(&key)?;
		Ok(())
	}
}