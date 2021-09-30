fn main() {
    let mut list = vec![2, 43, 3, 56, 7, 8, 9, 65, 10, 11, 12, 21];
    
    let length = list.len() - 1;
    let mut list2 = merge_sort(&mut list,0,length);
    for i in list2{
        println!("{}",i);
    }
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
    let mid = (left +right ) / 2;
    let result = union(&merge_sort(l,left,mid),&merge_sort(l,mid+1,right));
    return result;

}

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