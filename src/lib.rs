#[allow(unused)]
mod pb;

use pb::mydata::v1 as mydata;
use substreams_solana::pb::sf::solana::r#type::v1::Block;

// Solana System Program ID
const SYSTEM_PROGRAM_ID: &str = "11111111111111111111111111111111";

// System Program instruction discriminator for Transfer
const TRANSFER_INSTRUCTION: u32 = 2;

#[substreams::handlers::map]
fn map_my_data(blk: Block) -> mydata::MyData {
    let mut transfers = Vec::new();

    // Iterate through successful transactions in this block.
    for tx in blk.transactions() {

        // Walk through top-level and inner instructions.
        for instruction in tx.walk_instructions() {

            // Process only System Program instructions.
            if instruction.program_id().to_string() != SYSTEM_PROGRAM_ID {
                continue;
            }

            let data = instruction.data();

            // A native SOL transfer instruction contains:
            // 4 bytes: instruction type
            // 8 bytes: amount in lamports
            if data.len() < 12 {
                continue;
            }

            // Read the first 4 bytes as a little-endian instruction type.
            let instruction_type = u32::from_le_bytes([
                data[0], data[1], data[2], data[3],
            ]);

            // We only want the Transfer instruction (type 2).
            if instruction_type != TRANSFER_INSTRUCTION {
                continue;
            }

            // Read the next 8 bytes as the lamport amount.
            let amount = u64::from_le_bytes([
                data[4], data[5], data[6], data[7],
                data[8], data[9], data[10], data[11],
            ]);

            // Get sender and receiver accounts.
            let accounts = instruction.accounts();

            if accounts.len() < 2 {
                continue;
            }

            let from = accounts[0].to_string();
            let to = accounts[1].to_string();

            // Add the transfer to the output.
            transfers.push(mydata::Transfer {
                from,
                to,
                amount,
            });
        }
    }

    mydata::MyData { transfers }
}
