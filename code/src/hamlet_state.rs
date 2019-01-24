use crypto::digest::Digest;
use crypto::sha2::Sha512;
use sawtooth_sdk::processor::handler::ApplyError;
use sawtooth_sdk::processor::handler::TransactionContext;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::str::from_utf8;
use bytes::{Bytes, BytesMut, Buf, BufMut};

use crate::protos::asset::{Asset, AssetContainer,};

pub fn get_hamlet_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str("hamlet");
    sha.result_str()[..6].to_string()
}

pub struct HamletState<'a> {
    context: &'a mut TransactionContext,
    address_map: HashMap<String, Option<String>>,
}

impl<'a> HamletState<'a> {
    pub fn new(context: &'a mut TransactionContext) -> HamletState {
        HamletState {
            context,
            address_map: HashMap::new(),
        }
    }

    fn calculate_address(name: &str) -> String {
        let mut sha = Sha512::new();
        sha.input_str(name);
        get_hamlet_prefix() + &sha.result_str()[..64].to_string()
    }




    pub fn get_asset(&mut self, asset_name: &str) -> Result<Option<Asset>, ApplyError> {
        //let assets= self._load_assets(asset_name)?;
        let container = self._get_asset_container(asset_name)?;
        let asset = self._get_asset_from_container(asset_name, container)?;
        OK(asset)
    }

    pub fn set_asset(&mut self, asset_name: &str, a: Asset) -> Result<(), ApplyError> {
        let container = self._get_asset_container(asset_name)?;
        let asset = self._get_asset_from_container(asset_name,container)?;

        let mut assets = self._load_assets(asset_name)?;
        assets.insert(asset_name.to_string(), a);
        self._store_asset(asset_name, assets)?;
        Ok(())
    }

    pub fn _get_asset_container(&mut self, asset_name: &str) -> Result<AssetContainer, ApplyError> {
        let address = HamletState::calculate_address(asset_name);

        Ok(match self.address_map.entry(address.clone()) {
            Entry::Occupied(entry) => match entry.get() {
                Some(addr) => Bytes::from(addr).ok_or_else(|| {
                    ApplyError::InvalidTransaction("Invalid serialization of asset state".into())
                })?,
                None => AssetContainer::new(),
            },
            Entry::Vacant(entry) => match self.context.get_state(vec![address])? {
                Some(state_bytes) => {
                    let state_string = from_utf8(&state_bytes).map_err(|e| {
                        ApplyError::InvalidTransaction(format!(
                            "Invalid serialization of asset state: {}",
                            e
                        ))
                    })?;

                    entry.insert(Some(state_string.to_string()));
                    Bytes::from(state_string).ok_or_else(|| {
                        ApplyError::InvalidTransaction("Invalid serialization of asset state".into())
                    })?
                }
                None => {
                    entry.insert(None);
                    AssetContainer::new()
                }
            },
        })
    }
    fn _get_asset_from_container(&mut self, asset_name: &str, asset_container: AssetContainer) -> Result<Asset, ApplyError>{
        let assets = asset_container.get_entries();
        if assets.contains_key(asset_name) {
            Ok(Some(assets[asset_name].clone()))
        } else {
            Ok(Asset::new())
        }
    }
    fn _load_assets(&mut self, asset_name: &str) -> Result<HashMap<String, Asset>, ApplyError> {
        let address = HamletState::calculate_address(asset_name);

        Ok(match self.address_map.entry(address.clone()) {
            Entry::Occupied(entry) => match entry.get() {
                Some(addr) => Asset::deserialize_assets(addr).ok_or_else(|| {
                    ApplyError::InvalidTransaction("Invalid serialization of asset state".into())
                })?,
                None => HashMap::new(),
            },
            Entry::Vacant(entry) => match self.context.get_state(vec![address])? {
                Some(state_bytes) => {
                    let state_string = from_utf8(&state_bytes).map_err(|e| {
                        ApplyError::InvalidTransaction(format!(
                            "Invalid serialization of asset state: {}",
                            e
                        ))
                    })?;

                    entry.insert(Some(state_string.to_string()));
                    Asset::deserialize_games(state_string).ok_or_else(|| {
                        ApplyError::InvalidTransaction("Invalid serialization of asset state".into())
                    })?
                }
                None => {
                    entry.insert(None);
                    HashMap::new()
                }
            },
        })
    }
}