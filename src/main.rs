mod blockchain;
use blockchain::Blockchain;

fn main(){
    let mut my_blockchain = Blockchain {
        chain: Vec::new()
    };
    my_blockchain.create_genesis();
    my_blockchain.add_block(&String::from("data 1"));
    my_blockchain.add_block(&String::from("data 2"));
    my_blockchain.add_block(&String::from("data 3"));
    my_blockchain.add_block(&String::from("data 4"));

    my_blockchain.list_blocks();
}