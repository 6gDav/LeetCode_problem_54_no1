pub struct Solution;

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut matrix = matrix;
        let mut dummy_vector: Vec<i32> = Vec::new();

        while !matrix.is_empty() && !matrix[0].is_empty() {
            //frist row added to the resoult vector
            {
                dummy_vector.extend(matrix.remove(0));
            }

            //add very last elemt and remove from the original matrix
            {
                for row in matrix.iter_mut() {
                    if let Some(val) = row.pop() {
                        dummy_vector.push(val);
                    }
                }
            }

            //remove the last row reverse it and add to the list
            {
                if let Some(mut last_row) = matrix.pop() {
                    if !last_row.is_empty() {
                        last_row.reverse();
                        dummy_vector.extend(last_row);
                    }
                }  
            }

            //add every starter character to the list while remove it from the original matrix
            {
                for row in matrix.iter_mut().rev() {
                    if !row.is_empty() {
                        dummy_vector.push(row.remove(0));
                    }
                } 
            }

        }
        dummy_vector
    }
}
