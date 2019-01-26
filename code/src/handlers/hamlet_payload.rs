use sawtooth_sdk::processor::handler::ApplyError;
use crate::protos::*;
use protobuf;


#[derive(Debug, Clone)]
pub enum Action {
    CreateAsset(payload::CreateAsset),
    CreateAccount(payload::CreateAccount)
}

pub struct HamletPayload {
    action: Action
}

impl HamletPayload {
    pub fn new(payload: &[u8]) -> Result<Option<HamletPayload>, ApplyError> {
        let payload: payload::TransactionPayload = match protobuf::parse_from_bytes(payload) {
            Ok(payload) => payload,
            Err(_) => {
                return Err(ApplyError::InvalidTransaction(String::from(
                    "Cannot deserialize payload",
                )))
            }
        };

        let hamlet_action = payload.get_payload_type();
        let action = match hamlet_action {
            payload::TransactionPayload_PayloadType::CREATE_ASSET => {
                let create_asset = payload.get_create_asset();
                if create_asset.get_name() == "" {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Asset name cannot be an empty string",
                    )));
                }
                Action::CreateAsset(create_asset.clone())
            }
            payload::TransactionPayload_PayloadType::CREATE_ACCOUNT => {
                let create_account = payload.get_create_account();
                if create_account.get_label() == "" {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Account name cannot be an empty string",
                    )));
                }
                Action::CreateAccount(create_account.clone())
            }
            //
            // TODO: Implement below options
            //
            payload::TransactionPayload_PayloadType::CREATE_HOLDING=> {
                let create_account = payload.get_create_account();
                if create_account.get_label() == "" {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Account name cannot be an empty string",
                    )));
                }
                Action::CreateAccount(create_account.clone())
            }
            payload::TransactionPayload_PayloadType::CREATE_OFFER => {
                let create_account = payload.get_create_account();
                if create_account.get_label() == "" {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Account name cannot be an empty string",
                    )));
                }
                Action::CreateAccount(create_account.clone())
            }
            payload::TransactionPayload_PayloadType::CLOSE_OFFER => {
                let create_account = payload.get_create_account();
                if create_account.get_label() == "" {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Account name cannot be an empty string",
                    )));
                }
                Action::CreateAccount(create_account.clone())
            }
            payload::TransactionPayload_PayloadType::ACCEPT_OFFER => {
                let create_account = payload.get_create_account();
                if create_account.get_label() == "" {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Account name cannot be an empty string",
                    )));
                }
                Action::CreateAccount(create_account.clone())
            }
            payload::TransactionPayload_PayloadType::TYPE_UNSET => {
                let create_account = payload.get_create_account();
                if create_account.get_label() == "" {
                    return Err(ApplyError::InvalidTransaction(String::from(
                        "Account name cannot be an empty string",
                    )));
                }
                Action::CreateAccount(create_account.clone())
            }
        };


        Ok(Some(HamletPayload {
            action: action
        }))
    }

    pub fn get_action(&self) -> Action {
        self.action.clone()
    }


}