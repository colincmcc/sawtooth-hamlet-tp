use sawtooth_sdk::messages::processor::TpProcessRequest;
use sawtooth_sdk::processor::handler::{ApplyError, TransactionContext, TransactionHandler};
use protobuf::{Message, RepeatedField};
use protobuf;
use std::collections::HashMap;

use crate::protos::*;


use crate::handlers::hamlet_state::HamletState;
use crate::handlers::hamlet_payload::{HamletPayload, Action};
use crate::handlers::hamlet_handler;
use crate::addressing::get_hamlet_prefix;

pub struct HamletTransactionHandler {
    family_name: String,
    family_versions: Vec<String>,
    namespaces: Vec<String>,
}

impl HamletTransactionHandler {
    pub fn new() -> HamletTransactionHandler {
        HamletTransactionHandler {
            family_name: "hamlet_loyalty".into(),
            family_versions: vec!["1.0".into()],
            namespaces: vec![get_hamlet_prefix()],
        }
    }
    fn _create_account(
        &self,
        payload: payload::CreateAccount,
        mut state: HamletState,
        signer: &str
    ) -> Result<(), ApplyError> {
        let label = payload.get_label();
        let description = payload.get_description();
        match state.get_account(signer) {
            Ok(Some(_)) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Account already exists with label {}",
                    label
                )))
            }
            Ok(None) => (),
            Err(err) => return Err(err),
        }

        let mut new_account = account::Account::new();
        new_account.set_public_key(signer.to_string());
        new_account.set_label(label.to_string());
        new_account.set_description(description.to_string());
        new_account.set_holdings(RepeatedField::from_vec(vec![]));

        state.set_account(signer, new_account)?;
        Ok(())
    }

    fn _create_asset(
        &self,
        payload: payload::CreateAsset,
        mut state: HamletState,
        signer: &str
    ) -> Result<(), ApplyError> {
        match state.get_account(signer) {
            Ok(Some(_)) => (),
            Ok(None) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Account is not registered: {}",
                    signer
                )))
            }
            Err(err) => return Err(err),
        }

        let asset_name = payload.get_name();
        let asset_description = payload.get_description();
        let asset_rules = payload.get_rules().to_vec();


        match state.get_asset(asset_name) {
            Ok(Some(_)) => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Asset with this name already exists: {}",
                    asset_name
                )))
            }
            Ok(None) => (),
            Err(err) => return Err(err),
        }

        let mut new_asset = asset::Asset::new();
        new_asset.set_description(asset_description.to_string());
        new_asset.set_name(asset_name.to_string());
        new_asset.set_owners(RepeatedField::from_vec(vec![signer.to_string()]));
        //new_asset.set_rules(asset_rules);

        state.set_asset(asset_name, new_asset)?;
        Ok(())
    }

}

impl TransactionHandler for HamletTransactionHandler {
    fn family_name(&self) -> String {
        self.family_name.clone()
    }

    fn family_versions(&self) -> Vec<String> {
        self.family_versions.clone()
    }

    fn namespaces(&self) -> Vec<String> {
        self.namespaces.clone()
    }

    fn apply(
        &self,
        request: &TpProcessRequest,
        context: &mut TransactionContext,
    ) -> Result<(), ApplyError> {
        let payload = HamletPayload::new(request.get_payload());
        let payload = match payload {
            Err(e) => return Err(e),
            Ok(payload) => payload,
        };
        let payload = match payload {
            Some(x) => x,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Request must contain a payload",
                )))
            }
        };
        let signer = request.get_header().get_signer_public_key();
        /*
        let signer = match &header.as_ref() {
            Some(s) => &s.signer_public_key,
            None => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Invalid header",
                )))
            }
        };
*/


        let mut state = HamletState::new(context);

        info!(
            "Payload: {:?} {} {}",
            payload.get_action(),
            request.get_header().get_inputs()[0],
            request.get_header().get_outputs()[0]
        );

        match payload.get_action() {
            Action::CreateAccount(account_payload) => {
                self._create_account(account_payload, state, signer)?
            }
            Action::CreateAsset(asset_payload) => {
                self._create_asset(asset_payload, state, signer)?
            }
            /*
            other_action => {
                return Err(ApplyError::InvalidTransaction(format!(
                    "Invalid action: '{}'",
                    other_action
                )));
            }
            */
        }

        Ok(())
    }
}