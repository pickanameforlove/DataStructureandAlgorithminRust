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
pub fn sokoban_solve(board: & mut Vec<Vec<i32>>,i : i32,j : i32,value : i32,delta : usize,statelist : & mut Vec<Vec<Position>>){
    // if judge_dead(board) {
    //     return;
    // }
    let lenx = board.len() as i32;
    let leny = board[0].len() as i32;
    // println!("start");
    // for i in 0..lenx{
    //     for j in 0..leny{
    //         print!("{}\t",board[i as usize][j as usize]);
    //     }
    //     println!();
    // }
    // println!();
    if judge_success(board) {
        return;
    }
    
    
    let deltax = vec![0,1,0,-1];
    let deltay = vec![1,0,-1,0];
    
    let movement = vec![String::from("R"),String::from("D"),String::from("L"),String::from("U")];
    let mut index = delta;
    let mut loops = 0;
    while loops < 4{
        let delta_x = deltax[index];
        let delta_y = deltay[index];
        let temp_x = i + delta_x;
        let temp_y = j + delta_y;
        if temp_x<lenx && temp_y < leny && 0 <= temp_x&& 0 <= temp_y{
            match board[temp_x as usize][temp_y as usize] {
                0 => {
                    println!("{},{}",movement[index],board[i as usize][j as usize]);
                    // for i in 0..lenx{
                    //     for j in 0..leny{
                    //         print!("{}\t",board[i as usize][j as usize]);
                    //     }
                    //     println!();
                    // }
                    // println!();
                    
                    if value == 3{
                        board[i as usize][j as usize] = 0;
                    }else if value == 5{
                        board[i as usize][j as usize] = 2;
                    }
                    board[temp_x as usize][temp_y as usize] = 3;

                    

                    let state_length = statelist.len();
                    let mut newstate = (*statelist.get(state_length - 1).unwrap()).clone();
                    newstate[0].x += delta_x;
                    newstate[0].y += delta_y;
                    for index in 0..state_length{
                        let com_state = statelist.get(index).unwrap();
                        if judge_vector(com_state,  &newstate){
                            board[temp_x as usize][temp_y as usize] = 0;
                            board[i as usize][j as usize] = value;
                            return;
                        }
                    }
                    statelist.push(newstate);
                    

                    sokoban_solve(board, temp_x, temp_y, 3,index,statelist);
                    if judge_success(board) {
                        return;
                    }

                    statelist.pop();
                    board[temp_x as usize][temp_y as usize] = 0;
                    board[i as usize][j as usize] = value;
                },
                1 => {
                    let forward_second_x = temp_x + delta_x;
                    let forward_second_y = temp_y+delta_y;
                    let flag = board[forward_second_x as usize][forward_second_y as usize];
                    if flag == 0 {
                        // println!("{}",movement[index]);
                        if value == 3{
                            board[i as usize][j as usize] = 0;
                        }else if value == 5{
                            board[i as usize][j as usize] = 2;
                        }
                        board[temp_x as usize][temp_y as usize] = 3;
                        board[forward_second_x as usize][forward_second_y as usize] = 1;

                        println!("{},{}",movement[index],board[i as usize][j as usize]);
                        // for i in 0..lenx{
                        //     for j in 0..leny{
                        //         print!("{}\t",board[i as usize][j as usize]);
                        //     }
                        //     println!();
                        // }
                        // println!();

                        let state_length = statelist.len();
                        let mut newstate = (*statelist.get(state_length - 1).unwrap()).clone();
                        newstate[0].x += delta_x;
                        newstate[0].y += delta_y;
                        let position_length = newstate.len();
                        for i in 0..position_length{
                            let pos = newstate[i].clone();
                            if pos.x==temp_x && pos.y == temp_y{
                                newstate[i].x += delta_x;
                                newstate[i].y += delta_y;
                                break;
                            }
                        }

                        for index in 0..state_length{
                            let com_state = statelist.get(index).unwrap();
                            if judge_vector(com_state,  &newstate){
                                board[temp_x as usize][temp_y as usize] = 1;
                                board[forward_second_x as usize][forward_second_y as usize] = 0;
                                board[i as usize][j as usize] = value;
                                return;
                            }
                        }
                        statelist.push(newstate);

                        sokoban_solve(board, temp_x, temp_y, 3,index,statelist);
                        if judge_success(board) {
                            return;
                        }

                        statelist.pop();

                        board[temp_x as usize][temp_y as usize] = 1;
                        board[forward_second_x as usize][forward_second_y as usize] = 0;
                        board[i as usize][j as usize] = value;

                    }else if  flag == 2 {
                        if judge_success(board) {
                            return;
                        }

                        // println!("{}",movement[index]);
                        if value == 3{
                            board[i as usize][j as usize] = 0;
                        }else if value == 5{
                            board[i as usize][j as usize] = 2;
                        }
                        board[temp_x as usize][temp_y as usize] = 3;
                        board[forward_second_x as usize][forward_second_y as usize] = 4;

                        println!("{},{}",movement[index],board[i as usize][j as usize]);
                        // for i in 0..lenx{
                        //     for j in 0..leny{
                        //         print!("{}\t",board[i as usize][j as usize]);
                        //     }
                        //     println!();
                        // }
                        // println!();


                        let state_length = statelist.len();
                        let mut newstate = (*statelist.get(state_length - 1).unwrap()).clone();
                        newstate[0].x += delta_x;
                        newstate[0].y += delta_y;
                        let position_length = newstate.len();
                        for i in 0..position_length{
                            let pos = newstate[i].clone();
                            if pos.x==temp_x && pos.y == temp_y{
                                newstate[i].x += delta_x;
                                newstate[i].y += delta_y;
                                break;
                            }
                        }

                        for index in 0..state_length{
                            let com_state = statelist.get(index).unwrap();
                            if judge_vector(com_state,  &newstate){
                                board[temp_x as usize][temp_y as usize] = 1;
                                board[forward_second_x as usize][forward_second_y as usize] = 2;
                                board[i as usize][j as usize] = value;
                                return;
                            }
                        }
                        statelist.push(newstate);

                        
                        sokoban_solve(board, temp_x, temp_y, 3,index,statelist);
                        if judge_success(board) {
                            return;
                        }
                        statelist.pop();
                        board[temp_x as usize][temp_y as usize] = 1;
                        board[forward_second_x as usize][forward_second_y as usize] = 2;
                        board[i as usize][j as usize] = value;
                        
                    }
                },
                -1 => {
                    loops += 1;
                    index += 1;
                    index = index % 4;
                    continue},
                2 => {
                    // println!("{}",movement[index]);
                    if value == 3{
                        board[i as usize][j as usize] = 0;
                    }else if value == 5{
                        board[i as usize][j as usize] = 2;
                    }
                    board[temp_x as usize][temp_y as usize] = 5;

                    println!("{},{}",movement[index],board[i as usize][j as usize]);
                    // for i in 0..lenx{
                    //     for j in 0..leny{
                    //         print!("{}\t",board[i as usize][j as usize]);
                    //     }
                    //     println!();
                    // }
                    // println!();

                    let state_length = statelist.len();
                    let mut newstate = (*statelist.get(state_length - 1).unwrap()).clone();
                    newstate[0].x += delta_x;
                    newstate[0].y += delta_y;

                    for index in 0..state_length{
                        let com_state = statelist.get(index).unwrap();
                        if judge_vector(com_state,  &newstate){
                            board[i as usize][j as usize] = value;
                            board[temp_x as usize][temp_y as usize] = 2;
                            return;
                        }
                    }
                    statelist.push(newstate);

                    sokoban_solve(board, temp_x, temp_y, 5,index,statelist);
                    if judge_success(board) {
                        return;
                    }
                    statelist.pop();
                    board[i as usize][j as usize] = value;
                    board[temp_x as usize][temp_y as usize] = 2;
                },
                4 =>{
                    let forward_second_x = temp_x + delta_x;
                    let forward_second_y = temp_y + delta_y;
                    let flag = board[forward_second_x as usize][forward_second_y as usize];
                    if flag == 0{
                        // println!("{}",movement[index]);
                        if value == 3{
                            board[i as usize][j as usize] = 0;
                        }else if value == 5{
                            board[i as usize][j as usize] = 2;
                        }
                        board[temp_x as usize][temp_y as usize] = 5;
                        board[forward_second_x as usize][forward_second_y as usize] = 1;

                        println!("{},{}",movement[index],board[i as usize][j as usize]);
                        // for i in 0..lenx{
                        //     for j in 0..leny{
                        //         print!("{}\t",board[i as usize][j as usize]);
                        //     }
                        //     println!();
                        // }
                        // println!();


                        let state_length = statelist.len();
                        let mut newstate = (*statelist.get(state_length - 1).unwrap()).clone();
                        newstate[0].x += delta_x;
                        newstate[0].y += delta_y;
                        let position_length = newstate.len();
                        for i in 0..position_length{
                            let pos = newstate[i].clone();
                            if pos.x==temp_x && pos.y == temp_y{
                                newstate[i].x += delta_x;
                                newstate[i].y += delta_y;
                                break;
                            }
                        }

                        for index in 0..state_length{
                            let com_state = statelist.get(index).unwrap();
                            if judge_vector(com_state,  &newstate){
                                board[temp_x as usize][temp_y as usize] = 4;
                                board[forward_second_x as usize][forward_second_y as usize] = 0;
                                board[i as usize][j as usize] = value;
                                return;
                            }
                        }
                        statelist.push(newstate);

                        sokoban_solve(board, temp_x, temp_y, 5,index,statelist);
                        if judge_success(board) {
                            return;
                        }
                        statelist.pop();
                        board[temp_x as usize][temp_y as usize] = 4;
                        board[forward_second_x as usize][forward_second_y as usize] = 0;
                        board[i as usize][j as usize] = value;
                    }else if flag == 2 {
                        // println!("{}",movement[index]);
                        if value == 3{
                            board[i as usize][j as usize] = 0;
                        }else if value == 5{
                            board[i as usize][j as usize] = 2;
                        }
                        board[temp_x as usize][temp_y as usize] = 5;
                        board[forward_second_x as usize][forward_second_y as usize] = 4;

                        println!("{},{}",movement[index],board[i as usize][j as usize]);
                        // for i in 0..lenx{
                        //     for j in 0..leny{
                        //         print!("{}\t",board[i as usize][j as usize]);
                        //     }
                        //     println!();
                        // }
                        // println!();

                        let state_length = statelist.len();
                        let mut newstate = (*statelist.get(state_length - 1).unwrap()).clone();
                        newstate[0].x += delta_x;
                        newstate[0].y += delta_y;
                        let position_length = newstate.len();
                        for i in 0..position_length{
                            let pos = newstate[i].clone();
                            if pos.x==temp_x && pos.y == temp_y{
                                newstate[i].x += delta_x;
                                newstate[i].y += delta_y;
                                break;
                            }
                        }

                        for index in 0..state_length{
                            let com_state = statelist.get(index).unwrap();
                            if judge_vector(com_state,  &newstate){
                                board[temp_x as usize][temp_y as usize] = 4;
                                board[forward_second_x as usize][forward_second_y as usize] = 2;
                                board[i as usize][j as usize] = value;
                                return;
                            }
                        }
                        statelist.push(newstate);

                        sokoban_solve(board, temp_x, temp_y, 5,index,statelist);
                        if judge_success(board) {
                            return;
                        }
                        statelist.pop();
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
        loops += 1;
        index += 1;
        index = index % 4;
    }
}
fn judge_success(board: &Vec<Vec<i32>>) -> bool{
    let lenx = board.len() as i32;
    let leny = board[0].len() as i32;
    let mut i_x = 0;
    let mut i_y = 0;
    while i_x < lenx{
        i_y = 0;
        while i_y < leny {
            if board[i_x as usize][i_y as usize] == 1 || board[i_x as usize][i_y as usize] == 2{
                return false;
            }
            i_y += 1;
        }
        i_x += 1;
    }
    return true;

}
fn judge_dead(board: &Vec<Vec<i32>>) -> bool {
    let lenx = board.len() as i32;
    let leny = board[0].len() as i32;
    let mut i_x = 0;
    let mut i_y = 0;
    while i_x < lenx{
        i_y = 0;
        while i_y < leny {
            if board[i_x as usize][i_y as usize] == 1{
               if board[(i_x+1) as usize][i_y as usize] == -1 &&(board[i_x as usize][(i_y+1) as usize] == -1 || board[i_x as usize][(i_y-1) as usize] == -1) {
                   return true;
               }else if board[(i_x-1) as usize][i_y as usize] == -1 &&(board[i_x as usize][(i_y+1) as usize] == -1 || board[i_x as usize][(i_y-1) as usize] == -1){
                   return true;
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