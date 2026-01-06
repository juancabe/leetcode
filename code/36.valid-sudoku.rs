impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {

        const DIM: usize = 9;

        // check rows
        for i in 0..DIM { 
            let mut n_appears = [false; DIM];
            for j in 0..DIM {
                let n = if let Some(d) = board[i][j].to_digit(10) {
                    d
                } else {
                    continue
                };

                if n_appears[(n - 1) as usize] {
                    return false;
                }

                n_appears[(n - 1) as usize] = true;
            }
        }

        // check cols
        for j in 0..DIM { 
            let mut n_appears = [false; DIM];
            for i in 0..DIM {
                let n = if let Some(d) = board[i][j].to_digit(10) {
                    d
                } else {
                    continue
                };

                if n_appears[(n - 1) as usize] {
                    return false;
                }

                n_appears[(n - 1) as usize] = true;
            }
        }

        // check squares
        for x in 0..DIM / 3 {
            for y in 0..DIM / 3 {
                let x = x * 3;
                let y = y * 3;
                let mut n_appears = [false; DIM];
                for i in x..x + 3 {
                    for j in y..y + 3 {

                        let n = if let Some(d) = board[i][j].to_digit(10) {
                            d
                        } else {
                            continue
                        };

                        if n_appears[(n - 1) as usize] {
                            return false;
                        }

                        n_appears[(n - 1) as usize] = true;

                    }
                }

            }
        }

        true

    }
}
