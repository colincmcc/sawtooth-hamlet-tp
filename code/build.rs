extern crate protoc_rust;

use protoc_rust::Customize;

fn main() {
	protoc_rust::run(protoc_rust::Args {
	    out_dir: "src/protos",
	    input: &[
        "protos/account.proto",
        "protos/asset.proto",
        "protos/holding.proto",
        "protos/offer_history.proto",
        "protos/payload.proto",
        "protos/rule.proto",
        ],
	    includes: &["protos"],
	    customize: Customize {
        carllerche_bytes_for_bytes: Some(true),
        carllerche_bytes_for_string: Some(true),
	      ..Default::default()
	    },
	}).expect("protoc");
}