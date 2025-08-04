use core::blockchain;

fn main() {
    let mut bc = blockchain::BlockChain::new_blockchain();

    let mut block1_datas = Vec::new();
    block1_datas.push(utils::coder::one_piece_data(
        "Tom".to_string(),
        "Jerry".to_string(),
        5,
    ));
    block1_datas.push(utils::coder::one_piece_data(
        "Taki".to_string(),
        "Herry".to_string(),
        10,
    ));
    bc.add_block(block1_datas);

    let mut block2_datas = Vec::new();
    block2_datas.push(utils::coder::one_piece_data(
        "Kyomoto".to_string(),
        "Fujino".to_string(),
        5,
    ));
    block2_datas.push(utils::coder::one_piece_data(
        "Wower".to_string(),
        "Apex".to_string(),
        10,
    ));
    block2_datas.push(utils::coder::one_piece_data(
        "Wower".to_string(),
        "Apex".to_string(),
        10,
    ));
    block2_datas.push(utils::coder::one_piece_data(
        "Wower".to_string(),
        "Apex".to_string(),
        10,
    ));
    block2_datas.push(utils::coder::one_piece_data(
        "Wower".to_string(),
        "Apex".to_string(),
        10,
    ));
    bc.add_block(block2_datas);

    for b in bc.blocks {
        println!("====================================");
        println!("{:#?}", b);
        println!("====================================");
    }

    println!("Hello, world!");
}
