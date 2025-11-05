extern crate elements;
extern crate hal;
extern crate hex;
extern crate serde;

pub mod address;
pub mod block;
pub mod tx;

pub mod confidential;

pub use hal::HexBytes;
pub use elements::bitcoin;

use elements::AddressParams;
use serde::{Deserialize, Serialize};

/// Known Elements networks.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Network {
	ElementsRegtest,
	Liquid,
	LiquidTestnet,
}

impl Network {
	pub fn from_params(params: &AddressParams) -> Option<Network> {
		// Compare HRP and prefixes since AddressParams instances may not be pointer-equal
		// even if they represent the same network
		if params.bech_hrp == AddressParams::ELEMENTS.bech_hrp &&
		   params.p2pkh_prefix == AddressParams::ELEMENTS.p2pkh_prefix {
			Some(Network::ElementsRegtest)
		} else if params.bech_hrp == AddressParams::LIQUID.bech_hrp &&
		          params.p2pkh_prefix == AddressParams::LIQUID.p2pkh_prefix {
			Some(Network::Liquid)
		} else if params.bech_hrp == AddressParams::LIQUID_TESTNET.bech_hrp &&
		          params.p2pkh_prefix == AddressParams::LIQUID_TESTNET.p2pkh_prefix {
			Some(Network::LiquidTestnet)
		} else {
			None
		}
	}

	pub fn address_params(self) -> &'static AddressParams {
		match self {
			Network::ElementsRegtest => &AddressParams::ELEMENTS,
			Network::Liquid => &AddressParams::LIQUID,
			Network::LiquidTestnet => &AddressParams::LIQUID_TESTNET,
		}
	}
}

/// Get JSON-able objects that describe the type.
pub trait GetInfo<T: ::serde::Serialize> {
	/// Get a description of this object given the network of interest.
	fn get_info(&self, network: Network) -> T;
}
