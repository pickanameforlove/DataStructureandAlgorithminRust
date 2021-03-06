
#[derive(Clone)]
struct Node{
    value : i32,
    next : Option<Box<Node>>
}

impl Node{
    fn new(v : i32)->Self{
        Self{
            value : v,
            next : None,
        }
    }
    fn setNext(&mut self,value: i32){
        self.next = Some(Box::new(Node::new(value)));
    }
}
pub struct LinkedList{
    head : Option<Box<Node>>,
    length : i32
}

impl LinkedList{
    pub fn new()->Self{
        Self{
            head : None,
            length : 0,
        }
    }
    pub fn display(&mut self){
        let mut iter_node = &self.head;
        while let Some(p) = iter_node {
            println!("the value in the node is {}",p.value);
            iter_node = &p.next;
        }
        println!("--------------------");
        
    }
    pub fn push(&mut self,value : i32){
        let mut iter_node =  &mut self.head;
        while let Some(p) = iter_node{
            iter_node = &mut p.next;
        }
        //https://stackoverflow.com/questions/66084826/cannot-use-last-node-because-it-was-mutably-borrowed
        //该网址就是自己遇到的问题。
        //问题就是一个引用能不能继续被借用值。
        *iter_node = Some(Box::new(Node::new(value)));
        self.length += 1;
        //版本二
        // while let Some(p) = iter_node{
        //     if p.next.is_none(){
        //         break;
        //     }
        //     iter_node = &mut p.next;
        // }
        // iter_node.unwrap().setNext(value);

    }
    ///order为0时，代表head与第一个节点之间插入
    pub fn insert(&mut self, value : i32,order:i32){
        if order < 0 {
            println!("the input order is illegal");
        }
        let mut iter_node = &mut self.head;
        let mut index = 0;
        while index < order{
            match iter_node{
                Some(p) => {
                    iter_node = & mut p.next;
                    index += 1},
                None => break,
            }
            
        }
        println!("{},{}",index,order);
        if index == order{
            let iter_node_son = (*iter_node).clone();
            let mut node = Node::new(value);
            node.next = iter_node_son;
            *iter_node = Some(Box::new(node));
            self.length += 1
        }else{
            println!("the input order is illegal");
        }

    }
    pub fn pop(&mut self)->i32{
        let mut iter_node =  &mut self.head;
        let mut index = 0;
        //版本一，失败
        // while let Some(p) = iter_node{
        //     if index == self.length - 1{
        //         index += 1;
        //         iter_node = &mut p.next;
        //     }
        // }
        // let res = iter_node.clone();
        //版本二
        while  index < self.length-1 {
            match iter_node {
                Some(p) => {
                    iter_node = & mut p.next;
                    index += 1},
                None => break,
            };
        }
        //此处会发生iter_node的借用。
        let res = (*iter_node).clone();

        *iter_node  = None;
        self.length -= 1;
        res.unwrap().value
    }
    
    pub fn delete(&mut self, value : i32){
        let mut iter_node =  &mut self.head;
        let mut res = None;
        let mut flag = false;

        loop {
            match iter_node {
                Some(p) => {
                    if p.value == value {
                        res = p.next.clone();
                        flag = true;
                        break;
                    }
                    iter_node = &mut iter_node.as_mut().unwrap().next;
                    // iter_node = &mut (iter_node.unwrap().next);
                    },
                None => break,
            };
        }
        
        if flag{
            *iter_node = res;
            self.length -= 1;
        }else{
            println!("there is not the value to be deleted!");
        }
    }
 
    ///position代表更新的节点的位置，当position为0时代表第一个节点。
    pub fn update(&mut self, position: i32, value : i32){
        let mut iter_node = &mut self.head;
        if position > self.length - 1 || position < 0{
            println!("the position is illegal!");
        }
        let mut index = 0;
        while index < position{
            match iter_node {
                Some(p) => {
                    iter_node = &mut p.next;
                    index += 1;
                },
                None => break,
            };
        }
        iter_node.as_mut().unwrap().value = value;

    }
}

#[cfg(test)]
mod test{
    use super::LinkedList;
    #[test]
    fn linkedlist_test(){
        let mut linkedlist = LinkedList::new();
        linkedlist.push(1);
        linkedlist.push(2);
        linkedlist.display();
        linkedlist.push(3);
        linkedlist.insert(4, 0);
        linkedlist.display();
        // let v = linkedlist.pop();
        // println!("value is {}",v);
        linkedlist.delete(4);
        linkedlist.display();
        linkedlist.update(0, 5);
        linkedlist.display();
    }
}