///Name: Hangchen Qu
///ID: hq5na

use std::rand::random;
use std::os;
use std::io::File;

fn main() {
		let args = os::args();
        let fname1: &str = args[1];
		let fname2: &str = args[2];
        let path1 = Path::new(fname1.clone());
        let msg_file1 = File::open(&path1);
        let path2 = Path::new(fname2.clone());
        let msg_file2 = File::open(&path2);
		
		let output_file = File::create(&Path::new("output"));
		match (msg_file1, msg_file2) {
            (Some(mut msg1), Some(mut msg2)) => {
                let share1_bytes: ~[u8] = msg1.read_to_end();
            	let share2_bytes: ~[u8] = msg2.read_to_end();
                match (output_file) {
                    Some(output) => { 
                        join(share1_bytes, share2_bytes, output); 
                        } ,
                    None => fail!("Error opening output files!"),
                }
            } ,
            (_, _) => fail!("Error opening message file")
        }
}

fn xor(a: &[u8], b: &[u8]) -> ~[u8] {
    let mut ret = ~[];
    for i in range(0, a.len()) {
	ret.push(a[i] ^ b[i]);
    }
    ret
}

fn join(share1_bytes: &[u8], share2_bytes: &[u8], mut output_file: File) { 
    let decrypted_bytes = xor(share1_bytes, share2_bytes);
    output_file.write(decrypted_bytes);
}
