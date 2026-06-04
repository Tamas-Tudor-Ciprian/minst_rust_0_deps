use rand::Rng;
use std::f32::consts;

mod maths;
mod network;
mod mnist_parser;
mod network_manager;

use maths::*;
use network::*;
use mnist_parser::*;
use network_manager::*;

fn main(){

	//first I think I should see if i can import one of them dataset files

	let (n, rows , cols , images) = read_idx_images("data/train-images-idx3-ubyte");
	let (m, labels) = read_idx_labels("data/train-labels-idx1-ubyte");
	println!("Loaded {} images of size {}x{}",n, rows, cols);
	println!("Loaded {} labels", m);


	//now I'll try to print the first pixel of the first immage
	//(essentially this is the data I will send to the nn)
	//its also problably going to be white 99%
	println!("First pixel of the immage(probably white): {}", images[0]);
	
	//and also the label (the number it repersents)
	println!("The label of the first image: {}", labels[0]);


	//now we should just declare the network and start training
	


}
