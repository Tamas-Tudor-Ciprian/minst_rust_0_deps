//this file will handle saving a network to a file and reading from it
use crate::newtork::*;




pub fn write_to_file(net : &Network,name : &String, path : &String){
	//the layers hold the activations so they do not matter
	//we will just be writing the connections one strata at a time


	let mut file = File::create(name);

	//each line will have a con layer
	//first we must organise such a string
	
	let cons = net.cons.clone();

	//there surely must be a better way to read this other than a bunch of nested fors
	for con in cons {
	
		let weights = cons.weights.clone();
		
		file.write_all(b"weights\n");
		
		for i in 0..weights.len(){

			let neuron_cons = weights[0];

			for con in neuron_cons{
				
				file.write_all(con);
			

			}

		file_write_all(b"\n");
		
	
		}

		let biases = cons.biases.clone();

		for i in 0..biases.len(){

		}

	}

	





}
