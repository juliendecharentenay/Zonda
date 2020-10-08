// from https://gist.github.com/Twinklebear/f6751203f08ec3e12c62
extern crate sparse_matrix;
use sparse_matrix::{MatrixElement, SparseMatrix};

fn conjugate_gradient(matrix: &SparseMatrix, b: &Vec<f32>) -> Vec<f32> {
	let mut r = b.clone();
	let mut p = b.clone();
	let mut x = Vec::from_elem(b.len(), 0f32);
	let mut r_dot_r = [sparse_matrix::dot(&r, &r), 0f32];

	let mut step = 0u;
	let mut r_len = 1000f32;
	let converge_len = 1e-5f32;
	while step < 10 && r_len > converge_len {
		let mat_p = matrix * p;
		let alpha = r_dot_r[0] / sparse_matrix::dot(&p, &mat_p);

		for i in range(0, x.len()){
			*x.get_mut(i) += alpha * *p.get(i);
			*r.get_mut(i) -= alpha * *mat_p.get(i);
		}
		r_dot_r[1] = sparse_matrix::dot(&r, &r);

		let beta = r_dot_r[1] / r_dot_r[0];
		for i in range(0, p.len()){
			*p.get_mut(i) = *r.get(i) + beta * *p.get(i);
		}

		r_len = r_dot_r[1].sqrt();
		if r_len < converge_len {
			break;
		}
		r_dot_r[0] = r_dot_r[1];
		step += 1;
	}
	println!("Solution took {} iterations, residual length: {}", step + 1, r_len);
	x
}

#[test]
fn solve_identity(){
	let vals = vec![MatrixElement::new(0u, 0u, 1f32), MatrixElement::new(1u, 1u, 1f32)];
	let mat = SparseMatrix::new(2u, &vals);
	let b = vec![1f32, 2f32];
	let x = conjugate_gradient(&mat, &b);
	//Solving the identity should give x = b
	assert!((x.get(0) - 1f32).abs() < 0.001)
	assert!((x.get(1) - 2f32).abs() < 0.001);
}
#[test]
fn solve_wikipedia_ex(){
	let vals = vec![MatrixElement::new(0u, 0u, 4f32), MatrixElement::new(0u, 1u, 1f32),
		MatrixElement::new(1u, 0u, 1f32), MatrixElement::new(1u, 1u, 3f32)];
	let mat = SparseMatrix::new(2u, &vals);
	let b = vec![1f32, 2f32];
	let x = conjugate_gradient(&mat, &b);
	//Expected result here: [0.0909, 0.6363] from wikipedia
	assert!((x.get(0) - 0.0909).abs() < 0.001)
	assert!((x.get(1) - 0.6363).abs() < 0.001);
}
#[test]
fn solve_other_ex(){
	let vals = vec![MatrixElement::new(0u, 0u, 2f32), MatrixElement::new(0u, 1u, -1f32),
		MatrixElement::new(1u, 0u, -1f32), MatrixElement::new(1u, 1u, 2f32),
		MatrixElement::new(1u, 2u, -1f32), MatrixElement::new(2u, 1u, -1f32),
		MatrixElement::new(2u, 2u, 2f32)];
	let mat = SparseMatrix::new(3u, &vals);
	let b = vec![1f32, 2f32, 3f32];
	let x = conjugate_gradient(&mat, &b);
	//Expected result here: [2.5, 4.0, 3.5] from wolfram alpha
	assert!((x.get(0) - 2.5).abs() < 0.001);
	assert!((x.get(1) - 4.0).abs() < 0.001);
	assert!((x.get(2) - 3.5).abs() < 0.001);
}

