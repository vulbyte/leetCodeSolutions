struct Matrix(Vec<Vec<i32>>);

impl Solution {
    pub fn update_matrix(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        //for each group
        for group in mat {
            //go over each item
            for item in group{
                //loop 

                //checking adjacent cells for a 0
                //if found add to new arr
                //if not found, return error
            }
        }
    }

    fn CheckRowByColumn(&mat: Vec<Vec<i32>, &x:i8, &y:i8) {
        
    }
}

impl std::fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for each in self {
              
        }
    }
}

fn main() {
        

    let test_mat_1: Vec<Vec<i32>> = [[0,0,0],[0,1,0],[0,0,0]];
    test_mat_1;
    /* 
     * output: 
     * [
     * [0,0,0],
     * [0,1,0],
     * [0,0,0]
     * ] 
     * */
    let test_mat_2: Vec<Vec<i32>> = [[0,0,0],[0,1,0],[1,1,1]];
    /* 
     * output: 
     * [
     * [0,0,0],
     * [0,1,0],
     * [1,2,1]
     * ] 
     * */

}

