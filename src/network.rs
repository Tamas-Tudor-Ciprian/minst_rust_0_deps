use crate::maths::*;
use rand::Rng;

//the one helper function in this file
//this is so you can avergage the gradient batch
pub fn avg(gradient_batch: Vec<Vec<Connections>>,lr: f32)->Vec<Connections>{

        //we divide the sum of all values by this
        let n = gradient_batch.len();



        //now for the complicated part, we declare an copy of the first gradient and sum everything in it
        let mut sum = gradient_batch[0].clone();

        //now we iterate from the second and sum
        for i in 1..gradient_batch.len(){
                let current_grad = gradient_batch[i].clone();

                //now we go trough each connections element
                for j in 0..current_grad.len(){
                        let current_w = current_grad[j].weights.clone();
                        let current_b = current_grad[j].biases.clone();
                        //the actual summing algo starts here
                        for k in 0..current_w.len(){
                                sum[j].biases[k] +=current_grad[j].biases[k];
                                for l in 0..current_w[0].len(){
                                        sum[j].weights[k][l] += current_grad[j].weights[k][l];
                                }
                        }


                }



        }

        let to_apply = 1.0/(n as f32);

        for mut gradient in sum.iter_mut(){
                for i in 0..gradient.weights.len(){
                        gradient.biases[i] *= to_apply * lr;
                        for j in 0..gradient.weights[0].len(){

                                gradient.weights[i][j] *= to_apply * lr;

                        }

                }
        }

        sum


}





#[derive(Clone)]
struct Layer{
        activations: Vec<f32>,
}


impl Layer {
        fn first(input_data: Vec<f32>) -> Self{
                Self{
                        activations : input_data,

                }
        }

        fn new(input: Layer, cons: Connections ) -> Self{
                let non_biased = prod(cons.weights, input.activations);
                let biased = add(non_biased,cons.biases);
                let squished = biased.iter().map(|x| sigmoid(*x)).collect();
                Self{
                        activations: squished,

                }
        }
}


#[derive(Clone)]
struct Connections{
        weights: Vec<Vec<f32>>,
        biases: Vec<f32>,
}




impl Connections{

        fn new(st_layer_len: usize,nd_layer_len: usize) -> Self {
                let mut rng = rand::thread_rng();

                let mut weights =
                (0..nd_layer_len).map(|_| {
                (0..st_layer_len).map(|_| rng.gen::<f32>() * 4.0 - 2.0)//random in (-2,2)
                .collect()}).collect();

                let mut biases =
                (0..nd_layer_len).map(|_| rng.gen::<f32>() * 4.0 - 2.0) //hope the biases don't have too big a range
                .collect();

                Self {
                        weights: weights,
                        biases: biases,
                        }

                }

        fn from_con(w: Vec<Vec<f32>>, b: Vec<f32>)-> Self{
                Self{
                        weights: w,
                        biases: b,
                }
        }



        fn empty(st_layer_len: usize,nd_layer_len: usize) -> Self {
                let mut rng = rand::thread_rng();

                let mut weights = vec![vec![0.0;st_layer_len];nd_layer_len];

                let mut biases = vec![0.0;nd_layer_len];

                Self {
                        weights: weights,
                        biases: biases,
                }

        }



}



struct Network{
        layers: Vec<Layer>,
        cons: Vec<Connections>
}



impl Network{
        fn new(layers: Vec<usize>) -> Self{


                let mut cons = vec![];

                for i in 1..layers.len(){

                        let layer_con = Connections::new(layers[i-1], layers[i]);
                        cons.push(layer_con);
                }

                let empty_vec = vec![];


                Self{

                        cons: cons,
                        layers:empty_vec,

                }

        }

        /*
        it is to be mentioned that the first layer is to be the same size as the already declared
        input of this network
        */
        pub fn pass(&mut self,input : Layer){

                self.layers.clear();

                let mut current_layer = input;

                //making sure the first layer is also in the network
                self.layers.push(current_layer.clone());

                for con in self.cons.clone(){

                        let next_layer = Layer::new(current_layer.clone(), con.clone());

                        current_layer = next_layer.clone();

                        self.layers.push(next_layer);

                }
        }

        pub fn backprop_gradient(&mut self,y : Vec<f32>)->Vec<Connections>{


                let mut gradients: Vec<Connections> = vec![];


                //------------output layer--------------


                //lets handle the output layer to kick things off

                let output = self.layers.last().unwrap().activations.clone();

                //dL/da_n = a_n - y
                let output_error: Vec<_> = output.iter().zip(y.iter()).map(|(x,y)| x - y).collect();

                //dL/z_n = a * (1 - a) <- this is the sigmoid derivative
                let output_derivative: Vec<_> = output.iter().map(|x| x * (1.0 - x)).collect();

                //dL/a_n * dL/z_n <- chain rule derivative that will propagate
                let delta_output : Vec<_> = output_error.iter().zip(output_derivative.iter()).map(|(x,y)| x * y).collect();


                let mut last_delta = delta_output.clone();

                //=======================================layers loop=================================
                for i in 2..=self.layers.len(){
                        //------------hidden layer---------

                        //now lets try and handle the first hidden layer aka a_n-1
                        let hidden = self.layers[self.layers.len() - i].activations.clone();

                        //this is the sigmoid derivative for this layer and will help propagate the error further
                        let hidden_derivative : Vec<_> = hidden.iter().map(|x| x * (1.0 - x)).collect();

                        let w_out = self.cons[self.cons.len() - 1 - (i - 2)].weights.clone();


                        //now to get the gradient we need to transpose the weights
                        let mut w_out_T = transpose_matrix(w_out);

                        //now the past layer gradient times the transposed matrix will be the current gradient
                        let hidden_error = prod(w_out_T,last_delta.clone());

                        let delta_hidden :Vec<_> = hidden_error.iter().zip(hidden_derivative.iter()).map(|(x,y)| x*y).collect();


                        //=========weight and bias gradient============
                        //now this is the whole purpose of this function

                        // dL/dW = delta_n x a_n-1
                        let w_grad = prod_v2v(last_delta.clone(), hidden.clone());

                        //as per chain rule the bias gradient is simply the delta from the current layer

                        let bias_grad = last_delta.clone();

                        gradients.insert(0,Connections::from_con(w_grad,bias_grad));

                        last_delta = delta_hidden;

                }


                gradients


        }

        //once you got the averaged gradient over a batch you can apply it with this
        pub fn apply_gradient(&mut self , grad: Vec<Connections>){

                let mut network = &mut self.cons;

                for  k in 0..grad.len(){

                        let Connections { weights: w, biases: b } = &mut network[k];

                        let w_grad = &grad[k].weights;
                        let b_grad = &grad[k].biases;
                        for i in 0..w.len(){
                                b[i] -= b_grad[i];
                                for j in 0..w[0].len(){

                                        w[i][j] -= w_grad[i][j];

                                }

                        }
                }

        }

}


