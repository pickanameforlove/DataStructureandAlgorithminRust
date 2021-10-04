///该函数的作用是给定一个井字棋棋局，输出一个评分。
///评分很简单，根据gametree的演变，这个棋局如果能获胜就是1，如果平局就是0，如果失败了就是-1.
///评分是从第一个下的棋手出发的。
///因此输入的board上面的棋子数必为奇数，因为如果为偶数的话，则是对手应了一手的情况，这涉及到对手会不会选择的问题。
///board数组上只会出现3个值，0，-1，1，0代表空位置，1代表我方下的位置，-1代表对手下的位置。
pub fn getGameTree(board: & mut [[i32;3];3],flag:bool)->i32{
    
    let mut index_x = 0;
    let mut index_y = 0;
    let mut vecx = Vec::new();
    let mut vecy = Vec::new();
    while index_x < 3{
        index_y = 0;
        while index_y < 3{
            if board[index_x][index_y] == 0{
                vecx.push(index_x);
                vecy.push(index_y);
            }
            index_y += 1;
        }
        index_x += 1;
    }
    let length = vecx.len();
    let mid_value = getresult(board);
    if mid_value != 0{
        return mid_value;
    }else if  length == 0{
        return mid_value;
    }
    let mut index = 0;
    if flag{
        let mut max = -10;
        // let mut temp_index = 0;
        while index < length{
            let x = *vecx.get(index).unwrap();
            let y = *vecy.get(index).unwrap();
            board[x][y] = 1;
            // println!("{}\t{}\t{}",board[0][0],board[0][1],board[0][2]);
            // println!("{}\t{}\t{}",board[1][0],board[1][1],board[1][2]);
            // println!("{}\t{}\t{}",board[2][0],board[2][1],board[2][2]);
            let temp_res = getGameTree(board, !flag);
            if max < temp_res{
                max  = temp_res;
                // temp_index = index;
            }
            board[x][y] = 0;
            index += 1;
        }
        return max;
    }else{
        let mut min = 5;
        while index < length{
            let x = *vecx.get(index).unwrap();
            let y = *vecy.get(index).unwrap();
            board[x][y] = -1;
            // println!("{}\t{}\t{}",board[0][0],board[0][1],board[0][2]);
            // println!("{}\t{}\t{}",board[1][0],board[1][1],board[1][2]);
            // println!("{}\t{}\t{}",board[2][0],board[2][1],board[2][2]);
            let temp_res = getGameTree(board, !flag);
            if min > temp_res{
                min  = temp_res;
            }
            board[x][y] = 0;
            index += 1;
        }
        return min;
        
    }

}
fn getresult(board: & [[i32;3];3])->i32{
    if board[0][0]==board[0][1]&&board[0][1]==board[0][2]{
        return board[0][0];
    }else if  board[1][0]==board[1][1]&&board[1][1]==board[1][2]{
        return board[1][1];
    }else if board[2][0]==board[2][1]&&board[2][1]==board[2][2] {
        return board[2][2];
    }else if  board[0][0]==board[1][0]&&board[1][0]==board[2][0]{
        return board[0][0];
    }else if  board[0][1]==board[1][1]&&board[1][1]==board[2][1]{
        return board[1][2];
    }else if  board[0][2]==board[1][2]&&board[1][2]==board[2][2]{
        return board[2][2];
    }else if board[0][0]==board[1][1] && board[1][1]==board[2][2]{
        return board[1][1];
    }else if board[0][2]==board[1][1] && board[1][1]==board[2][0]{
        return board[1][1];
    }else{
        return 0;
    }
}