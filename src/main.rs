use blockchainlib::*;

fn main () {
    let difficulty = 0x000fffffffffffffffffffffffffffff; //difficulty is for safety

    let mut genesis_block = Block::new(0, now(), vec![0; 32], vec![     //creating new genesis
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Ata".to_owned(),                              //ownerships for person 1
                    value: 50,
                },
                transaction::Output {
                    to_addr: "Okan".to_owned(),                            //ownerships for person 1
                    value: 10,
                },
            ],
        },
    ], difficulty);

    genesis_block.mine();

    println!("Mined genesis block {:?}", &genesis_block);

    let mut last_hash = genesis_block.hash.clone();

    let mut blockchain = Blockchain::new();

    blockchain.update_with_block(genesis_block).expect("Failed to add genesis block");

    let mut block = Block::new(1, now(), last_hash, vec![
        Transaction {
            inputs: vec![ ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Ali".to_owned(),
                    value: 250,
                },
            ],
        },
        Transaction {
            inputs: vec![
                blockchain.blocks[0].transactions[0].outputs[0].clone(),
            ],
            outputs: vec![
                transaction::Output {
                    to_addr: "Ata".to_owned(),
                    value: 300,
                },
                transaction::Output {
                    to_addr: "Okan".to_owned(),
                    value: 5,
                },
            ],
        },
    ], difficulty);

    block.mine();

    println!("Mined block {:?}", &block);

    last_hash = block.hash.clone();

    blockchain.update_with_block(block).expect("Failed to add block");
}
