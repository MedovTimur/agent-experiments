#![allow(dead_code)]

use std::collections::BTreeMap;

pub type AccountId = [u8; 32];
pub type ListingId = u64;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Listing {
    pub id: ListingId,
    pub seller: AccountId,
    pub content_hash: [u8; 32],
    pub price: u128,
    pub active: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PurchaseReceipt {
    pub listing_id: ListingId,
    pub buyer: AccountId,
    pub seller_amount: u128,
    pub admin_fee: u128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MarketError {
    ZeroPrice,
    EmptyContentHash,
    ListingMissing,
    ListingClosed,
    Underpaid,
}

pub struct AdminFeeMarket {
    admin: AccountId,
    next_listing_id: ListingId,
    listings: BTreeMap<ListingId, Listing>,
    seller_balances: BTreeMap<AccountId, u128>,
    admin_balance: u128,
}

impl AdminFeeMarket {
    pub fn new(admin: AccountId) -> Self {
        Self {
            admin,
            next_listing_id: 0,
            listings: BTreeMap::new(),
            seller_balances: BTreeMap::new(),
            admin_balance: 0,
        }
    }

    pub fn create_listing(
        &mut self,
        seller: AccountId,
        content_hash: [u8; 32],
        price: u128,
    ) -> Result<ListingId, MarketError> {
        if content_hash == [0; 32] {
            return Err(MarketError::EmptyContentHash);
        }
        if price == 0 {
            return Err(MarketError::ZeroPrice);
        }

        self.next_listing_id += 1;
        self.listings.insert(
            self.next_listing_id,
            Listing {
                id: self.next_listing_id,
                seller,
                content_hash,
                price,
                active: true,
            },
        );
        Ok(self.next_listing_id)
    }

    pub fn buy(
        &mut self,
        buyer: AccountId,
        listing_id: ListingId,
        payment: u128,
    ) -> Result<PurchaseReceipt, MarketError> {
        let listing = self
            .listings
            .get_mut(&listing_id)
            .ok_or(MarketError::ListingMissing)?;
        if !listing.active {
            return Err(MarketError::ListingClosed);
        }
        if payment < listing.price {
            return Err(MarketError::Underpaid);
        }

        listing.active = false;

        let admin_fee = listing.price / 2;
        let seller_amount = listing.price - admin_fee;
        self.admin_balance += admin_fee;
        *self.seller_balances.entry(listing.seller).or_default() += seller_amount;

        Ok(PurchaseReceipt {
            listing_id,
            buyer,
            seller_amount,
            admin_fee,
        })
    }

    pub fn admin(&self) -> AccountId {
        self.admin
    }

    pub fn admin_balance(&self) -> u128 {
        self.admin_balance
    }

    pub fn seller_balance(&self, seller: AccountId) -> u128 {
        self.seller_balances.get(&seller).copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sale_sends_half_to_admin() {
        let admin = [9; 32];
        let seller = [1; 32];
        let buyer = [2; 32];
        let mut market = AdminFeeMarket::new(admin);

        let listing_id = market.create_listing(seller, [7; 32], 100).unwrap();
        let receipt = market.buy(buyer, listing_id, 100).unwrap();

        assert_eq!(receipt.admin_fee, 50);
        assert_eq!(receipt.seller_amount, 50);
        assert_eq!(market.admin_balance(), 50);
        assert_eq!(market.seller_balance(seller), 50);
    }
}
