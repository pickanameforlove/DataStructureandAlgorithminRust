mod graph;
mod LinkedList;
mod gameTree;
mod tree;
mod sokoban;

use graph::*;
use LinkedList::*;
use gameTree::getGameTree;
use std::rc::Rc;
use sokoban::Position;

use crate::{sokoban::sokoban_solve, tree::Tree};
fn main() {
    let mut list = vec![2, 43, 3, 56, 7, 8, 9, 65, 10, 11, 12, 21];
    // let length = list.len() - 1;
    // let mut list2 = merge_sort(&mut list,0,length);
    quick_sort(&mut list);
    for i in list{
        println!("{}",i);
    }
    let mut edgelist = vec![Edge::new(1, 2, 7),Edge::new(1, 4, 5),Edge::new(4, 2, 9),Edge::new(3, 2, 8),Edge::new(5, 2, 7),
    Edge::new(3, 5, 5),Edge::new(4, 5, 15),Edge::new(4, 6, 6),Edge::new(5, 6, 8),Edge::new(5, 7, 9),Edge::new(6, 7, 11)];

    kruskal(&mut edgelist, 7);
    let mut linkedlist = LinkedList::LinkedList::new();
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
    let mut board = [[1,0,0],[0,1,0],[0,0,-1]];
    println!("{}\t{}\t{}",board[0][0],board[0][1],board[0][2]);
    println!("{}\t{}\t{}",board[1][0],board[1][1],board[1][2]);
    println!("{}\t{}\t{}",board[2][0],board[2][1],board[2][2]);
    let res = getGameTree(&mut board, false);
    println!("the result is {}",res);
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
    let mut board_2 = vec![
        vec![-1,-1,-1,-1,-1,-1,-1,-1],
        vec![-1,-1,-1,0,0,0,-1,-1],
        vec![-1,2,3,1,0,0,-1,-1],
        vec![-1,-1,-1,0,1,2,-1,-1],
        vec![-1,2,-1,-1,1,0,-1,-1],
        vec![-1,0,-1,0,2,0,-1,-1],
        vec![-1,1,0,4,1,1,2,-1],
        vec![-1,0,0,0,2,0,0,-1],
        vec![-1,-1,-1,-1,-1,-1,-1,-1]];

    // let mut posList = Vec::new();
    // posList.push(Position::new(2,2));
    // posList.push(Position::new(2,3));
    // posList.push(Position::new(3,4));
    // posList.push(Position::new(4,4));
    // posList.push(Position::new(6,1));
    // posList.push(Position::new(6,3));
    // posList.push(Position::new(6,4));
    // posList.push(Position::new(6,5));
    // let mut statelist = Vec::new();
    // statelist.push(posList);
    println!("test");
    let mut board_3 =  vec![
        vec![-1,-1,-1,-1,-1,-1,-1,-1],
        vec![-1,0,0,1,0,0,2,-1],
        vec![-1,0,3,1,0,0,2,-1],
        vec![-1,-1,-1,-1,-1,-1,-1,-1]];
    let mut posList = Vec::new();
    posList.push(Position::new(2,2));
    posList.push(Position::new(1,3));
    posList.push(Position::new(2,4));
  
    let mut statelist = Vec::new();
    statelist.push(posList);
    sokoban_solve(& mut board_3, 2, 2, 3,0,& mut &mut statelist);
    // let mut l = Vec::new();
    // l.push(1);
    // l.push(2);
    // l.push(3);
    // l.pop();
    // println!("{},{}",l[0],l[1]);
}

/// this is bubble sort
fn bubble_sort(l : &mut [u32]){
    let length = l.len();
    let mut i = 0;
    while i < length{
        let mut tmp_index = i;
        let mut edge = l[i];
        let mut j = i + 1;
        while j <  length{
            if l[j] < edge{
                edge = l[j];
                tmp_index = j;
            }
            j += 1;
        }
        //change value between i and tmp_index
        l[tmp_index] = l[i];
        l[i] = edge;
        i += 1;
    }
}

fn merge_sort(l:&[u32],left: usize,right:usize)->Vec<u32>{
    if left == right{
        return vec![l[left]];
    }
    //分
    let mid = (left +right ) / 2;
    let result = union(&merge_sort(l,left,mid),&merge_sort(l,mid+1,right));
    return result;

}
//合
fn union(l:&[u32],r:&[u32])-> Vec<u32>{
    let l1 = l.len();
    let l2 = r.len();
    let mut result = vec![0;l1+l2];
    
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < l1 && j < l2{
        if l[i] < r[j] {
            result[k] = l[i];
            i += 1;
            k += 1;
        }else{
            result[k] = r[j];
            j += 1;
            k += 1;
        }
    }
    while i < l1{
        result[k] = l[i];
        i += 1;
        k += 1;
    }
    while j < l2 {
        result[k] = r[j];
        j += 1;
        k += 1;
    }
    return result;
}

fn quick_sort(l:&mut [u32]){
    let length = l.len();
    if length <= 1{
        return;
    }
    let anchor = l[0];
    let mut low = 0;
    let mut high = l.len()-1;
    let mut direction = true;
    
    while low < high {
        if direction {
            if l[high] <= anchor {
                direction = false;
                //交换完之后不能改变指针。
                //要始终保证low/high指针指向anchor值。
                let mut temp = l[high];
                l[high] = l[low];
                l[low] = temp;
            }else{
                high -= 1;
            }
           
        }else{
            if l[low] > anchor{
                direction = true;
                let mut temp = l[high];
                l[high] = l[low];
                l[low] = temp;
            }else{
                low += 1;
            }
            
        }
    }
    
    quick_sort(&mut ((*l)[0..=high]));
    quick_sort(&mut ((*l)[high+1..length]));
}