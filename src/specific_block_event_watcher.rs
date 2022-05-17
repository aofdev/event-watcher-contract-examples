use std::str::FromStr;
use web3::types::{BlockNumber, FilterBuilder, H160, H256, U64};
#[tokio::main]
async fn main() -> web3::contract::Result<()> {
    let web3 = web3::Web3::new(web3::transports::Http::new(
        "https://bsc-dataseed.binance.org/",
    )?);

    let contract_address: H160 =
        H160::from_str("0x0eD7e52944161450477ee417DE9Cd3a859b14fD0").unwrap();

    let contract_event_swap: H256 =
        H256::from_str("0xd78ad95fa46c994b6551d0da85fc275fe613ce37657fb8d5e3d130840159d822")
            .unwrap();

    let contract_event_transfer: H256 =
        H256::from_str("0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef")
            .unwrap();

    let filter = FilterBuilder::default()
        .address(vec![contract_address])
        .topics(
            Some(vec![contract_event_swap, contract_event_transfer]),
            None,
            None,
            None,
        )
        // maximum block range: 5000
        .from_block(BlockNumber::Number(U64::from(17_854_300)))
        .to_block(BlockNumber::Number(U64::from(17_854_500)))
        .build();

    let event_logs = web3.eth().logs(filter).await?;
    for log in event_logs {
        println!("----------------------------------------");
        println!("Block Number: {}", log.block_number.unwrap());
        println!("Event Address: {:?}", log.topics[0]);
        println!("From Address: {}", log.topics[1]);
        println!("To Address: {}", log.topics[2]);
        println!("Data: {}", format!("0x{}", hex::encode(log.data.0)));
    }
    Ok(())
}
