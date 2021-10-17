use eframe::epi;
use eframe::egui::containers::*;
use tauri_hotkey::{parse_hotkey, HotkeyManager};
use native_dialog::{FileDialog, MessageDialog, MessageType};
use std::fs;
pub struct TextEditor {
    pub content : String,
    pub content_copy : String,
    pub saveKey : String,
    pub BindingKeyManager : HotkeyManager,
    pub rec : Receiver<String>,
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

        match self.rec.try_recv(){
            Ok(_) => self.save(),
            Err(_) => println!("123")
        };
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
                    if ui.button("open").clicked(){
                        let path = FileDialog::new()
                            .set_location("C:")
                            .add_filter("txt file", &["txt"])
                            .show_open_single_file()
                            .unwrap();
                        let path = match path {
                            Some(path) => path,
                            None => return,
                        };

                        let yes = MessageDialog::new()
                            .set_type(MessageType::Info)
                            .set_title("Do you want to open the file?")
                            .set_text(&format!("{:#?}", path))
                            .show_confirm()
                            .unwrap();

                        if yes {
                            // do_something(path);
                            self.open(path.to_str().unwrap());
                        }
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
        
        let send_clone = self.send.clone();
        if let Err(err) = key_mng.register(parse_hotkey(self.saveKey.as_str()).unwrap(),  move|| {
            //这里相当于一个触发器
            send_clone.send(String::from("yes")).unwrap();
        }) {
            panic!("{:?}", err);
        }
    }
    pub fn save(&mut self){
        use std::fs::File;
        let mut f = File::create("foo.txt").expect("open error");
        f.write(self.content.as_bytes());
        std::process::exit(0);
    }
    pub fn open(&mut self,path : &str){
        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the file");

        self.content = contents;
    }
   
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