use eframe::epi;
use eframe::egui::containers::*;
use tauri_hotkey::{parse_hotkey, HotkeyManager};

pub struct TextEditor {
    pub content : String,
    pub content_copy : String,
    pub saveKey : String,
    pub BindingKeyManager : HotkeyManager,
    pub rec : Arc<Mutex<Receiver<String>>>,
    pub send : Sender<String>
}

impl epi::App for TextEditor{
    fn name(&self) -> &str {
        "text editor"
    }
    fn setup(&mut self, _ctx: &eframe::egui::CtxRef, _frame: &mut epi::Frame<'_>, _storage: Option<&dyn epi::Storage>) {
        
        install_fonts(_ctx);
        self.register_bindingKey();
    }
    fn update(&mut self, ctx: &eframe::egui::CtxRef, frame: &mut epi::Frame<'_>) {

        if self.content != self.content_copy{
            self.send.send(self.content.clone()).unwrap();
            self.content_copy = self.content.clone();
        }
        CentralPanel::default().show(ctx, |ui| {
            use eframe::egui::menu;

            menu::bar(ui, |ui| {
                menu::menu(ui, "File", |ui| {
                    if ui.button("save").clicked(){
                        use std::fs::File;
                        let mut f = File::create("foo.txt").expect("open error");
                        f.write(&self.content.as_bytes());
                        self.content = String::from("");
                    }
                    
                })
            });
            
            ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&mut self.content));
            
        });
    }
}

// impl Default for TextEditor{
//     fn default() -> Self {
//         Self{
//             content : String::from(""),
//             saveKey : String::from("CMDORCTRL+S"),
//             BindingKeyManager : HotkeyManager::new(),
            
//         }
//     }
// }
impl TextEditor{
    pub fn register_bindingKey(& mut self){
        let key_mng = &mut self.BindingKeyManager;
        
        let rec_clone = Arc::clone(&self.rec);
        if let Err(err) = key_mng.register(parse_hotkey(self.saveKey.as_str()).unwrap(),  move|| {
            loop{
                match rec_clone.lock().unwrap().recv()  {
                    Ok(content) => {
                        save(&content);
                        // std::process::exit(0);
                        // break;
                        },
                    Err(_) => {
                        println!("213");
                        std::process::exit(0);
                        break;
                }
                }
                
            }
           
            
        }) {
            panic!("{:?}", err)
        }
    }
   
}
fn save(content : &String){
    use std::fs::File;
    let mut f = File::create("foo.txt").expect("open error");
    f.write(content.as_bytes());
}
use eframe::egui::{self, FontDefinitions, FontFamily,TextStyle};
use std::borrow::Borrow;
use std::collections::BTreeMap;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
pub fn install_fonts(egui_ctx: &egui::CtxRef) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        "LXGWWenKai-Regular".to_owned(),
        std::borrow::Cow::Borrowed(include_bytes!("../res/LXGWWenKai-Regular.ttf")),
    );
    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());
    fonts
        .fonts_for_family
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, "LXGWWenKai-Regular".to_owned());
    //上面是load中文字体，而下面是设置每个区域对应的字体大小。
    let mut family_and_size = BTreeMap::new();
    family_and_size.insert(TextStyle::Small, (FontFamily::Proportional, 18.0));
    family_and_size.insert(TextStyle::Body, (FontFamily::Proportional, 20.0));
    family_and_size.insert(TextStyle::Button, (FontFamily::Proportional, 20.0));
    family_and_size.insert(TextStyle::Heading, (FontFamily::Proportional, 28.0));
    family_and_size.insert(TextStyle::Monospace, (FontFamily::Monospace, 18.0));
    fonts.family_and_size = family_and_size;

    egui_ctx.set_fonts(fonts);
}

#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn text_editor_test(){
        // let app = TextEditor::default();
        // let native_options = eframe::NativeOptions::default();
        // eframe::run_native(Box::new(app), native_options);
    }
}