use std::{borrow::BorrowMut, iter, rc::Rc};
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
        }else{
            println!("the input order is illegal");
        }

    }
    
}