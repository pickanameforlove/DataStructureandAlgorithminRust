#[derive(Clone)]
pub struct AVLTree{
    data: i32,
    bf : i32,
    left:Option<Box<AVLTree>>,
    right: Option<Box<AVLTree>>
}
impl AVLTree {
    pub fn new(d:i32)-> Self {
        Self{
            data: d,
            bf : 0,
            left : None,
            right : None
        }
    }
    fn height(&self) -> i32{
        let node_clone = self.clone();
        if node_clone.right.is_none() && node_clone.left.is_none(){
            return 1;
        }
        if node_clone.right.is_none(){
            return node_clone.left.unwrap().height()+ 1;
        }else if node_clone.left.is_none() {
            return node_clone.right.unwrap().height()+ 1;
        }else{
            return std::cmp::max(node_clone.left.unwrap().height(), node_clone.right.unwrap().height())+1;
        }
        
    }
    fn setBalancedFactor(&mut self){
        let node_clone = self.clone();
        let lh = node_clone.left.unwrap().height();
        let rh = node_clone.right.unwrap().height();
        self.bf = lh - rh;
    }
    fn insertNode(&mut self,data : i32){
        let mut temp_node = self.clone();

        while true {
            let data_node = temp_node.data;
            if data_node < data && temp_node.right.is_some(){
                temp_node = *temp_node.right.unwrap();
            }else if data_node < data && temp_node.right.is_none() {
                temp_node.right = Some(Box::new(AVLTree::new(data)));
                break;
            }else if data_node >= data && temp_node.left.is_some() {
                temp_node = *temp_node.left.unwrap();
            }else{
                temp_node.left = Some(Box::new(AVLTree::new(data)));
                break;
            }
        }
    }

}

fn R_Rotate(node : &mut AVLTree){
    //不要嫌rust中需要自己创建变量的clone，rust的这个设置让你更加掌握内存的使用。
    //其他的语言自动实现的内存的copy！
    let mut node_clone = node.clone();
    let mut lc = node_clone.left.unwrap();

   //如果lc是引用的话，就必须clone一个变量，如果lc不是引用的话，可以直接使用。
    node.left = lc.right;
    node.setBalancedFactor();

    lc.right = Some(Box::new(node.clone()));
    *node = *lc;
    node.setBalancedFactor();
    

}
fn L_Rotate(node : & mut AVLTree){
    let node_clone = node.clone();
    let mut rc = node_clone.right.unwrap();

    node.right = rc.left;
    node.setBalancedFactor();

    rc.left = Some(Box::new(node.clone()));
    *node = *rc;
    node.setBalancedFactor();
}

