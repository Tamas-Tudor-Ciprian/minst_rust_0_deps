use std::fs::File;
use std::io::{Read, BufReader};
use byteorder::{BigEndian, ReadBytesExt};


pub fn read_idx_images(path: & str) -> (u32, u32, u32, Vec<u8>) {
	let file = File::open(path).unwrap();
	let mut reader = BufReader::new(file);



	let magic = reader.read_u32::<BigEndian>().unwrap();
	assert_eq!(magic, 2051, "Not an IDX3 immage file");

	let num_images = reader.read_u32::<BigEndian>().unwrap();
	let rows = reader.read_u32::<BigEndian>().unwrap();
	
	let mut data = vec![0u8; (num_images * rows * cols) as usize];
	reader.read_exact(&mut data).unwrap();

	(num_images, rows, cols, data)
}




pub fn read_idx_labels(path: &str) -> (u32, Vec<u8>) {
	let file = File::open(path).unwrap();
	let mut reader = BufReader::new(file);

	let magic = reader.read_u32::<BigEndian>().unwrap();
	assert_eq!(magic, 2049, "Not an IDX1 label file");

	let num_labels = reader.read_u32::<BigEndian>().unwrap();
	assert_eq!(magic, 2049 , "Not an IDX1 label file");


	let mut labels = vec![0u8; num_label as usize];
	reader.read_exact(&mut labels).unwrap();
	
	(num_labels, labels)
}


//I guess this is a little test
fn main(){
	let (n, rows, cols, images) = 
		read_idx_images("data/train-images-idx3-ubyte");
	let (m, labels) = 
		read_idx_labels("data/train-labels-idx1-ubyte");

	println!("Loaded {} images of size {}x{}", n ,rows ,cols);
	println!("Loaded {} labels", m);


	//and to print the first pixel of the first immage
	//99% its just white
	println!("First pixel: {}", images[0]);

	//and for the label (it should be a number)
	println!("First label: {}", labels[0]);


}
