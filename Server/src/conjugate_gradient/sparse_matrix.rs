// From https://gist.github.com/Twinklebear/f6751203f08ec3e12c62

#![crate_type="lib"]
#![crate_id="sparse_matrix#0.0.1"]

use std::fmt::{Show, Formatter, FormatError};

/*
 * An element in a sparse matrix, the value at some [row, col] index
 */
#[deriving(Clone)]
pub struct MatrixElement {
	row: uint,
	col: uint,
	val: f32
}

/*
 * A square sparse matrix of some dimension
 */
#[deriving(Clone)]
pub struct SparseMatrix {
	pub dim: uint,
	matrix: Vec<MatrixElement>
}

//Compute the dot product of two large vectors
pub fn dot(a: &Vec<f32>, b: &Vec<f32>) -> f32 {
	assert_eq!(a.len(), b.len());
	let mut result = 0f32;
	for (x, y) in a.iter().zip(b.iter()) {
		result += *x * *y;
	}
	result
}

impl MatrixElement {
	pub fn new(row: uint, col: uint, val: f32) -> MatrixElement {
		MatrixElement { row: row, col: col, val: val }
	}
}

impl Show for MatrixElement {
	fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
		write!(f, "[{}, {}] = {}", self.row, self.col, self.val)
	}
}

impl SparseMatrix {
	//Elements passed in should be sorted by row
	pub fn new(dim: uint, matrix: &Vec<MatrixElement>) -> SparseMatrix {
		SparseMatrix { dim: dim, matrix: matrix.clone() }
	}
	pub fn print(&self){
		for e in self.matrix.iter() {
			println!("[{}, {}] = {}", e.row, e.col, e.val);
		}
	}
}

impl Show for SparseMatrix {
	fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
		try!(write!(f, "[[\n"));
		for e in self.matrix.iter() {
			try!(write!(f, "{}\n", e));
		}
		write!(f, "]]")
	}
}

//Multiply the sparse matrix by a column vector. The vector should have dim elements
impl Mul<Vec<f32>, Vec<f32>> for SparseMatrix {
	fn mul(&self, v: &Vec<f32>) -> Vec<f32> {
		assert_eq!(self.dim, v.len());
		let mut result = Vec::new();
		//Multiply each row with the column vector
		for row in range(0u, self.dim){
			//Determine the range of indices covering this row, perhaps later keep track of the
			//previous index and resume search there?
			let mut indices = [-1, -1];
			for i in range(0u, self.matrix.len()){
				if self.matrix.get(i).row == row && indices[0] == -1 {
					indices[0] = i;
				}
				if self.matrix.get(i).row == row + 1 && indices[1] == -1 {
					indices[1] = i - 1;
					break;
				}
				if i == self.matrix.len() - 1 && indices[1] == -1 {
					indices[1] = i;
				}
			}
			let mut sum = 0f32;
			for i in range(indices[0] as uint, indices[1] + 1 as uint){
				sum += self.matrix.get(i).val * *v.get(self.matrix.get(i).col);
			}
			result.push(sum);
		}
		result
	}
}
