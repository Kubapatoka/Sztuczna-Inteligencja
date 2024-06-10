use std::fs;


#[derive(Clone, Copy, PartialEq)]
struct Field
{
    is_white : bool,
    is_black : bool,
    can_be_white : bool,
    can_be_black : bool,
}

fn possibilities(dane : Vec<i32>, pos : i32, max_pos : i32, descr : Vec<char>) -> Vec<Vec<char>>
{
    let mut new_dane = dane.clone();

    if dane.len() == 0
    {
        let mut new_descr = descr.clone();
        for _ in pos..max_pos
        {
            new_descr.push('.');
        }
        let res = vec![new_descr;1];
        return res;
    }

    let mini : i32 = dane.iter().sum::<i32>() + (dane.len() - 1) as i32;

    let how_many_options = max_pos-pos-mini;

    if how_many_options < 0
    {
        return vec![vec![]];
    }

    let d = new_dane.pop().unwrap();
    let mut res : Vec<Vec<char>> = [].to_vec();

    for i in 0..(how_many_options+1)
    {
        let mut new_descr = descr.clone();
        for _ in 0..i
        {
            new_descr.push('.');
        }

        for _ in 0..d
        {
            new_descr.push('#');
        }

        let mut new_pos = pos+i+d;

        if !new_dane.is_empty()
        {
            new_descr.push('.');
            new_pos += 1;
        }

        let res_from_deeper = possibilities(new_dane.clone(), new_pos, max_pos, new_descr);

        for r in res_from_deeper
        {
            res.push(r);
        }
    }

    res
}

fn is_option_fit_row(row: Vec<Field>, option: &Vec<char>) -> bool
{
    //println!("{}",  option.into_iter().collect::<String>());

    for i in 0..row.len()
    {
        let f = row[i];
        let s = option[i];

        if f.is_black && s == '.'
        {
            return false;
        }

        if f.is_white && s == '#'
        {
            return false;
        }
    }

    return true;
}

fn fit_row(mut row: Vec<Field>, options: Vec<Vec<char>>) -> Option<Vec<Field>>
{
    let mut valid_options = options.clone();
    valid_options = valid_options.iter()
                                .filter(|p| is_option_fit_row(row.clone(), p) )
                                .map(|x| x.clone())
                                .collect::<Vec<Vec<char>>>();

    if valid_options.is_empty()
    {
        return None;
    }

    for i in 0..row.len()
    {
        if !row[i].is_white && !row[i].is_black
        {
            for v in valid_options.clone()
            {
                if v[i] == '.'
                {
                    row[i].can_be_white = true;
                }

                if v[i] == '#'
                {
                    row[i].can_be_black = true;
                }
            }

            if row[i].can_be_black && !row[i].can_be_white
            {
                row[i].is_black = true;
            }

            if !row[i].can_be_black && row[i].can_be_white
            {
                row[i].is_white = true;
            }

            row[i].can_be_white = false;
            row[i].can_be_black = false;
        }
    }

    Some(row)
}

fn fit(mut board: Vec<Vec<Field>>, rows : Vec<Vec<Vec<char>>>, cols : Vec<Vec<Vec<char>>>) -> Option<Vec<Vec<Field>>>
{
    let h = board.len();
    let w = board[0].len();

    // println!("before first loop in fit");
    for i in 0..h
    {
        let res = fit_row(board[i].clone(), rows[i].clone());

        if res.is_none()
        {
            // print!("wiersz :{}\n", i);
            return None;
        }
        board[i] = res.unwrap();
    }

    // println!("after first loop in fit");


    for j in 0..w
    {
        let mut col = [].to_vec();
        for i in 0..h
        {
            col.push(board[i][j]);
        }

        let res = fit_row(col, cols[j].clone());
        if res.is_none()
        {
            // print!("kolumna :{}\n", j);
            return None;
        }
        col = res.unwrap();

        for i in 0..h
        {
            board[i][j] = col[i];
        }
    }

    Some(board)
}



fn main() 
{
    let input_file_path = "/home/jakub/Desktop/SI/nonograms_v2/nonogram_solver/zad_input.txt";
    let output_file_path = "/home/jakub/Desktop/SI/nonograms_v2/nonogram_solver/zad_output.txt";
    
    let content = fs::read_to_string(input_file_path)
        .expect("I am not able to read this file").split('\n')
        .map(|s| s.split(' ')
                        .map(|x| x.parse::<i32>().unwrap_or(-1))
                        .collect::<Vec<i32>>())
        .collect::<Vec<Vec<i32>>>();

    let h = content[0][0];
    let w = content[0][1];

    // for i in 1..h+1
    // {
    //     for j in content[i as usize].clone()
    //     {
    //         print!("{} ", j);
    //     }
    //     println!("");
    // }

    // for i in h+1..h+w+1
    // {
    //     for j in content[i as usize].clone()
    //     {
    //         print!("{} ", j);
    //     }
    //     println!("");
    // }


    let mut rows = [].to_vec();
    let mut cols = [].to_vec();

    for i in 1..(h+1)
    {
        let mut r = content[i as usize].clone();
        r.reverse();
        rows.push(possibilities(r, 0, w, vec![]));

        // println!("Rows {}", i-1);
        // let s = rows[(i-1) as usize].clone();
        // for ssi in s
        // {
        //     println!("{}", ssi.into_iter().collect::<String>());
        // }

    }

    for i in (h+1)..(h+w+1)
    {
        let mut r = content[i as usize].clone();
        r.reverse();
        cols.push(possibilities(r, 0, h, vec![]));

        // println!("Cols {}", i-h-1);
        // let s = cols[(i-h-1) as usize].clone();
        // for ssi in s
        // {
        //     println!("{}", ssi.into_iter().collect::<String>());
        // }
    }

    let mut board = vec![vec![Field{is_white: false,
                                                    is_black: false,
                                                    can_be_black: false,
                                                    can_be_white: false};w as usize];h as usize];

    let mut is_the_same = false;
    while !is_the_same
    {
        // println!("first loop\n");
        // wypisz(board.clone());
        // println!("\n");
        let old_board = board.clone();

        // println!("before fit");
        board = fit(board, rows.clone(), cols.clone()).unwrap();
        // println!("after fit");

        is_the_same = compare_boards(old_board.clone(), board.clone());
        if is_the_same{ break; }
    }
    // println!("end loop");

    if is_not_finished(board.clone())
    {
        board = bt(board.clone(), rows.clone(), cols.clone()).unwrap();
    }

    let mut res : Vec<char> = vec![];

    for b in  board
    {
        for c in b
        {
            if c.is_black
            {
                res.push('#');
            }
            else if c.is_white
            {
                res.push('.');
            }
            else
            {
                res.push(' ');
            }
        }
        res.push('\n');

    }

    res.pop();

    let _ = fs::write(output_file_path, res.into_iter().collect::<String>());
}

fn bt(board: Vec<Vec<Field>>, rows: Vec<Vec<Vec<char>>>, cols: Vec<Vec<Vec<char>>>) -> Option<Vec<Vec<Field>>> 
{
    // println!("\n");
    // _wypisz(board.clone());

    let h = board.len();
    let w = board[0].len();

    let mut is_row = true;
    let mut mozliwosci_l = 1000000000;
    let mut mozliwosci = vec![];
    let mut rzad = 0;

    for i in 0..h
    {
        let mut valid_options = rows[i].clone();
        valid_options = valid_options.iter()
                                    .filter(|p| is_option_fit_row(board[i].clone(), p) )
                                    .map(|x| x.clone())
                                    .collect::<Vec<Vec<char>>>();

        let len_val_opt = valid_options.len();

        if len_val_opt > 1 && len_val_opt < mozliwosci_l
        //if  len_val_opt < mozliwosci_l
        {
            mozliwosci_l = len_val_opt;
            rzad = i;
            mozliwosci = valid_options.clone();
        }
    }

    for j in 0..w
    {
        let mut col = [].to_vec();
        for i in 0..h
        {
            col.push(board[i][j]);
        }

        let mut valid_options = cols[j].clone();
        valid_options = valid_options.iter()
                                    .filter(|p| is_option_fit_row(col.clone(), p) )
                                    .map(|x| x.clone())
                                    .collect::<Vec<Vec<char>>>();

        let len_val_opt = valid_options.len();

        if len_val_opt > 1 && len_val_opt < mozliwosci_l
        //if  len_val_opt < mozliwosci_l
        {
            mozliwosci_l = len_val_opt;
            rzad = j;
            mozliwosci = valid_options.clone();
            is_row = false;
        }
    }

    // println!("\nliczba mozliwosci: {}, wiersz:{}, nr: {}", mozliwosci_l, is_row, rzad);

    for i in 0..mozliwosci_l
    {
        let mut nb = board.clone();
        let mo = mozliwosci[i].clone();
        if is_row
        {
            for j in 0..w
            {
                if mo[j] == '#'
                {
                    nb[rzad][j].is_black = true;
                }
                else
                {
                    nb[rzad][j].is_white = true;
                }
            }
        }
        else
        {
            for j in 0..h
            {
                if mo[j] == '#'
                {
                    nb[j][rzad].is_black = true;
                }
                else
                {
                    nb[j][rzad].is_white = true;
                }
            }
        }

        let mut is_the_same = false;
        let mut next_var = false;
        while !is_the_same
        {
            let old_board = nb.clone();
    
            let res = fit(nb.clone(), rows.clone(), cols.clone());

            if res.is_none()
            {
                next_var = true;
                break;
            }
            else 
            {
                nb = res.unwrap();
            }

            is_the_same = compare_boards(old_board.clone(), nb.clone());
            if is_the_same{ break; }
        }

        if next_var
        {
            continue;
        }

        if is_not_finished(nb.clone())
        {
            let res = bt(nb.clone(), rows.clone(), cols.clone());

            if res.is_some()
            {
                return res;
            }
        }
        else 
        {
            return Some(nb);
        }
    }
    None
}

fn is_not_finished(board: Vec<Vec<Field>>) -> bool {
    let h = board.len();
    let w = board[0].len();

    for i in 0..h
    {
        for j in 0..w
        {
            if board[i][j].is_black == false && board[i][j].is_white == false
            {
                return true;
            }
        }
    }
    false
}

fn compare_boards(old_board: Vec<Vec<Field>>, board: Vec<Vec<Field>>) -> bool 
{
    let h = board.len();
    let w = board[0].len();

    for i in 0..h
    {
        for j in 0..w
        {
            if old_board[i][j] != board[i][j]
            {
                return false;
            }
        }
    }

    true
}


fn _wypisz(board: Vec<Vec<Field>>)
{
    let mut res : Vec<char> = vec![];

    for b in  board
    {
        for c in b
        {
            if c.is_black
            {
                res.push('#');
            }
            else if c.is_white
            {
                res.push('.');
            }
            else
            {
                res.push('x');
            }
        }
        res.push('\n');

    }

    res.pop();

    print!("{}",res.into_iter().collect::<String>());
}