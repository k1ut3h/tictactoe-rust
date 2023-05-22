use std::io;

#[derive(Debug)]
struct Board {
    state: Vec<Vec<Option<char>>>,
}

impl Board {
    fn new() -> Board {
        let board = vec![vec![None; 3]; 3];
        Board { state: board }
    }
    fn render(&self) {
        println!("   -------");
        for (i, row) in self.state.iter().enumerate() {
            print!("{} ", i);
            print!("| ");
            for elem in row {
                if let Some(val) = elem {
                    print!("{} ", val);
                } else {
                    print!("  ");
                }
            }
            print!("|");
            println!();
        }
        println!("   -------");
        println!("    0 1 2");
    }
    fn make_move(&mut self, coords: Vec<String>, turn: char){
        let x = coords[0].parse::<usize>().unwrap();
        let y = coords[1].parse::<usize>().unwrap();
        if x<=2 && y<=2{
            self.state[y][x] = Some(turn);
        }
    }

    fn has_won(&self)->bool{
        return false;
    }

    fn is_full(&self)->bool{
        for i in 0..self.state.len(){
            for j in 0..self.state[i].len(){
                if self.state[i][j]==None{
                    return false;
                }
            }
        }
        return true;
    }

    fn get_all_line_coords(&self){
        let mut cols = vec![];
        for x in 0..3{
            let mut col = vec![];
            for y in 0..3{
                col.push((x,y));
                cols.push(col);
            }
        }
        let mut rows = vec![];
        for x in 0..3{
            let mut row = vec![];
            for y in 0..3{
                row.push((x,y));
                rows.push(row);
            }
        }
        let mut diagonals = vec![
            vec![(0,0), (1,1), (2,2)],
            vec![(0,2), (1,1), (2,0)],
        ];
    }
}

fn main() {
    let mut board = Board::new();
    board.render();
    let mut i = 0;
    loop{
        let turn;
        if i%2==0{
            turn = 'X'; 
        } else {
            turn = 'O';
        }
        i+=1;
        println!("{}'s turn", turn);
        let coords = get_coords();
        if coords[0]=="exit"{
            break;
        }
        if coords.len()==2{
            board.make_move(coords, turn);
            board.render();
            if board.has_won(){
                println!("Congratulations {}, for the big win", turn);
                break;
            }
            if board.is_full(){
                println!("Board is full. Nobody won. Try again.");
                break;
            }
        } else {
            println!("Enter valid coordinates next time");
        }
    }
}

fn get_coords()->Vec<String>{
    let mut input = String::new();
    println!("Enter valid coordinates:");
    io::stdin().read_line(&mut input).unwrap();
    input
        .split(',')
        .map(|val| val.trim().to_string())
        .collect::<Vec<String>>()
}
