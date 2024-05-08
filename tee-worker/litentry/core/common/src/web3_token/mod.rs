// Copyright 2020-2024 Trust Computing GmbH.
// This file is part of Litentry.
//
// Litentry is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Litentry is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Litentry. If not, see <https://www.gnu.org/licenses/>.

#[cfg(all(feature = "std", feature = "sgx"))]
compile_error!("feature \"std\" and feature \"sgx\" cannot be enabled at the same time");

#[cfg(all(not(feature = "std"), feature = "sgx"))]
extern crate sgx_tstd as std;

use std::vec::Vec;

use litentry_primitives::Web3TokenType;

use crate::Web3Network;

const ETH_AMOUNT_RNAGE: [f64; 10] = [
	0.0,
	0.01,
	0.05,
	0.2,
	0.6,
	1.2,
	3.0,
	8.0,
	20.0,
	50.0,
];
const USDC_AMOUNT_RANGE: [f64; 9] = [
	0.0,
	10.0,
	30.0,
	80.0,
	200.0,
	500.0,
	1_000.0,
	2_000.0,
	5_000.0,
];
const ADA_AMOUNT_RANGE: [f64; 7] = [
	0.0,
	1_000.0,
	5_000.0,
	20_000.0,
	50_000.0,
	100_000.0,
	300_000.0,
];
const DOGE_AMOUNT_RANGE: [f64; 7] = [
	0.0,
	1_000.0,
	5_000.0,
	20_000.0,
	50_000.0,
	100_000.0,
	300_000.0,
];
const SHIB_AMOUNT_RANGE: [f64; 8] = [
	0.0,
	400_000.0,
	2_000_000.0,
	10_000_000.0,
	20_000_000.0,
	40_000_000.0,
	100_000_000.0,
	200_000_000.0,
];
const UNI_AMOUNT_RANGE: [f64; 9] = [
	0.0,
	10.0,
	30.0,
	80.0,
	200.0,
	500.0,
	1_000.0,
	2_000.0,
	5_000.0,
];
const BCH_AMOUNT_RANGE: [f64; 6] = [
	0.0,
	0.1,
	0.5,
	2.0,
	6.0,
	12.0,
];
const ETC_AMOUNT_RANGE: [f64; 6] = [
	0.0,
	1.0,
	5.0,
	20.0,
	50.0,
	80.0,
];
const ATOM_AMOUNT_RANGE: [f64; 6] = [
	0.0,
	1.0,
	5.0,
	20.0,
	50.0,
	80.0,
];
const DAI_AMOUNT_RANGE: [f64; 9] = [
	0.0,
	10.0,
	30.0,
	80.0,
	200.0,
	500.0,
	1_000.0,
	2_000.0,
	5_000.0,
];
const LEO_AMOUNT_RANGE: [f64; 6] = [
	0.0,
	10.0,
	30.0,
	80.0,
	200.0,
	500.0,
];
const FIL_AMOUNT_RANGE: [f64; 6] = [
	0.0,
	10.0,
	30.0,
	80.0,
	200.0,
	500.0,
];
const IMX_AMOUNT_RANGE: [f64; 8] = [
	0.0,
	10.0,
	30.0,
	80.0,
	200.0,
	500.0,
	1_000.0,
	2_000.0,
];
const CRO_AMOUNT_RANGE: [f64; 7] = [
	0.0,
	1_000.0,
	5_000.0,
	20_000.0,
	50_000.0,
	100_000.0,
	300_000.0,
];
const INJ_AMOUNT_RANGE: [f64; 6] = [
	0.0,
	1.0,
	5.0,
	20.0,
	50.0,
	80.0,
];

pub trait TokenName {
	fn get_token_name(&self) -> &'static str;
}

impl TokenName for Web3TokenType {
	fn get_token_name(&self) -> &'static str {
		match self {
			Self::Bnb => "BNB",
			Self::Eth => "ETH",
			Self::SpaceId => "SPACE_ID",
			Self::Lit => "LIT",
			Self::Wbtc => "WBTC",
			Self::Usdc => "USDC",
			Self::Usdt => "USDT",
			Self::Crv => "CRV",
			Self::Matic => "MATIC",
			Self::Dydx => "DYDX",
			Self::Amp => "AMP",
			Self::Cvx => "CVX",
			Self::Tusd => "TUSD",
			Self::Usdd => "USDD",
			Self::Gusd => "GUSD",
			Self::Link => "LINK",
			Self::Grt => "GRT",
			Self::Comp => "COMP",
			Self::People => "PEOPLE",
			Self::Gtc => "GTC",
			Self::Ton => "TON",
			Self::Trx => "TRX",
			Self::Nfp => "NFP",
			Self::Sol => "SOL",
			Self::Mcrt => "MCRT",
			Self::Btc => "BTC",
			Self::Ada => "ADA",
			Self::Doge => "DOGE",
			Self::Shib => "SHIB",
			Self::Uni => "UNI",
			Self::Bch => "BCH",
			Self::Etc => "ETC",
			Self::Atom => "ATOM",
			Self::Dai => "DAI",
			Self::Leo => "LEO",
			Self::Fil => "FIL",
			Self::Imx => "IMX",
			Self::Cro => "CRO",
			Self::Inj => "INJ",
		}
	}
}

pub trait TokenAddress {
	fn get_token_address(&self, network: Web3Network) -> Option<&'static str>;
}

impl TokenAddress for Web3TokenType {
	fn get_token_address(&self, network: Web3Network) -> Option<&'static str> {
		match (self, network) {
			// Bnb
			(Self::Bnb, Web3Network::Ethereum) =>
				Some("0xb8c77482e45f1f44de1745f52c74426c631bdd52"),
			// Eth
			(Self::Eth, Web3Network::Bsc) => Some("0x2170ed0880ac9a755fd29b2688956bd959f933f8"),
			// SpaceId
			(Self::SpaceId, Web3Network::Bsc) | (Self::SpaceId, Web3Network::Ethereum) =>
				Some("0x2dff88a56767223a5529ea5960da7a3f5f766406"),
			// Lit
			(Self::Lit, Web3Network::Bsc) | (Self::Lit, Web3Network::Ethereum) =>
				Some("0xb59490ab09a0f526cc7305822ac65f2ab12f9723"),
			// Wbtc
			(Self::Wbtc, Web3Network::Ethereum) =>
				Some("0x2260fac5e5542a773aa44fbcfedf7c193bc2c599"),
			// Usdc
			(Self::Usdc, Web3Network::Bsc) => Some("0x8ac76a51cc950d9822d68b83fe1ad97b32cd580d"),
			(Self::Usdc, Web3Network::Ethereum) =>
				Some("0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"),
			(Self::Usdc, Web3Network::Solana) => Some("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
			(Self::Usdc, Web3Network::Arbitrum) => Some("0xaf88d065e77c8cC2239327C5EDb3A432268e5831"),
			(Self::Usdc, Web3Network::Polygon) => Some("0x3c499c542cEF5E3811e1192ce70d8cC03d5c3359"),

			// Usdt
			(Self::Usdt, Web3Network::Bsc) => Some("0x55d398326f99059ff775485246999027b3197955"),
			(Self::Usdt, Web3Network::Ethereum) =>
				Some("0xdac17f958d2ee523a2206206994597c13d831ec7"),
			// Crv
			(Self::Crv, Web3Network::Ethereum) =>
				Some("0xd533a949740bb3306d119cc777fa900ba034cd52"),
			// Matic
			(Self::Matic, Web3Network::Bsc) => Some("0xcc42724c6683b7e57334c4e856f4c9965ed682bd"),
			(Self::Matic, Web3Network::Ethereum) =>
				Some("0x7d1afa7b718fb893db30a3abc0cfc608aacfebb0"),
			// Dydx
			(Self::Dydx, Web3Network::Ethereum) =>
				Some("0x92d6c1e31e14520e676a687f0a93788b716beff5"),
			// Amp
			(Self::Amp, Web3Network::Ethereum) =>
				Some("0xff20817765cb7f73d4bde2e66e067e58d11095c2"),
			// Cvx
			(Self::Cvx, Web3Network::Ethereum) =>
				Some("0x4e3fbd56cd56c3e72c1403e103b45db9da5b9d2b"),
			// Tusd
			(Self::Tusd, Web3Network::Bsc) => Some("0x40af3827F39D0EAcBF4A168f8D4ee67c121D11c9"),
			(Self::Tusd, Web3Network::Ethereum) =>
				Some("0x0000000000085d4780b73119b644ae5ecd22b376"),
			// Usdd
			(Self::Usdd, Web3Network::Bsc) => Some("0xd17479997f34dd9156deef8f95a52d81d265be9c"),
			(Self::Usdd, Web3Network::Ethereum) =>
				Some("0x0c10bf8fcb7bf5412187a595ab97a3609160b5c6"),
			// Gusd
			(Self::Gusd, Web3Network::Ethereum) =>
				Some("0x056fd409e1d7a124bd7017459dfea2f387b6d5cd"),
			// Link
			(Self::Link, Web3Network::Bsc) => Some("0xf8a0bf9cf54bb92f17374d9e9a321e6a111a51bd"),
			(Self::Link, Web3Network::Ethereum) =>
				Some("0x514910771af9ca656af840dff83e8264ecf986ca"),
			// Grt
			(Self::Grt, Web3Network::Bsc) => Some("0x52ce071bd9b1c4b00a0b92d298c512478cad67e8"),
			(Self::Grt, Web3Network::Ethereum) =>
				Some("0xc944e90c64b2c07662a292be6244bdf05cda44a7"),
			// Comp
			(Self::Comp, Web3Network::Ethereum) =>
				Some("0xc00e94cb662c3520282e6f5717214004a7f26888"),
			// People
			(Self::People, Web3Network::Ethereum) =>
				Some("0x7a58c0be72be218b41c608b7fe7c5bb630736c71"),
			// Gtc
			(Self::Gtc, Web3Network::Ethereum) =>
				Some("0xde30da39c46104798bb5aa3fe8b9e0e1f348163f"),
			// Ton
			(Self::Ton, Web3Network::Bsc) => Some("0x76a797a59ba2c17726896976b7b3747bfd1d220f"),
			(Self::Ton, Web3Network::Ethereum) =>
				Some("0x582d872a1b094fc48f5de31d3b73f2d9be47def1"),
			// Trx
			(Self::Trx, Web3Network::Bsc) => Some("0xCE7de646e7208a4Ef112cb6ed5038FA6cC6b12e3"),
			(Self::Trx, Web3Network::Ethereum) =>
				Some("0x50327c6c5a14dcade707abad2e27eb517df87ab5"),
			// Nfp
			(Self::Nfp, Web3Network::Bsc) => Some("0x75e8ddb518bb757b4282cd5b83bb70d4101d12fb"),
			// Sol
			(Self::Sol, Web3Network::Bsc) => Some("0x570a5d26f7765ecb712c0924e4de545b89fd43df"),
			(Self::Sol, Web3Network::Ethereum) =>
				Some("0x5288738df1aeb0894713de903e1d0c001eeb7644"),
			// Mcrt
			(Self::Mcrt, Web3Network::Bsc) => Some("0x4b8285aB433D8f69CB48d5Ad62b415ed1a221e4f"),
			(Self::Mcrt, Web3Network::Ethereum) =>
				Some("0xde16ce60804a881e9f8c4ebb3824646edecd478d"),
			(Self::Mcrt, Web3Network::Solana) =>
				Some("FADm4QuSUF1K526LvTjvbJjKzeeipP6bj5bSzp3r6ipq"),
			
			// Ada
			(Self::Ada, Web3Network::Bsc) => Some("0xaf88d065e77c8cC2239327C5EDb3A432268e5831"),

			// Doge
			(Self::Doge, Web3Network::Bsc) => Some("0xba2ae424d960c26247dd6c32edc70b295c744c43"),

			// Shib
			(Self::Shib, Web3Network::Ethereum) => Some("0x95ad61b0a150d79219dcf64e1e6cc01f0b64c4ce"),

			// Uni
			(Self::Uni, Web3Network::Ethereum) => Some("0x1f9840a85d5af5bf1d1762f925bdaddc4201f984"),
			(Self::Uni, Web3Network::Bsc) => Some("0xbf5140a22578168fd562dccf235e5d43a02ce9b1"),
			(Self::Uni, Web3Network::Solana) => Some("8FU95xFJhUUkyyCLU13HSzDLs7oC4QZdXQHL6SCeab36"),
			(Self::Uni, Web3Network::Arbitrum) => Some("0xFa7F8980b0f1E64A2062791cc3b0871572f1F7f0"),
			(Self::Uni, Web3Network::Polygon) => Some("0xb33eaad8d922b1083446dc23f610c2567fb5180f"),

			// Bch
			(Self::Bch, Web3Network::Bsc) => Some("0x8fF795a6F4D97E7887C79beA79aba5cc76444aDf"),

			// Etc
			(Self::Etc, Web3Network::Bsc) => Some("0x3d6545b08693dae087e957cb1180ee38b9e3c25e"),

			// Atom
			(Self::Atom, Web3Network::Ethereum) => Some("0x8D983cb9388EaC77af0474fA441C4815500Cb7BB"),
			(Self::Atom, Web3Network::Bsc) => Some("0x0eb3a705fc54725037cc9e008bdede697f62f335"),
			(Self::Atom, Web3Network::Polygon) => Some("0xac51C4c48Dc3116487eD4BC16542e27B5694Da1b"),

			// Dai
			(Self::Dai, Web3Network::Ethereum) => Some("0x6b175474e89094c44da98b954eedeac495271d0f"),
			(Self::Dai, Web3Network::Bsc) => Some("0x1af3f329e8be154074d8769d1ffa4ee058b1dbc3"),
			(Self::Dai, Web3Network::Polygon) => Some("0x8f3Cf7ad23Cd3CaDbD9735AFf958023239c6A063"),
			(Self::Dai, Web3Network::Solana) => Some("EjmyN6qEC1Tf1JxiG1ae7UTJhUxSwk1TCWNWqxWV4J6o"),
			(Self::Dai, Web3Network::Arbitrum) => Some("0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1"),

			// Leo
			(Self::Leo, Web3Network::Ethereum) => Some("0x2af5d2ad76741191d15dfe7bf6ac92d4bd912ca3"),

			// Fil
			(Self::Fil, Web3Network::Bsc) => Some("0x0d8ce2a99bb6e3b7db580ed848240e4a0f9ae153"),

			// Imx
			(Self::Imx, Web3Network::Ethereum) => Some("0xf57e7e7c23978c3caec3c3548e3d615c346e79ff"),

			// Cro
			(Self::Cro, Web3Network::Ethereum) => Some("0xa0b73e1ff0b80914ab6fe0444e65848c4c34450b"),
			(Self::Cro, Web3Network::Solana) => Some("DvjMYMVeXgKxaixGKpzQThLoG98nc7HSU7eanzsdCboA"),

			// Inj
			(Self::Inj, Web3Network::Ethereum) => Some("0xe28b3b32b6c345a34ff64674606124dd5aceca30"),
			(Self::Inj, Web3Network::Bsc) => Some("0xa2b726b1145a4773f68593cf171187d8ebe4d495"),

			_ => None,
		}
	}
}

pub trait TokenDecimals {
	fn get_decimals(&self, network: Web3Network) -> u64;
}

impl TokenDecimals for Web3TokenType {
	fn get_decimals(&self, network: Web3Network) -> u64 {
		let decimals = match (self, network) {
			// Bnb
			(Self::Bnb, Web3Network::Bsc) | (Self::Bnb, Web3Network::Ethereum) |
			// Eth
			(Self::Eth, Web3Network::Bsc) | (Self::Eth, Web3Network::Ethereum) |
			// SpaceId
			(Self::SpaceId, Web3Network::Bsc) | (Self::SpaceId, Web3Network::Ethereum) |
			// Lit
			(Self::Lit, Web3Network::Bsc) | (Self::Lit, Web3Network::Ethereum) |
			// Usdc
			(Self::Usdc, Web3Network::Bsc) |
			// Usdt
			(Self::Usdt, Web3Network::Bsc) |
			// Crv
			(Self::Crv, Web3Network::Ethereum) |
			// Matic
			(Self::Matic, Web3Network::Bsc) | (Self::Matic, Web3Network::Ethereum) |
			// Dydx
			(Self::Dydx, Web3Network::Ethereum) |
			// Amp
			(Self::Amp, Web3Network::Ethereum) |
			// Cvx
			(Self::Cvx, Web3Network::Ethereum) |
			// Tusd
			(Self::Tusd, Web3Network::Bsc) | (Self::Tusd, Web3Network::Ethereum) |
			// Usdd
			(Self::Usdd, Web3Network::Bsc) | (Self::Usdd, Web3Network::Ethereum) |
			// Link
			(Self::Link, Web3Network::Bsc) | (Self::Link, Web3Network::Ethereum) |
			// Grt
			(Self::Grt, Web3Network::Bsc) | (Self::Grt, Web3Network::Ethereum) |
			// Comp
			(Self::Comp, Web3Network::Ethereum) |
			// People
			(Self::People, Web3Network::Ethereum) |
			// Gtc
			(Self::Gtc, Web3Network::Ethereum) |
			// Nfp
			(Self::Nfp, Web3Network::Bsc) |
			// Sol
			(Self::Sol, Web3Network::Bsc) | (Self::Sol, Web3Network::Ethereum) |
			
			// Ada
			(Self::Ada, Web3Network::Bsc) |

			// Shib
			(Self::Shib, Web3Network::Ethereum) |

			// Uni
			(Self::Uni, Web3Network::Ethereum) | (Self::Uni, Web3Network::Bsc) | (Self::Uni, Web3Network::Arbitrum) | (Self::Uni, Web3Network::Polygon) |

			// Bch
			(Self::Bch, Web3Network::Bsc) |

			// Etc
			(Self::Etc, Web3Network::Bsc) |

			// Atom
			(Self::Atom, Web3Network::Bsc) |

			// Dai
			(Self::Dai, Web3Network::Ethereum) | (Self::Dai, Web3Network::Bsc) | (Self::Dai, Web3Network::Polygon) | (Self::Dai, Web3Network::Arbitrum) |

			// Leo
			(Self::Leo, Web3Network::Ethereum) |

			// Fil
			(Self::Fil, Web3Network::Bsc) |

			// Imx
			(Self::Imx, Web3Network::Ethereum) |

			// Inj
			(Self::Inj, Web3Network::Ethereum) | (Self::Inj, Web3Network::Bsc)

			=> 18,

			// Ton
			(Self::Ton, Web3Network::Bsc) | (Self::Ton, Web3Network::Ethereum) |
			// Mcrt
			(Self::Mcrt, Web3Network::Bsc) | (Self::Mcrt, Web3Network::Ethereum) => 9,

			// Wbtc
			(Self::Wbtc, Web3Network::Bsc) | (Self::Wbtc, Web3Network::Ethereum) |
			// Mcrt
			(Self::Mcrt, Web3Network::Solana) |
			// Btc
			(Self::Btc, Web3Network::BitcoinP2tr) | (Self::Btc, Web3Network::BitcoinP2pkh) |
			 (Self::Btc, Web3Network::BitcoinP2sh) | (Self::Btc, Web3Network::BitcoinP2wpkh) |
			  (Self::Btc, Web3Network::BitcoinP2wsh) |
			
			// Doge
			(Self::Doge, Web3Network::Bsc) |

			// Uni
			(Self::Uni, Web3Network::Solana) |

			// Dai
			(Self::Dai, Web3Network::Solana) |

			// Cro
			(Self::Cro, Web3Network::Ethereum) | (Self::Cro, Web3Network::Solana)
			
			=> 8,

			// Usdc
			(Self::Usdc, Web3Network::Solana) |
			(Self::Usdc, Web3Network::Arbitrum) |
			(Self::Usdc, Web3Network::Polygon) |
			(Self::Usdc, Web3Network::Ethereum) |
			// Usdt
			(Self::Usdt, Web3Network::Ethereum) |
			// Trx
			(Self::Trx, Web3Network::Bsc) | (Self::Trx, Web3Network::Ethereum) |
			
			// Atom
			(Self::Atom, Web3Network::Ethereum) | (Self::Atom, Web3Network::Polygon)

			=> 6,


			// Gusd
			(Self::Gusd, Web3Network::Ethereum) => 2,
			_ => 1,
		};

		10_u64.pow(decimals)
	}
}

pub trait TokenHoldingAmountRange {
	fn get_token_holding_amount_range(&self) -> Vec<f64>;
}

impl TokenHoldingAmountRange for Web3TokenType {
	fn get_token_holding_amount_range(&self) -> Vec<f64> {
		match self {
			Self::Mcrt => vec![0.0, 2000.0, 10000.0, 50000.0, 150000.0, 500000.0],

			// Eth
			Self::Eth => ETH_AMOUNT_RNAGE.to_vec(),

			// Usdc
			Self::Usdc => USDC_AMOUNT_RANGE.to_vec(),
			
			// Ada
			Self::Ada => ADA_AMOUNT_RANGE.to_vec(),

			// Doge
			Self::Doge => DOGE_AMOUNT_RANGE.to_vec(),

			// Shib
			Self::Shib => SHIB_AMOUNT_RANGE.to_vec(),

			// Uni
			Self::Uni => UNI_AMOUNT_RANGE.to_vec(),

			// Bch
			Self::Bch => BCH_AMOUNT_RANGE.to_vec(),

			// Etc
			Self::Etc => ETC_AMOUNT_RANGE.to_vec(),

			// Atom
			Self::Atom => ATOM_AMOUNT_RANGE.to_vec(),

			// Dai
			Self::Dai => DAI_AMOUNT_RANGE.to_vec(),

			// Leo
			Self::Leo => LEO_AMOUNT_RANGE.to_vec(),

			// Fil
			Self::Fil => FIL_AMOUNT_RANGE.to_vec(),

			// Imx
			Self::Imx => IMX_AMOUNT_RANGE.to_vec(),

			// Cro
			Self::Cro => CRO_AMOUNT_RANGE.to_vec(),

			// Inj
			Self::Inj => INJ_AMOUNT_RANGE.to_vec(),

			_ => vec![0.0, 1.0, 50.0, 100.0, 200.0, 500.0, 800.0, 1200.0, 1600.0, 3000.0],
		}
	}
}
