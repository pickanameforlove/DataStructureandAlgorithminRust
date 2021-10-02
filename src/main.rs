mod graph;
mod LinkedList;
use graph::*;
use LinkedList::*;
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