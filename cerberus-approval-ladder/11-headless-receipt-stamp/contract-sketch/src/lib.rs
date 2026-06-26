#![allow(dead_code)]

/// Contract-shaped sketch only. This is not a Sails workspace.
pub struct Receipt {
    pub id: u64,
    pub owner: [u8; 32],
    pub receipt_hash: [u8; 32],
    pub block_height: u64,
}

pub enum ReceiptError {
    ZeroReceiptHash,
}

pub struct ReceiptStamp {
    next_id: u64,
    receipts: Vec<Receipt>,
}

impl ReceiptStamp {
    pub fn stamp(&mut self, owner: [u8; 32], receipt_hash: [u8; 32], block_height: u64) -> Result<u64, ReceiptError> {
        if receipt_hash == [0; 32] {
            return Err(ReceiptError::ZeroReceiptHash);
        }

        self.next_id += 1;
        self.receipts.push(Receipt {
            id: self.next_id,
            owner,
            receipt_hash,
            block_height,
        });
        Ok(self.next_id)
    }
}

