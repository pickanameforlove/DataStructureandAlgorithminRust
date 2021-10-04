use std::{cmp::max, rc::Rc};

pub struct Tree{
    pub value : i32,
    pub left : Option<Rc<Tree>>,
    pub right: Option<Rc<Tree>>
}

impl Tree{
    pub fn height(&self) -> i32{
        max(self.left.as_ref().unwrap().height(),self.right.as_ref().unwrap().height()) + 1
        
    }
    pub fn pretravel(&self){
        print!("{}\t",self.value);
        match self.left.as_ref() {
            Some(p) => p.pretravel(),
            None => print!("")
        }; 
        match self.right.as_ref() {
            Some(p) => p.pretravel(),
            None => print!("")
        }; 
    }

    pub fn midtravel(&self){
        match self.left.as_ref() {
            Some(p) => p.midtravel(),
            None => print!("")
        }; 
        print!("{}\t",self.value);
        match self.right.as_ref() {
            Some(p) => p.midtravel(),
            None => print!("")
        }; 
    }
    pub fn lasttravel(&self){
        match self.left.as_ref() {
            Some(p) => p.lasttravel(),
            None => print!("")
        }; 
        
        match self.right.as_ref() {
            Some(p) => p.lasttravel(),
            None => print!("")
        }; 
        print!("{}\t",self.value);
    }
}