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
// along with Litentry.  If not, see <https://www.gnu.org/licenses/>.

// SPDX-License-Identifier: GPL-3.0-or-later

pragma solidity ^0.8.8;

import { TokenQueryLogic } from "./TokenQueryLogic.sol";
import "../libraries/Identities.sol";

// brc20
import { Btcs } from "./brc20/Btcs.sol";
import { Cats } from "./brc20/Cats.sol";
import { Long } from "./brc20/Long.sol";
import { Mmss } from "./brc20/Mmss.sol";
import { Ordi } from "./brc20/Ordi.sol";
import { Rats } from "./brc20/Rats.sol";
import { Sats } from "./brc20/Sats.sol";

// erc20
import { Ada } from "./erc20/Ada.sol";
import { Amp } from "./erc20/Amp.sol";
import { Atom } from "./erc20/Atom.sol";
import { Bch } from "./erc20/Bch.sol";
import { Bean } from "./erc20/Bean.sol";
import { Bnb } from "./erc20/Bnb.sol";
import { Comp } from "./erc20/Comp.sol";
import { Cro } from "./erc20/Cro.sol";
import { Crv } from "./erc20/Crv.sol";
import { Dai } from "./erc20/Dai.sol";
import { Doge } from "./erc20/Doge.sol";
import { Dydx } from "./erc20/Dydx.sol";
import { Etc } from "./erc20/Etc.sol";
import { Eth } from "./erc20/Eth.sol";
import { Fil } from "./erc20/Fil.sol";
import { Grt } from "./erc20/Grt.sol";
import { Gtc } from "./erc20/Gtc.sol";
import { Gusd } from "./erc20/Gusd.sol";
import { Imx } from "./erc20/Imx.sol";
import { Inj } from "./erc20/Inj.sol";
import { Leo } from "./erc20/Leo.sol";
import { Link } from "./erc20/Link.sol";
import { Lit } from "./erc20/Lit.sol";
import { Matic } from "./erc20/Matic.sol";
import { Mcrt } from "./erc20//Mcrt.sol";
import { Nfp } from "./erc20/Nfp.sol";
import { People } from "./erc20/People.sol";
import { Shib } from "./erc20//Shib.sol";
import { Sol } from "./erc20/Sol.sol";
import { SpaceId } from "./erc20/SpaceId.sol";
import { Ton } from "./erc20/Ton.sol";
import { Trx } from "./erc20/Trx.sol";
import { Tusd } from "./erc20/Tusd.sol";
import { Uni } from "./erc20/Uni.sol";
import { Usdc } from "./erc20/Usdc.sol";
import { Usdt } from "./erc20/Usdt.sol";
import { Wbtc } from "./erc20//Wbtc.sol";
import { Cvx } from "./erc20/Cvx.sol";
import { Usdd } from "./erc20/Usdd.sol";
contract TokenMapping is TokenQueryLogic {
	constructor() {
		// btcs
		tokenNames["btcs"] = Btcs.getTokenName();
		tokenRanges["btcs"] = Btcs.getTokenRanges();
		tokenNetworks["btcs"] = Btcs.getTokenNetworks();

		// cats
		tokenNames["cats"] = Cats.getTokenName();
		tokenRanges["cats"] = Cats.getTokenRanges();
		tokenNetworks["cats"] = Cats.getTokenNetworks();

		// long
		tokenNames["long"] = Long.getTokenName();
		tokenRanges["long"] = Long.getTokenRanges();
		tokenNetworks["long"] = Long.getTokenNetworks();

		// long
		tokenNames["mmss"] = Mmss.getTokenName();
		tokenRanges["mmss"] = Mmss.getTokenRanges();
		tokenNetworks["mmss"] = Mmss.getTokenNetworks();

		// ordi
		tokenNames["ordi"] = Ordi.getTokenName();
		tokenRanges["ordi"] = Ordi.getTokenRanges();
		tokenNetworks["ordi"] = Ordi.getTokenNetworks();

		// rats
		tokenNames["rats"] = Rats.getTokenName();
		tokenRanges["rats"] = Rats.getTokenRanges();
		tokenNetworks["rats"] = Rats.getTokenNetworks();

		// sats
		tokenNames["sats"] = Sats.getTokenName();
		tokenRanges["sats"] = Sats.getTokenRanges();
		tokenNetworks["sats"] = Sats.getTokenNetworks();

		// ada
		tokenNames["ada"] = Ada.getTokenName();
		tokenRanges["ada"] = Ada.getTokenRanges();
		tokenNetworks["ada"] = Ada.getTokenNetworks();
		tokenAddresses["ada"][Web3Networks.Bsc] = Ada.getTokenBscAddress();
		tokenAddresses["ada"][Web3Networks.Ethereum] = Ada
			.getTokenEthereumAddress();

		// amp
		tokenNames["amp"] = Amp.getTokenName();
		tokenRanges["amp"] = Amp.getTokenRanges();
		tokenNetworks["amp"] = Amp.getTokenNetworks();
		tokenAddresses["amp"][Web3Networks.Bsc] = Amp.getTokenBscAddress();
		tokenAddresses["amp"][Web3Networks.Ethereum] = Amp
			.getTokenEthereumAddress();

		// atom
		tokenNames["atom"] = Atom.getTokenName();
		tokenRanges["atom"] = Atom.getTokenRanges();
		tokenNetworks["atom"] = Atom.getTokenNetworks();
		tokenAddresses["atom"][Web3Networks.Bsc] = Atom.getTokenBscAddress();
		tokenAddresses["atom"][Web3Networks.Ethereum] = Atom
			.getTokenEthereumAddress();

		// bch
		tokenNames["bch"] = Bch.getTokenName();
		tokenRanges["bch"] = Bch.getTokenRanges();
		tokenNetworks["bch"] = Bch.getTokenNetworks();
		tokenAddresses["bch"][Web3Networks.Bsc] = Bch.getTokenBscAddress();
		tokenAddresses["bch"][Web3Networks.Ethereum] = Bch
			.getTokenEthereumAddress();

		// bean
		tokenNames["bean"] = Bean.getTokenName();
		tokenRanges["bean"] = Bean.getTokenRanges();
		tokenNetworks["bean"] = Bean.getTokenNetworks();
		tokenAddresses["bean"][Web3Networks.Bsc] = Bean.getTokenBscAddress();
		tokenAddresses["bean"][Web3Networks.Ethereum] = Bean
			.getTokenEthereumAddress();

		// bnb
		tokenNames["bnb"] = Bnb.getTokenName();
		tokenRanges["bnb"] = Bnb.getTokenRanges();
		tokenNetworks["bnb"] = Bnb.getTokenNetworks();
		tokenAddresses["bnb"][Web3Networks.Bsc] = Bnb.getTokenBscAddress();
		tokenAddresses["bnb"][Web3Networks.Ethereum] = Bnb
			.getTokenEthereumAddress();

		// comp
		tokenNames["comp"] = Comp.getTokenName();
		tokenRanges["comp"] = Comp.getTokenRanges();
		tokenNetworks["comp"] = Comp.getTokenNetworks();
		tokenAddresses["comp"][Web3Networks.Bsc] = Comp.getTokenBscAddress();
		tokenAddresses["comp"][Web3Networks.Ethereum] = Comp
			.getTokenEthereumAddress();

		// cro
		tokenNames["cro"] = Cro.getTokenName();
		tokenRanges["cro"] = Cro.getTokenRanges();
		tokenNetworks["cro"] = Cro.getTokenNetworks();
		tokenAddresses["cro"][Web3Networks.Bsc] = Cro.getTokenBscAddress();
		tokenAddresses["cro"][Web3Networks.Ethereum] = Cro
			.getTokenEthereumAddress();

		// crv
		tokenNames["crv"] = Crv.getTokenName();
		tokenRanges["crv"] = Crv.getTokenRanges();
		tokenNetworks["crv"] = Crv.getTokenNetworks();
		tokenAddresses["crv"][Web3Networks.Bsc] = Crv.getTokenBscAddress();
		tokenAddresses["crv"][Web3Networks.Ethereum] = Crv
			.getTokenEthereumAddress();

		// cvx
		tokenNames["cvx"] = Cvx.getTokenName();
		tokenRanges["cvx"] = Cvx.getTokenRanges();
		tokenNetworks["cvx"] = Cvx.getTokenNetworks();
		tokenAddresses["cvx"][Web3Networks.Bsc] = Cvx.getTokenBscAddress();
		tokenAddresses["cvx"][Web3Networks.Ethereum] = Cvx
			.getTokenEthereumAddress();

		// dai
		tokenNames["dai"] = Dai.getTokenName();
		tokenRanges["dai"] = Dai.getTokenRanges();
		tokenNetworks["dai"] = Dai.getTokenNetworks();
		tokenAddresses["dai"][Web3Networks.Bsc] = Dai.getTokenBscAddress();
		tokenAddresses["dai"][Web3Networks.Ethereum] = Dai
			.getTokenEthereumAddress();

		// doge
		tokenNames["doge"] = Doge.getTokenName();
		tokenRanges["doge"] = Doge.getTokenRanges();
		tokenNetworks["doge"] = Doge.getTokenNetworks();
		tokenAddresses["doge"][Web3Networks.Bsc] = Doge.getTokenBscAddress();
		tokenAddresses["doge"][Web3Networks.Ethereum] = Doge
			.getTokenEthereumAddress();

		// dydx
		tokenNames["dydx"] = Dydx.getTokenName();
		tokenRanges["dydx"] = Dydx.getTokenRanges();
		tokenNetworks["dydx"] = Dydx.getTokenNetworks();
		tokenAddresses["dydx"][Web3Networks.Bsc] = Dydx.getTokenBscAddress();
		tokenAddresses["dydx"][Web3Networks.Ethereum] = Dydx
			.getTokenEthereumAddress();

		// etc
		tokenNames["etc"] = Etc.getTokenName();
		tokenRanges["etc"] = Etc.getTokenRanges();
		tokenNetworks["etc"] = Etc.getTokenNetworks();
		tokenAddresses["etc"][Web3Networks.Bsc] = Etc.getTokenBscAddress();
		tokenAddresses["etc"][Web3Networks.Ethereum] = Etc
			.getTokenEthereumAddress();

		// eth
		tokenNames["eth"] = Eth.getTokenName();
		tokenRanges["eth"] = Eth.getTokenRanges();
		tokenNetworks["eth"] = Eth.getTokenNetworks();
		tokenAddresses["eth"][Web3Networks.Bsc] = Eth.getTokenBscAddress();
		tokenAddresses["eth"][Web3Networks.Ethereum] = Eth
			.getTokenEthereumAddress();

		// fil
		tokenNames["fil"] = Fil.getTokenName();
		tokenRanges["fil"] = Fil.getTokenRanges();
		tokenNetworks["fil"] = Fil.getTokenNetworks();
		tokenAddresses["fil"][Web3Networks.Bsc] = Fil.getTokenBscAddress();
		tokenAddresses["fil"][Web3Networks.Ethereum] = Fil
			.getTokenEthereumAddress();

		// grt
		tokenNames["grt"] = Grt.getTokenName();
		tokenRanges["grt"] = Grt.getTokenRanges();
		tokenNetworks["grt"] = Grt.getTokenNetworks();
		tokenAddresses["grt"][Web3Networks.Bsc] = Grt.getTokenBscAddress();
		tokenAddresses["grt"][Web3Networks.Ethereum] = Grt
			.getTokenEthereumAddress();

		// gtc
		tokenNames["gtc"] = Gtc.getTokenName();
		tokenRanges["gtc"] = Gtc.getTokenRanges();
		tokenNetworks["gtc"] = Gtc.getTokenNetworks();
		tokenAddresses["gtc"][Web3Networks.Bsc] = Gtc.getTokenBscAddress();
		tokenAddresses["gtc"][Web3Networks.Ethereum] = Gtc
			.getTokenEthereumAddress();

		// gusd
		tokenNames["gusd"] = Gusd.getTokenName();
		tokenRanges["gusd"] = Gusd.getTokenRanges();
		tokenNetworks["gusd"] = Gusd.getTokenNetworks();
		tokenAddresses["gusd"][Web3Networks.Bsc] = Gusd.getTokenBscAddress();
		tokenAddresses["gusd"][Web3Networks.Ethereum] = Gusd
			.getTokenEthereumAddress();

		// imx
		tokenNames["imx"] = Imx.getTokenName();
		tokenRanges["imx"] = Imx.getTokenRanges();
		tokenNetworks["imx"] = Imx.getTokenNetworks();
		tokenAddresses["imx"][Web3Networks.Bsc] = Imx.getTokenBscAddress();
		tokenAddresses["imx"][Web3Networks.Ethereum] = Imx
			.getTokenEthereumAddress();

		// inj
		tokenNames["inj"] = Inj.getTokenName();
		tokenRanges["inj"] = Inj.getTokenRanges();
		tokenNetworks["inj"] = Inj.getTokenNetworks();
		tokenAddresses["inj"][Web3Networks.Bsc] = Inj.getTokenBscAddress();
		tokenAddresses["inj"][Web3Networks.Ethereum] = Inj
			.getTokenEthereumAddress();

		// leo
		tokenNames["leo"] = Leo.getTokenName();
		tokenRanges["leo"] = Leo.getTokenRanges();
		tokenNetworks["leo"] = Leo.getTokenNetworks();
		tokenAddresses["leo"][Web3Networks.Bsc] = Leo.getTokenBscAddress();
		tokenAddresses["leo"][Web3Networks.Ethereum] = Leo
			.getTokenEthereumAddress();

		// link
		tokenNames["link"] = Link.getTokenName();
		tokenRanges["link"] = Link.getTokenRanges();
		tokenNetworks["link"] = Link.getTokenNetworks();
		tokenAddresses["link"][Web3Networks.Bsc] = Link.getTokenBscAddress();
		tokenAddresses["link"][Web3Networks.Ethereum] = Link
			.getTokenEthereumAddress();

		// lit
		tokenNames["lit"] = Lit.getTokenName();
		tokenRanges["lit"] = Lit.getTokenRanges();
		tokenNetworks["lit"] = Lit.getTokenNetworks();
		tokenAddresses["lit"][Web3Networks.Bsc] = Lit.getTokenBscAddress();
		tokenAddresses["lit"][Web3Networks.Ethereum] = Lit
			.getTokenEthereumAddress();

		// matic
		tokenNames["matic"] = Matic.getTokenName();
		tokenRanges["matic"] = Matic.getTokenRanges();
		tokenNetworks["matic"] = Matic.getTokenNetworks();
		tokenAddresses["matic"][Web3Networks.Bsc] = Matic.getTokenBscAddress();
		tokenAddresses["matic"][Web3Networks.Ethereum] = Matic
			.getTokenEthereumAddress();

		// mcrt
		tokenNames["mcrt"] = Mcrt.getTokenName();
		tokenRanges["mcrt"] = Mcrt.getTokenRanges();
		tokenNetworks["mcrt"] = Mcrt.getTokenNetworks();
		tokenAddresses["mcrt"][Web3Networks.Bsc] = Mcrt.getTokenBscAddress();
		tokenAddresses["mcrt"][Web3Networks.Ethereum] = Mcrt
			.getTokenEthereumAddress();

		// nfp
		tokenNames["nfp"] = Nfp.getTokenName();
		tokenRanges["nfp"] = Nfp.getTokenRanges();
		tokenNetworks["nfp"] = Nfp.getTokenNetworks();
		tokenAddresses["nfp"][Web3Networks.Bsc] = Nfp.getTokenBscAddress();
		tokenAddresses["nfp"][Web3Networks.Ethereum] = Nfp
			.getTokenEthereumAddress();

		// people
		tokenNames["people"] = People.getTokenName();
		tokenRanges["people"] = People.getTokenRanges();
		tokenNetworks["people"] = People.getTokenNetworks();
		tokenAddresses["people"][Web3Networks.Bsc] = People
			.getTokenBscAddress();
		tokenAddresses["people"][Web3Networks.Ethereum] = People
			.getTokenEthereumAddress();

		// shib
		tokenNames["shib"] = Shib.getTokenName();
		tokenRanges["shib"] = Shib.getTokenRanges();
		tokenNetworks["shib"] = Shib.getTokenNetworks();
		tokenAddresses["shib"][Web3Networks.Bsc] = Shib.getTokenBscAddress();
		tokenAddresses["shib"][Web3Networks.Ethereum] = Shib
			.getTokenEthereumAddress();

		// sol
		tokenNames["sol"] = Sol.getTokenName();
		tokenRanges["sol"] = Sol.getTokenRanges();
		tokenNetworks["sol"] = Sol.getTokenNetworks();
		tokenAddresses["sol"][Web3Networks.Bsc] = Sol.getTokenBscAddress();
		tokenAddresses["sol"][Web3Networks.Ethereum] = Sol
			.getTokenEthereumAddress();

		// spaceid
		tokenNames["spaceid"] = SpaceId.getTokenName();
		tokenRanges["spaceid"] = SpaceId.getTokenRanges();
		tokenNetworks["spaceid"] = SpaceId.getTokenNetworks();
		tokenAddresses["spaceid"][Web3Networks.Bsc] = SpaceId
			.getTokenBscAddress();
		tokenAddresses["spaceid"][Web3Networks.Ethereum] = SpaceId
			.getTokenEthereumAddress();

		// ton
		tokenNames["ton"] = Ton.getTokenName();
		tokenRanges["ton"] = Ton.getTokenRanges();
		tokenNetworks["ton"] = Ton.getTokenNetworks();
		tokenAddresses["ton"][Web3Networks.Bsc] = Ton.getTokenBscAddress();
		tokenAddresses["ton"][Web3Networks.Ethereum] = Ton
			.getTokenEthereumAddress();

		// trx
		tokenNames["trx"] = Trx.getTokenName();
		tokenRanges["trx"] = Trx.getTokenRanges();
		tokenNetworks["trx"] = Trx.getTokenNetworks();
		tokenAddresses["trx"][Web3Networks.Bsc] = Trx.getTokenBscAddress();
		tokenAddresses["trx"][Web3Networks.Ethereum] = Trx
			.getTokenEthereumAddress();

		// tusd
		tokenNames["tusd"] = Tusd.getTokenName();
		tokenRanges["tusd"] = Tusd.getTokenRanges();
		tokenNetworks["tusd"] = Tusd.getTokenNetworks();
		tokenAddresses["tusd"][Web3Networks.Bsc] = Tusd.getTokenBscAddress();
		tokenAddresses["tusd"][Web3Networks.Ethereum] = Tusd
			.getTokenEthereumAddress();

		// uni
		tokenNames["uni"] = Uni.getTokenName();
		tokenRanges["uni"] = Uni.getTokenRanges();
		tokenNetworks["uni"] = Uni.getTokenNetworks();
		tokenAddresses["uni"][Web3Networks.Bsc] = Uni.getTokenBscAddress();
		tokenAddresses["uni"][Web3Networks.Ethereum] = Uni
			.getTokenEthereumAddress();

		// usdc
		tokenNames["usdc"] = Usdc.getTokenName();
		tokenRanges["usdc"] = Usdc.getTokenRanges();
		tokenNetworks["usdc"] = Usdc.getTokenNetworks();
		tokenAddresses["usdc"][Web3Networks.Bsc] = Usdc.getTokenBscAddress();
		tokenAddresses["usdc"][Web3Networks.Ethereum] = Usdc
			.getTokenEthereumAddress();

		// usdd
		tokenNames["usdd"] = Usdd.getTokenName();
		tokenRanges["usdd"] = Usdd.getTokenRanges();
		tokenNetworks["usdd"] = Usdd.getTokenNetworks();
		tokenAddresses["usdd"][Web3Networks.Bsc] = Usdd.getTokenBscAddress();
		tokenAddresses["usdd"][Web3Networks.Ethereum] = Usdd
			.getTokenEthereumAddress();

		// usdt
		tokenNames["usdt"] = Usdt.getTokenName();
		tokenRanges["usdt"] = Usdt.getTokenRanges();
		tokenNetworks["usdt"] = Usdt.getTokenNetworks();
		tokenAddresses["usdt"][Web3Networks.Bsc] = Usdt.getTokenBscAddress();
		tokenAddresses["usdt"][Web3Networks.Ethereum] = Usdt
			.getTokenEthereumAddress();

		// wbtc
		tokenNames["wbtc"] = Wbtc.getTokenName();
		tokenRanges["wbtc"] = Wbtc.getTokenRanges();
		tokenNetworks["wbtc"] = Wbtc.getTokenNetworks();
		tokenAddresses["wbtc"][Web3Networks.Bsc] = Wbtc.getTokenBscAddress();
		tokenAddresses["wbtc"][Web3Networks.Ethereum] = Wbtc
			.getTokenEthereumAddress();
	}
}
