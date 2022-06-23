use super::layer::Layer;


type LossFn = fn (target:&Layer,predict:&Layer,dst:&Layer);
type TrainFn = fn (net:&NeuralNet,lr_rate:f32,epoch:usize,batch:usize);
enum TypeNet {
    Convolution,
    Perceptron,
    Recurrent,
    Other,
}
pub struct NeuralNet {
    buf:Vec<f32>,
    layers:Vec<Layer>,
    pub loss:LossFn,
    pub train:TrainFn,   
}

impl NeuralNet {
    pub fn new(loss:LossFn,train:TrainFn) -> Self {
        NeuralNet{
            buf:Vec::new(),
            layers:Vec::new(),
            loss,
            train,
        }
    }

    pub fn add_layer(&mut self,layer:Layer) {
        self.layers.push(layer)
    }
}
