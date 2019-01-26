use crypto::digest::Digest;
use crypto::sha2::Sha512;
use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::from_utf8;
use crate::addressing::{get_hamlet_prefix, make_asset_address, make_account_address};
use crate::protos::*;
use protobuf::Message;
use protobuf;

pub struct HamletState<'a> {
    context: &'a mut TransactionContext
}

impl<'a> HamletState<'a> {
    pub fn new(context: &'a mut TransactionContext) -> HamletState {
        HamletState {
            context
        }
    }


    pub fn get_asset(&mut self, asset_name: &str) -> Result<Option<asset::Asset>, ApplyError> {
        let address = make_asset_address(asset_name);
        let asset_containers = self.context.get_state(vec![address])?;

        match asset_containers {
            Some(packed) => {
                let assets: asset::AssetContainer =
                    match protobuf::parse_from_bytes(packed.as_slice()) {
                        Ok(assets) => assets,
                        Err(_) => {
                            return Err(ApplyError::InternalError(String::from(
                                "Cannot deserialize asset container",
                            )))
                        }
                    };

                for asset in assets.get_entries() {
                    if asset.name == asset_name {
                        return Ok(Some(asset.clone()));
                    }
                }
                Ok(None)
            }
            None => Ok(None),
        }
    }

    pub fn set_asset(&mut self, asset_name: &str, a: asset::Asset) -> Result<(), ApplyError> {
        let address = make_asset_address(asset_name);
        let asset_containers = self.context.get_state(vec![address.clone()])?;

        let mut asset_container = match asset_containers {
            Some(packed) => match protobuf::parse_from_bytes(packed.as_slice()) {
                Ok(assets) => assets,
                Err(_) => {
                    return Err(ApplyError::InternalError(String::from(
                        "Cannot deserialize asset container",
                    )))
                }
            },
            None => asset::AssetContainer::new(),
        };
        /*
         *  Remove old asset if it exists and sort the assets by name
         */
        let assets = asset_container.get_entries().to_vec();
        let mut index = None;
        let mut count = 0;

        // Find the existing Asset's index in the deserialized Asset list
        for asset in assets.clone() {
            if asset.name == asset_name {
                index = Some(count);
                break;
            }
            count = count + 1;
        }

        // Match that index in the container and remove the entry
        match index {
            Some(x) => {
                asset_container.entries.remove(x);
            }
            None => (),
        };

        asset_container.entries.push(a);
        asset_container
            .entries
            .sort_by_key(|x| x.clone().name);

        let serialized = match asset_container.write_to_bytes() {
            Ok(serialized) => serialized,
            Err(_) => {
                return Err(ApplyError::InternalError(String::from(
                    "Cannot serialize record container",
                )))
            }
        };
        let mut sets = HashMap::new();
        sets.insert(address, serialized);
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;
        Ok(())
    }


    pub fn get_account(&mut self, account_id: &str) -> Result<Option<account::Account>, ApplyError> {
        let address = make_account_address(account_id);
        let d = self.context.get_state(vec![address])?;
        match d {
            Some(packed) => {
                let accounts: account::AccountContainer =
                    match protobuf::parse_from_bytes(packed.as_slice()) {
                        Ok(accounts) => accounts,
                        Err(_) => {
                            return Err(ApplyError::InternalError(String::from(
                                "Cannot deserialize account container",
                            )))
                        }
                    };

                for account in accounts.get_entries() {
                    if account.public_key == account_id {
                        return Ok(Some(account.clone()));
                    }
                }
                Ok(None)
            }
            None => Ok(None),
        }
    }

    pub fn set_account(&mut self, account_id: &str, account: account::Account) -> Result<(), ApplyError> {
        let address = make_account_address(account_id);
        let d = self.context.get_state(vec![address.clone()])?;
        let mut accounts = match d {
            Some(packed) => match protobuf::parse_from_bytes(packed.as_slice()) {
                Ok(accounts) => accounts,
                Err(_) => {
                    return Err(ApplyError::InternalError(String::from(
                        "Cannot deserialize account container",
                    )))
                }
            },
            None => account::AccountContainer::new(),
        };

        accounts.entries.push(account);
        accounts.entries.sort_by_key(|a| a.clone().public_key);
        let serialized = match accounts.write_to_bytes() {
            Ok(serialized) => serialized,
            Err(_) => {
                return Err(ApplyError::InternalError(String::from(
                    "Cannot serialize account container",
                )))
            }
        };
        let mut sets = HashMap::new();
        sets.insert(address, serialized);
        self.context
            .set_state(sets)
            .map_err(|err| ApplyError::InternalError(format!("{}", err)))?;
        Ok(())
    }
}