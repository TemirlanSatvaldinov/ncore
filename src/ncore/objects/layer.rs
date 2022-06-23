type ComputeFn = fn (weight:&Layer,input:&Layer,output:&Layer);
type ActivationFn = fn  (x:f32) -> f32;

pub struct Layer {
    pub n:usize,
    pub c:usize,
    pub h:usize,
    pub w:usize,
    pub compute: ComputeFn,
    pub activation: ActivationFn,
    weight:Vec<f32>
    
}

impl Layer {
    pub fn new (n:usize,c:usize,h:usize,w:usize,compute:ComputeFn,activation:ActivationFn )-> Self {
        Layer {
            n,
            c,
            h,
            w,
            compute,
            activation,
            weight: vec![0.0;n*c*h*w],
        }
    }
}