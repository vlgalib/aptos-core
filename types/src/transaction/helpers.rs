// Copyright (c) The Diem Core Contributors
// SPDX-License-Identifier: Apache-2.0

use crate::{
    account_address::AccountAddress,
    chain_id::ChainId,
    transaction::{RawTransaction, SignedTransaction, TransactionPayload},
};
use anyhow::Result;
use aptos_crypto::{ed25519::*, test_utils::KeyPair, traits::SigningKey};
use chrono::Utc;

pub fn create_unsigned_txn(
    payload: TransactionPayload,
    sender_address: AccountAddress,
    sender_sequence_number: u64,
    max_gas_amount: u64,
    gas_unit_price: u64,
    gas_currency_code: String,
    txn_expiration_duration_secs: i64, // for compatibility with UTC's timestamp.
    chain_id: ChainId,
) -> RawTransaction {
    RawTransaction::new(
        sender_address,
        sender_sequence_number,
        payload,
        max_gas_amount,
        gas_unit_price,
        gas_currency_code,
        (Utc::now().timestamp() + txn_expiration_duration_secs) as u64,
        chain_id,
    )
}

pub trait TransactionSigner {
    fn sign_txn(&self, raw_txn: RawTransaction) -> Result<SignedTransaction>;
}

/// Craft a transaction request.
pub fn create_user_txn<T: TransactionSigner + ?Sized>(
    signer: &T,
    payload: TransactionPayload,
    sender_address: AccountAddress,
    sender_sequence_number: u64,
    max_gas_amount: u64,
    gas_unit_price: u64,
    gas_currency_code: String,
    txn_expiration_duration_secs: i64, // for compatibility with UTC's timestamp.
    chain_id: ChainId,
) -> Result<SignedTransaction> {
    let raw_txn = create_unsigned_txn(
        payload,
        sender_address,
        sender_sequence_number,
        max_gas_amount,
        gas_unit_price,
        gas_currency_code,
        txn_expiration_duration_secs,
        chain_id,
    );
    signer.sign_txn(raw_txn)
}

impl TransactionSigner for KeyPair<Ed25519PrivateKey, Ed25519PublicKey> {
    fn sign_txn(&self, raw_txn: RawTransaction) -> Result<SignedTransaction> {
        let signature = self.private_key.sign(&raw_txn);
        Ok(SignedTransaction::new(
            raw_txn,
            self.public_key.clone(),
            signature,
        ))
    }
}
