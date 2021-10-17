use tauri_hotkey::{parse_hotkey, HotkeyManager};
pub struct HotKey{
    BindingKeys : String,
    hk_manager : HotkeyManager
} 
impl Default for HotKey{
    fn default() -> Self {
        Self{
            BindingKeys : String::from("CMDORCTRL+S"),
            hk_manager : HotkeyManager::new(),
        }
    }
}
impl HotKey{
    pub fn register_hotkey(&mut self){
        let hk_manager = & mut self.hk_manager;
        if let Err(err) = hk_manager.register(parse_hotkey(self.BindingKeys.as_str()).unwrap(), move || {
            std::process::exit(0)
        }) {
            panic!("{:?}", err)
        }
    }
    pub fn unregister_all(&mut self){
        let _ = self.hk_manager.unregister_all();
    }
}