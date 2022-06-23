mod ncore;
use crate::ncore::objects::layer::Layer;
use crate::ncore::objects::net::*;
fn main() {
  let weight:Layer = Layer::new(1,1,1,1,fully,relu);
  let input:Layer = Layer::new(1,1,1,1,fully,relu);
  let out:Layer = Layer::new(1,1,1,1,fully,relu);

  //
  let mut net:NeuralNet  = NeuralNet::new(loss,train);
  net.add_layer(weight);
  net.add_layer(input);
  net.add_layer(out);

  println!("worked !");

}




