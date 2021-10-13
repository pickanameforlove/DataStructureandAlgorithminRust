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
#[cfg(test)]
mod tests{
    use std::rc::Rc;
    use super::Tree;
    #[test]
    fn tree_test(){
        let node1 = Rc::new(Tree{
            value : 1,
            left : None,
            right : None
        });
        let node2 = Rc::new(Tree{
            value : 2,
            left : None,
            right : None
        });
        let node3 = Rc::new(Tree{
            value : 3,
            left : Some(Rc::clone(&node1)),
            right : Some(Rc::clone(&node2))
        });
        let node4 = Rc::new(Tree{
            value : 4,
            left : None,
            right : None
        });
        let node5 = Rc::new(Tree{
            value : 5,
            left : Some(Rc::clone(&node4)),
            right : None
        });
        let node6 = Rc::new(Tree{
            value : 6,
            left : Some(Rc::clone(&node3)),
            right : Some(Rc::clone(&node5))
        });
        node6.midtravel();
        println!("");
        node6.pretravel();
        println!("");
        node6.lasttravel();
        println!("");
    }
}