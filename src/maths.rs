use crate::network::*;

//backprop essential
pub fn transpose_matrix(matrix: Vec<Vec<f32>>) -> Vec<Vec<f32>>{

        let mut matrix_T = vec![];

        for i in 0..matrix[0].len(){
                let mut row = vec![];
                for j in 0..matrix.len(){
                        row.push(matrix[j][i]);
                }
                matrix_T.push(row);
        }
        matrix_T
}

//the output function
pub fn sigmoid(input: f32) -> f32{

        1.0/(1.0 + (-input).exp())


}


//foward pass essential
pub fn prod(weights: Vec<Vec<f32>>, input: Vec<f32>) -> Vec<f32>{

        let mut output= vec![];


        for i in 0..weights.len() {
                let mut sum :f32 = 0.0;
                for j in 0..weights[0].len(){

                        sum += weights[i][j] * input[j];

                }
                output.push(sum);
                }

        output

}

//you get the weight gradient with this
pub fn prod_v2v(gradient: Vec<f32>, input: Vec<f32>) -> Vec<Vec<f32>>{


        let mut weights_gradient = vec![];

        for i in 0..gradient.len(){

                let mut row = vec![];

                for j in 0..input.len(){

                                row.push(input[j] * gradient[i]);

                }

                weights_gradient.push(row);

        }

        weights_gradient

}

//you add the bias in foward pass with this
pub fn add(input: Vec<f32>,bias: Vec<f32>) -> Vec<f32>{

        let mut output = vec![];

        for i in 0..input.len(){

                output.push(input[i] + bias[i]);
        }
        output
}


