
use crypto::digest::Digest;
use crypto::sha2::Sha512;

const FAMILY_NAME: &str = "hamlet_loyalty";
const ASSET: &str = "as";
const ACCOUNT: &str = "ac";


pub fn get_hamlet_prefix() -> String {
    let mut sha = Sha512::new();
    sha.input_str(&FAMILY_NAME);
    sha.result_str()[..6].to_string()
}

pub fn hash(to_hash: &str, num: usize) -> String {
    let mut sha = Sha512::new();
    sha.input_str(to_hash);
    let temp = sha.result_str().to_string();
    let hash = match temp.get(..num) {
        Some(x) => x,
        None => "",
    };
    hash.to_string()
}

pub fn make_asset_address(asset_name: &str) -> String {
    get_hamlet_prefix() + &ASSET + &hash(asset_name, 62)
}

pub fn make_account_address(identifier: &str) -> String {
    get_hamlet_prefix() + &ACCOUNT + &hash(identifier, 62)
}
