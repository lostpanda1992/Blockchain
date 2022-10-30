mod blockchain;
use crate::blockchain::block::Blk;
use crate::blockchain::hash::*;



fn main() {
    println!("blockchain test\n\n");
    // create a better and more secure difficulty script
    let mut block = Blk::new(0,0,vec![0, 32], 0, "Gen blk".to_owned(), 0x0000ffffffffffffffffffffffffff);
    //unmined
    block.hash = block.hash();
    println!("{:?}", &block);


    //mining
    block.mine();
    println!("{:?}", &block);
}