#[derive(Clone)]
pub struct Position{
    pub x : i32,
    pub y : i32
}
impl Position {
    pub fn new(x : i32,y : i32)->Self {
        Self{
            x : x,
            y : y,
        }
    }
}
impl PartialEq for Position{
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

//推箱子游戏，棋盘上0代表空地，1代表箱子，-1代表墙，2代表目的地，
//3代表人。4代表箱子在目的地上,5代表人在目的地上。
//遇到的两个问题，一是match中竟然出现了_的情况，原因是变量名i重复了，导致填的值的位置不对。
//二是dfs没有搜索充分，很明显有的情况就没有搜索到，是因为判断重复状态时的return，因为可能此时index还没有到3呢？直接return后面的情况完全没有运行。
//三是判定重复状态的时候，复制粘贴？？？？？
//遗留下问题，在sokoban_solve一开始判定重复状态和在match的各个子arm中判定重复状态有什么不同？
//遗留下问题，为什么最终输出的steplist有所遗漏？原来是terminal的限制。。。
pub fn sokoban_solve(board: & mut Vec<Vec<i32>>,i : i32,j : i32,value : i32,delta : usize,statelist : & mut Vec<Vec<Position>>,stepList : &mut Vec<String>){
    if judge_dead(board) {
        return;
    }
    let lenx = board.len() as i32;
    let leny = board[0].len() as i32;

    let state_length = statelist.len();
    let mut newstate = Vec::new();
    for i in 0..lenx{
        for j in 0..leny{
            if board[i as usize][j as usize]==value{
                newstate.push(Position::new(i, j));
            }
        }
    }
    for i in 0..lenx{
        for j in 0..leny{
            if board[i as usize][j as usize]==1 || board[i as usize][j as usize]==4{
                newstate.push(Position::new(i, j));
            }
        }
    }
    for index in 0..state_length{
        let com_state = statelist.get(index).unwrap();
        if judge_vector(com_state,  &newstate){
            return;
        }
    }
    statelist.push(newstate);

    if judge_success(board) {
        return;
    }
    
    
    let deltax = vec![-1,1,0,0];
    let deltay = vec![0,0,-1,1];
    
    let movement = vec![String::from("U"),String::from("D"),String::from("L"),String::from("R")];
    let mut index = 0;
    while index < 4{
        let delta_x = deltax[index];
        let delta_y = deltay[index];
        let temp_x = i + delta_x;
        let temp_y = j + delta_y;
        if temp_x<lenx && temp_y < leny && 0 <= temp_x&& 0 <= temp_y{
            match board[temp_x as usize][temp_y as usize] {
                0 => {
                    if value == 3{
                        board[i as usize][j as usize] = 0;
                    }else if value == 5{
                        board[i as usize][j as usize] = 2;
                    }
                    board[temp_x as usize][temp_y as usize] = 3;
                    stepList.push(movement[index].clone());

                    sokoban_solve(board, temp_x, temp_y, 3,index,statelist,stepList);
                    if judge_success(board) {
                        return;
                    }
                    stepList.pop();
                    board[temp_x as usize][temp_y as usize] = 0;
                    board[i as usize][j as usize] = value;
                },
                1 => {
                    let forward_second_x = temp_x + delta_x;
                    let forward_second_y = temp_y+delta_y;
                    let flag = board[forward_second_x as usize][forward_second_y as usize];
                    if flag == 0 {
                        if value == 3{
                            board[i as usize][j as usize] = 0;
                        }else if value == 5{
                            board[i as usize][j as usize] = 2;
                        }
                        board[temp_x as usize][temp_y as usize] = 3;
                        board[forward_second_x as usize][forward_second_y as usize] = 1;
                        stepList.push(movement[index].clone());

                        sokoban_solve(board, temp_x, temp_y, 3,index,statelist,stepList);
                        if judge_success(board) {
                            return;
                        }

                        stepList.pop();

                        board[temp_x as usize][temp_y as usize] = 1;
                        board[forward_second_x as usize][forward_second_y as usize] = 0;
                        board[i as usize][j as usize] = value;

                    }else if  flag == 2 {
                        if value == 3{
                            board[i as usize][j as usize] = 0;
                        }else if value == 5{
                            board[i as usize][j as usize] = 2;
                        }
                        board[temp_x as usize][temp_y as usize] = 3;
                        board[forward_second_x as usize][forward_second_y as usize] = 4;

                        stepList.push(movement[index].clone());
                        
                        sokoban_solve(board, temp_x, temp_y, 3,index,statelist,stepList);
                        if judge_success(board) {
                            return;
                        }
                        stepList.pop();
                        board[temp_x as usize][temp_y as usize] = 1;
                        board[forward_second_x as usize][forward_second_y as usize] = 2;
                        board[i as usize][j as usize] = value;
                        
                    }
                },
                -1 => {
                    index += 1;
                    continue},
                2 => {
                    if value == 3{
                        board[i as usize][j as usize] = 0;
                    }else if value == 5{
                        board[i as usize][j as usize] = 2;
                    }
                    board[temp_x as usize][temp_y as usize] = 5;

                    stepList.push(movement[index].clone());
                    sokoban_solve(board, temp_x, temp_y, 5,index,statelist,stepList);
                    if judge_success(board) {
                        return;
                    }
                    stepList.pop();
                    board[i as usize][j as usize] = value;
                    board[temp_x as usize][temp_y as usize] = 2;
                },
                4 =>{
                    let forward_second_x = temp_x + delta_x;
                    let forward_second_y = temp_y + delta_y;
                    let flag = board[forward_second_x as usize][forward_second_y as usize];
                    if flag == 0{
                        if value == 3{
                            board[i as usize][j as usize] = 0;
                        }else if value == 5{
                            board[i as usize][j as usize] = 2;
                        }
                        board[temp_x as usize][temp_y as usize] = 5;
                        board[forward_second_x as usize][forward_second_y as usize] = 1;

                        stepList.push(movement[index].clone());

                        sokoban_solve(board, temp_x, temp_y, 5,index,statelist,stepList);
                        if judge_success(board) {
                            return;
                        }
                        stepList.pop();
                        board[temp_x as usize][temp_y as usize] = 4;
                        board[forward_second_x as usize][forward_second_y as usize] = 0;
                        board[i as usize][j as usize] = value;
                    }else if flag == 2 {
                        if value == 3{
                            board[i as usize][j as usize] = 0;
                        }else if value == 5{
                            board[i as usize][j as usize] = 2;
                        }
                        board[temp_x as usize][temp_y as usize] = 5;
                        board[forward_second_x as usize][forward_second_y as usize] = 4;

                        stepList.push(movement[index].clone());


                        sokoban_solve(board, temp_x, temp_y, 5,index,statelist,stepList);
                        if judge_success(board) {
                            return;
                        }
                        stepList.pop();

                        board[temp_x as usize][temp_y as usize] = 4;
                        board[forward_second_x as usize][forward_second_y as usize] = 2;
                        board[i as usize][j as usize] = value;
                    }
                },
                _ => {
                    println!("this situation is impossible!{}",board[temp_x as usize][temp_y as usize]);
                    for i in 0..lenx{
                        for j in 0..leny{
                            print!("{}\t",board[i as usize][j as usize]);
                        }
                        println!();
                    }
                    println!();
                    
            },

            }
        }
        index += 1;
    }
}
fn judge_success(board: &Vec<Vec<i32>>) -> bool{
    // for i in 0..7{
    //     for j in 0..7{
    //         print!("{}\t",board[i as usize][j as usize]);
    //     }
    //     println!();
    // }
    // println!();
    let lenx = board.len() as i32;
    let leny = board[0].len() as i32;
    let mut i_x = 0;
    let mut i_y = 0;
    while i_x < lenx{
        i_y = 0;
        while i_y < leny {
            if board[i_x as usize][i_y as usize] == 1 || board[i_x as usize][i_y as usize] == 2{
                // println!("{}",0);
                return false;
            }
            i_y += 1;
        }
        i_x += 1;
    }
    // println!("{}",1);
    return true;

}
fn judge_dead(board: &Vec<Vec<i32>>) -> bool {
    let pattern = vec![
        vec![0,1,2,3,4,5,6,7,8],
        vec![2,5,8,1,4,7,0,3,6],
        vec![8,7,6,5,4,3,2,1,0],
        vec![6,3,0,7,4,1,8,5,2],
        vec![2,1,0,5,4,3,8,7,6],
        vec![0,3,6,1,4,7,2,5,8],
        vec![6,7,8,3,4,5,0,1,2],
        vec![8,5,2,7,4,1,6,3,0]
    ];
    let lenx = board.len() as i32;
    let leny = board[0].len() as i32;
    let mut i_x = 1;
    let mut i_y = 1;
    while i_x < lenx{
        i_y = 0;
        while i_y < leny {
            if board[i_x as usize][i_y as usize] == 1{
                let pattern_board_x = vec![i_x-1,i_x-1,i_x-1,i_x,i_x,i_x,i_x+1,i_x+1,i_x+1];
                let pattern_board_y = vec![i_y-1,i_y,i_y+1,i_y-1,i_y,i_y+1,i_y-1,i_y,i_y+1];
                for j_pattern in &pattern{
                let mut new_pattern_board = Vec::new();
                for k in j_pattern{
                    // println!("{:?}",j_pattern);
                    let value = board[pattern_board_x[*k] as usize][pattern_board_y[*k] as usize];
                    new_pattern_board.push(value);
                }
                if new_pattern_board[1]==-1 &&new_pattern_board[5]==-1{
                    return true;
                }else if new_pattern_board[1]==1 &&new_pattern_board[2]==-1 &&new_pattern_board[5]==-1 {
                    return true;
                }else if new_pattern_board[1]==1 &&new_pattern_board[2]==-1 &&new_pattern_board[5]==1 {
                    return true;
                }else if  new_pattern_board[1]==1 &&new_pattern_board[2]==1 &&new_pattern_board[5]==1{
                    return true;
                }else if new_pattern_board[1]==1 &&new_pattern_board[6]==1 &&new_pattern_board[2]==-1&&new_pattern_board[3]==-1 &&new_pattern_board[8]==-1 {
                    return true;
                }

                   
               }
                
            }
            i_y += 1;
        }
        i_x += 1;
    }
    return false;
}
fn judge_vector(v1:&Vec<Position>,v2:&Vec<Position>) -> bool{
    let length = v1.len();
    for i in 0..length{
        if v1[i] != v2[i]{
            return false;
        }
    }
    return true;
}