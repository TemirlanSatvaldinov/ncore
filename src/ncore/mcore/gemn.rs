/**
 * N - batch,
 * C - channel,
 * H - row/height,
 * W = col/weight
 */

pub fn gemn_n (A:&Vec<f32>,row_a:usize,col_a:usize,B:&Vec<f32> ,row_b:usize,col_b:usize , C:&mut Vec<f32>) {
    if  col_a != row_b { panic!("trying multiple {}x{} with {}x{}",row_a,col_a,row_b,col_b)}

    for i in 0..row_a {
        let mut c = &mut C[i*col_b..];
        for k in 0..col_a {
            let  b = &B[k*col_b..];
            let  a = A[i*col_a+k];
            for j in 0..col_b {
                c[j] += a * b[j];
            }
        }
    }
}

pub fn gemn_fn (A:&Vec<f32>,row_a:usize,col_a:usize,B:&Vec<f32> ,row_b:usize,col_b:usize , C:&mut Vec<f32>) {
    if  col_a != row_b { panic!("trying multiple {}x{} with {}x{}",row_a,col_a,row_b,col_b)}

    for i in 0..row_a {
        let mut c = &mut C[i*col_b..];
        for k in 0..col_a {
            let  b = &B[k*col_b..];
            let  a = A[i*col_a+k];
            for j in 0..col_b {
                c[j] += a * b[j];
            }
        }
    }
}