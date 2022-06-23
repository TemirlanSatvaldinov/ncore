
pub fn relu (x:f32) ->f32 {
    if x <= 0.0 {
        return 0.001 * x ;
    }
    return  x;
}

pub fn soft_max (src:&Vec<f32>) {

}
