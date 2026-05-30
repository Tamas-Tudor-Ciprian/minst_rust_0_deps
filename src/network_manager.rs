//this file will handle saving a network to a file and reading from it
use crate::network::*;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub fn write_to_file(net : &Network,name : &str, path : &str){
	//the layers hold the activations so they do not matter
	//we will just be writing the connections one strata at a time


	let mut file = File::create(name).unwrap();

	//each line will have a con layer
	//first we must organise such a string
	
	let cons = net.cons.clone();

	//there surely must be a better way to read this other than a bunch of nested fors
	for con in cons {
	
		let weights = con.weights.clone();
		
		file.write_all(b"weights\n").unwrap();
		
		for i in 0..weights.len(){

			let neuron_cons = weights[0].clone();

			for con in neuron_cons{
				
				file.write_all(&con.to_le_bytes()).unwrap();
				file.write_all(b" ").unwrap();
			

			}

		file.write_all(b"\n").unwrap();
	
		}

		let biases = con.biases.clone();

		file.write_all(b"biases\n");
		for bias in biases{

			file.write_all(&bias.to_le_bytes());		
			file.write_all(b" ");

		}
		
		file.write_all(b"\n");

	}

}

/*
fn get_from_file(path: &str) -> Vec<Connections>{

	//we have a from Connections constructor we only need to read them from the file
	



}
*/

#[cfg(test)]

mod tests {
	use super::*;



	#[test]
	fn test_write(){
	// I think here I should generate a random network and see if it works

		//lets have it be like the function aproximator
		let net_size = vec![1,10,10,1];

		let net = Network::new(net_size);

		
		write_to_file(&net,"test_network.nn","./" );

		let current = env::current_dir().expect("Failed to get current dir");

		let parrent = current.parent().expect("Failed to get parent directory");
		let network_file = parrent.join("test_network.nn");

		assert!(network_file.exists());

	}

}
