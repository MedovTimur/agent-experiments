//! AdminFeeMarket: a deliberately weak marketplace used as a Cerberus review
//! ladder project.
//!
//! Sellers create hashed listings. Buyers call `Buy` with an accounting
//! payment amount. The program records a 50% admin fee and a 50% seller
//! balance. This intentionally avoids full escrow semantics, which is part of
//! why it is weaker than TinyPoll.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use sails_rs::cell::RefCell;
use sails_rs::gstd::msg;
use sails_rs::prelude::*;

pub type ListingId = u64;

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct Listing {
    pub id: ListingId,
    pub seller: ActorId,
    pub content_hash: [u8; 32],
    pub price: u128,
    pub active: bool,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub struct PurchaseReceipt {
    pub listing_id: ListingId,
    pub buyer: ActorId,
    pub seller: ActorId,
    pub seller_amount: u128,
    pub admin_fee: u128,
    pub paid: u128,
}

#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum Error {
    Unauthorized,
    ZeroPrice,
    EmptyContentHash,
    ListingMissing,
    ListingClosed,
    Underpaid,
    ArithmeticOverflow,
    WithdrawExceedsBalance,
    TransferFailed,
}

#[derive(Clone)]
pub struct AdminFeeMarketState {
    pub admin: ActorId,
    pub next_listing_id: ListingId,
    pub listings: BTreeMap<ListingId, Listing>,
    pub seller_balances: BTreeMap<ActorId, u128>,
    pub admin_balance: u128,
}

impl Default for AdminFeeMarketState {
    fn default() -> Self {
        Self {
            admin: ActorId::zero(),
            next_listing_id: 0,
            listings: BTreeMap::new(),
            seller_balances: BTreeMap::new(),
            admin_balance: 0,
        }
    }
}

#[sails_rs::event]
#[derive(Encode, Decode, TypeInfo, Clone, Debug, PartialEq, Eq)]
#[codec(crate = sails_rs::scale_codec)]
#[scale_info(crate = sails_rs::scale_info)]
pub enum AdminFeeMarketEvent {
    ListingCreated {
        listing_id: ListingId,
        seller: ActorId,
        price: u128,
    },
    ListingPurchased {
        listing_id: ListingId,
        buyer: ActorId,
        seller: ActorId,
        seller_amount: u128,
        admin_fee: u128,
    },
    AdminFeeWithdrawn {
        admin: ActorId,
        amount: u128,
    },
    SellerBalanceWithdrawn {
        seller: ActorId,
        amount: u128,
    },
}

pub struct AdminFeeMarketService<'a> {
    state: &'a RefCell<AdminFeeMarketState>,
}

impl<'a> AdminFeeMarketService<'a> {
    pub fn new(state: &'a RefCell<AdminFeeMarketState>) -> Self {
        Self { state }
    }
}

#[sails_rs::service(events = AdminFeeMarketEvent)]
impl<'a> AdminFeeMarketService<'a> {
    #[export]
    pub fn create_listing(
        &mut self,
        content_hash: [u8; 32],
        price: u128,
    ) -> Result<ListingId, Error> {
        if content_hash == [0; 32] {
            return Err(Error::EmptyContentHash);
        }
        if price == 0 {
            return Err(Error::ZeroPrice);
        }

        let seller = msg::source();
        let listing_id = {
            let mut state = self.state.borrow_mut();
            let listing_id = state
                .next_listing_id
                .checked_add(1)
                .ok_or(Error::ArithmeticOverflow)?;
            state.next_listing_id = listing_id;
            state.listings.insert(
                listing_id,
                Listing {
                    id: listing_id,
                    seller,
                    content_hash,
                    price,
                    active: true,
                },
            );
            listing_id
        };

        self.emit_event(AdminFeeMarketEvent::ListingCreated {
            listing_id,
            seller,
            price,
        })
        .map_err(|_| Error::TransferFailed)?;
        Ok(listing_id)
    }

    #[export]
    pub fn buy(&mut self, listing_id: ListingId, payment: u128) -> Result<PurchaseReceipt, Error> {
        let buyer = msg::source();

        let result = {
            let mut state = self.state.borrow_mut();
            let (seller, price) = match state.listings.get_mut(&listing_id) {
                Some(listing) => {
                    if !listing.active {
                        return Err(Error::ListingClosed);
                    }
                    if payment < listing.price {
                        return Err(Error::Underpaid);
                    }

                    listing.active = false;
                    (listing.seller, listing.price)
                }
                None => return Err(Error::ListingMissing),
            };

            let admin_fee = price / 2;
            let seller_amount = price - admin_fee;

            state.admin_balance = state
                .admin_balance
                .checked_add(admin_fee)
                .ok_or(Error::ArithmeticOverflow)?;
            let seller_balance = state.seller_balances.entry(seller).or_default();
            *seller_balance = seller_balance
                .checked_add(seller_amount)
                .ok_or(Error::ArithmeticOverflow)?;

            PurchaseReceipt {
                listing_id,
                buyer,
                seller,
                seller_amount,
                admin_fee,
                paid: price,
            }
        };

        self.emit_event(AdminFeeMarketEvent::ListingPurchased {
            listing_id,
            buyer,
            seller: result.seller,
            seller_amount: result.seller_amount,
            admin_fee: result.admin_fee,
        })
        .map_err(|_| Error::TransferFailed)?;

        Ok(result)
    }

    #[export]
    pub fn withdraw_admin_fee(&mut self, amount: u128) -> Result<u128, Error> {
        let caller = msg::source();
        let admin = self.state.borrow().admin;
        if caller != admin {
            return Err(Error::Unauthorized);
        }

        {
            let mut state = self.state.borrow_mut();
            if amount > state.admin_balance {
                return Err(Error::WithdrawExceedsBalance);
            }
            state.admin_balance -= amount;
        }

        if amount > 0 {
            msg::send_bytes(admin, [], amount).map_err(|_| Error::TransferFailed)?;
        }
        self.emit_event(AdminFeeMarketEvent::AdminFeeWithdrawn { admin, amount })
            .map_err(|_| Error::TransferFailed)?;
        Ok(amount)
    }

    #[export]
    pub fn withdraw_seller_balance(&mut self, amount: u128) -> Result<u128, Error> {
        let seller = msg::source();
        {
            let mut state = self.state.borrow_mut();
            let balance = state.seller_balances.entry(seller).or_default();
            if amount > *balance {
                return Err(Error::WithdrawExceedsBalance);
            }
            *balance -= amount;
        }

        if amount > 0 {
            msg::send_bytes(seller, [], amount).map_err(|_| Error::TransferFailed)?;
        }
        self.emit_event(AdminFeeMarketEvent::SellerBalanceWithdrawn { seller, amount })
            .map_err(|_| Error::TransferFailed)?;
        Ok(amount)
    }

    #[export]
    pub fn get_listing(&self, listing_id: ListingId) -> Option<Listing> {
        self.state.borrow().listings.get(&listing_id).cloned()
    }

    #[export]
    pub fn listing_count(&self) -> ListingId {
        self.state.borrow().next_listing_id
    }

    #[export]
    pub fn admin(&self) -> ActorId {
        self.state.borrow().admin
    }

    #[export]
    pub fn admin_balance(&self) -> u128 {
        self.state.borrow().admin_balance
    }

    #[export]
    pub fn seller_balance(&self, seller: ActorId) -> u128 {
        self.state
            .borrow()
            .seller_balances
            .get(&seller)
            .copied()
            .unwrap_or_default()
    }
}

pub struct Program {
    state: RefCell<AdminFeeMarketState>,
}

#[sails_rs::program]
impl Program {
    pub fn create(admin: ActorId) -> Self {
        Self {
            state: RefCell::new(AdminFeeMarketState {
                admin,
                ..Default::default()
            }),
        }
    }

    pub fn admin_fee_market(&self) -> AdminFeeMarketService<'_> {
        AdminFeeMarketService::new(&self.state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sails_rs::gstd::services::Service as _;

    #[test]
    fn create_listing_validates_input() {
        let state = RefCell::new(AdminFeeMarketState {
            admin: ActorId::from(9),
            ..Default::default()
        });
        let mut service = AdminFeeMarketService::new(&state).expose(0);

        assert_eq!(
            Err(Error::EmptyContentHash),
            service.create_listing([0; 32], 100)
        );
        assert_eq!(Err(Error::ZeroPrice), service.create_listing([1; 32], 0));
    }
}
