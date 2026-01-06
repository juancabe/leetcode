impl Solution {
    // r r r r r
    // c y y y y <- y - (1, 3)
    // c x w w w <- x = (2, 1)
    // c x v 1 1
    // c x v 2 p <- v = (4, 2)

    // c c c c r
    // x x x y r <- x = (1, 2)
    // v . . y r <- v = (2, 0)
    // . . . y r <- y = (3, 3)
    // . . . y r


    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        use std::mem::swap;
        let l = matrix.len();
        for layer in 0..l / 2 {
           for j in layer..(l - 1 - layer) {
//                 let upper = &mut matrix[layer][j];
//                 let right = &mut matrix[j][l - 1 - layer];
//                 let down = &mut matrix[l - 1 - layer][l - 1 - j];
//                 let left = &mut matrix[l - 1 - j][layer];
                let temp = matrix[layer][j];
                matrix[layer][j] = matrix[l - 1 - j][layer];
                matrix[l - 1 - j][layer] = matrix[l - 1 - layer][l - 1 - j];
                matrix[l - 1 - layer][l - 1 - j] = matrix[j][l - 1 - layer];
                matrix[j][l - 1 - layer] = temp;
            } 
        }
    }
}
