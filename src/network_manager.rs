//this file will handle saving a network to a file and reading from it
use crate::newtork::*;







pub fn write_to_file(net : &Network,name : &String, path : &String){
	//the layers hold the activations so they do not matter
	//we will just be writing the connections one strata at a time


	let mut file = File::create(name);

	//each line will have a con layer
	//first we must organise such a string
	
	let cons = net.cons.clone();

	for con in cons {
		
		for i in 0..weights.len(){
		
			let neuron_cons = weights[i];


			for neuron_con in neuron_cons{

				


			}

	
		}



	}

	





}
