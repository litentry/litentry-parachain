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
		tokenRanges["btcs"] = Btcs.getTokenRanges();
		tokenNetworks["btcs"] = Btcs.getTokenNetworks();

		// cats
		tokenRanges["cats"] = Cats.getTokenRanges();
		tokenNetworks["cats"] = Cats.getTokenNetworks();

		// long
		tokenRanges["long"] = Long.getTokenRanges();
		tokenNetworks["long"] = Long.getTokenNetworks();

		// mmss
		tokenRanges["mmss"] = Mmss.getTokenRanges();
		tokenNetworks["mmss"] = Mmss.getTokenNetworks();

		// ordi
		tokenRanges["ordi"] = Ordi.getTokenRanges();
		tokenNetworks["ordi"] = Ordi.getTokenNetworks();

		// rats
		tokenRanges["rats"] = Rats.getTokenRanges();
		tokenNetworks["rats"] = Rats.getTokenNetworks();

		// sats
		tokenRanges["sats"] = Sats.getTokenRanges();
		tokenNetworks["sats"] = Sats.getTokenNetworks();

		// ada
		tokenRanges["ada"] = Ada.getTokenRanges();
		tokenNetworks["ada"] = Ada.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["ada"].length; i++) {
			tokenAddresses["ada"][tokenNetworks["ada"][i]] = Ada
				.getTokenAddress(tokenNetworks["ada"][i]);
		}

		// amp
		tokenRanges["amp"] = Amp.getTokenRanges();
		tokenNetworks["amp"] = Amp.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["amp"].length; i++) {
			tokenAddresses["amp"][tokenNetworks["amp"][i]] = Amp
				.getTokenAddress(tokenNetworks["amp"][i]);
		}

		// atom
		tokenRanges["atom"] = Atom.getTokenRanges();
		tokenNetworks["atom"] = Atom.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["atom"].length; i++) {
			tokenAddresses["atom"][tokenNetworks["atom"][i]] = Atom
				.getTokenAddress(tokenNetworks["atom"][i]);
		}

		// bch
		tokenRanges["bch"] = Bch.getTokenRanges();
		tokenNetworks["bch"] = Bch.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["bch"].length; i++) {
			tokenAddresses["bch"][tokenNetworks["bch"][i]] = Bch
				.getTokenAddress(tokenNetworks["bch"][i]);
		}

		// bean
		tokenRanges["bean"] = Bean.getTokenRanges();
		tokenNetworks["bean"] = Bean.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["bean"].length; i++) {
			tokenAddresses["bean"][tokenNetworks["bean"][i]] = Bean
				.getTokenAddress(tokenNetworks["bean"][i]);
		}

		// bnb
		tokenRanges["bnb"] = Bnb.getTokenRanges();
		tokenNetworks["bnb"] = Bnb.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["bnb"].length; i++) {
			tokenAddresses["bnb"][tokenNetworks["bnb"][i]] = Bnb
				.getTokenAddress(tokenNetworks["bnb"][i]);
		}

		// comp
		tokenRanges["comp"] = Comp.getTokenRanges();
		tokenNetworks["comp"] = Comp.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["comp"].length; i++) {
			tokenAddresses["comp"][tokenNetworks["comp"][i]] = Comp
				.getTokenAddress(tokenNetworks["comp"][i]);
		}

		// cro
		tokenRanges["cro"] = Cro.getTokenRanges();
		tokenNetworks["cro"] = Cro.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["cro"].length; i++) {
			tokenAddresses["cro"][tokenNetworks["cro"][i]] = Cro
				.getTokenAddress(tokenNetworks["cro"][i]);
		}

		// crv
		tokenRanges["crv"] = Crv.getTokenRanges();
		tokenNetworks["crv"] = Crv.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["crv"].length; i++) {
			tokenAddresses["crv"][tokenNetworks["crv"][i]] = Crv
				.getTokenAddress(tokenNetworks["crv"][i]);
		}

		// dai
		tokenRanges["dai"] = Dai.getTokenRanges();
		tokenNetworks["dai"] = Dai.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["dai"].length; i++) {
			tokenAddresses["dai"][tokenNetworks["dai"][i]] = Dai
				.getTokenAddress(tokenNetworks["dai"][i]);
		}

		// doge
		tokenRanges["doge"] = Doge.getTokenRanges();
		tokenNetworks["doge"] = Doge.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["doge"].length; i++) {
			tokenAddresses["doge"][tokenNetworks["doge"][i]] = Doge
				.getTokenAddress(tokenNetworks["doge"][i]);
		}

		// dydx
		tokenRanges["dydx"] = Dydx.getTokenRanges();
		tokenNetworks["dydx"] = Dydx.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["dydx"].length; i++) {
			tokenAddresses["dydx"][tokenNetworks["dydx"][i]] = Dydx
				.getTokenAddress(tokenNetworks["dydx"][i]);
		}

		// etc
		tokenRanges["etc"] = Etc.getTokenRanges();
		tokenNetworks["etc"] = Etc.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["etc"].length; i++) {
			tokenAddresses["etc"][tokenNetworks["etc"][i]] = Etc
				.getTokenAddress(tokenNetworks["etc"][i]);
		}

		// eth
		tokenRanges["eth"] = Eth.getTokenRanges();
		tokenNetworks["eth"] = Eth.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["eth"].length; i++) {
			tokenAddresses["eth"][tokenNetworks["eth"][i]] = Eth
				.getTokenAddress(tokenNetworks["eth"][i]);
		}

		// fil
		tokenRanges["fil"] = Fil.getTokenRanges();
		tokenNetworks["fil"] = Fil.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["fil"].length; i++) {
			tokenAddresses["fil"][tokenNetworks["fil"][i]] = Fil
				.getTokenAddress(tokenNetworks["fil"][i]);
		}

		// grt
		tokenRanges["grt"] = Grt.getTokenRanges();
		tokenNetworks["grt"] = Grt.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["grt"].length; i++) {
			tokenAddresses["grt"][tokenNetworks["grt"][i]] = Grt
				.getTokenAddress(tokenNetworks["grt"][i]);
		}

		// gtc
		tokenRanges["gtc"] = Gtc.getTokenRanges();
		tokenNetworks["gtc"] = Gtc.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["gtc"].length; i++) {
			tokenAddresses["gtc"][tokenNetworks["gtc"][i]] = Gtc
				.getTokenAddress(tokenNetworks["gtc"][i]);
		}

		// gusd
		tokenRanges["gusd"] = Gusd.getTokenRanges();
		tokenNetworks["gusd"] = Gusd.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["gusd"].length; i++) {
			tokenAddresses["gusd"][tokenNetworks["gusd"][i]] = Gusd
				.getTokenAddress(tokenNetworks["gusd"][i]);
		}

		// imx
		tokenRanges["imx"] = Imx.getTokenRanges();
		tokenNetworks["imx"] = Imx.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["imx"].length; i++) {
			tokenAddresses["imx"][tokenNetworks["imx"][i]] = Imx
				.getTokenAddress(tokenNetworks["imx"][i]);
		}

		// inj
		tokenRanges["inj"] = Inj.getTokenRanges();
		tokenNetworks["inj"] = Inj.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["inj"].length; i++) {
			tokenAddresses["inj"][tokenNetworks["inj"][i]] = Inj
				.getTokenAddress(tokenNetworks["inj"][i]);
		}

		// leo
		tokenRanges["leo"] = Leo.getTokenRanges();
		tokenNetworks["leo"] = Leo.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["leo"].length; i++) {
			tokenAddresses["leo"][tokenNetworks["leo"][i]] = Leo
				.getTokenAddress(tokenNetworks["leo"][i]);
		}

		// link
		tokenRanges["link"] = Link.getTokenRanges();
		tokenNetworks["link"] = Link.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["link"].length; i++) {
			tokenAddresses["link"][tokenNetworks["link"][i]] = Link
				.getTokenAddress(tokenNetworks["link"][i]);
		}

		// lit
		tokenRanges["lit"] = Lit.getTokenRanges();
		tokenNetworks["lit"] = Lit.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["lit"].length; i++) {
			tokenAddresses["lit"][tokenNetworks["lit"][i]] = Lit
				.getTokenAddress(tokenNetworks["lit"][i]);
		}

		// matic
		tokenRanges["matic"] = Matic.getTokenRanges();
		tokenNetworks["matic"] = Matic.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["matic"].length; i++) {
			tokenAddresses["matic"][tokenNetworks["matic"][i]] = Matic
				.getTokenAddress(tokenNetworks["matic"][i]);
		}

		// mcrt
		tokenRanges["mcrt"] = Mcrt.getTokenRanges();
		tokenNetworks["mcrt"] = Mcrt.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["mcrt"].length; i++) {
			tokenAddresses["mcrt"][tokenNetworks["mcrt"][i]] = Mcrt
				.getTokenAddress(tokenNetworks["mcrt"][i]);
		}

		// nfp
		tokenRanges["nfp"] = Nfp.getTokenRanges();
		tokenNetworks["nfp"] = Nfp.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["nfp"].length; i++) {
			tokenAddresses["nfp"][tokenNetworks["nfp"][i]] = Nfp
				.getTokenAddress(tokenNetworks["nfp"][i]);
		}

		// people
		tokenRanges["people"] = People.getTokenRanges();
		tokenNetworks["people"] = People.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["people"].length; i++) {
			tokenAddresses["people"][tokenNetworks["people"][i]] = People
				.getTokenAddress(tokenNetworks["people"][i]);
		}

		// shib
		tokenRanges["shib"] = Shib.getTokenRanges();
		tokenNetworks["shib"] = Shib.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["shib"].length; i++) {
			tokenAddresses["shib"][tokenNetworks["shib"][i]] = Shib
				.getTokenAddress(tokenNetworks["shib"][i]);
		}

		// sol
		tokenRanges["sol"] = Sol.getTokenRanges();
		tokenNetworks["sol"] = Sol.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["sol"].length; i++) {
			tokenAddresses["sol"][tokenNetworks["sol"][i]] = Sol
				.getTokenAddress(tokenNetworks["sol"][i]);
		}

		// spaceid
		tokenRanges["spaceid"] = SpaceId.getTokenRanges();
		tokenNetworks["spaceid"] = SpaceId.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["spaceid"].length; i++) {
			tokenAddresses["spaceid"][tokenNetworks["spaceid"][i]] = SpaceId
				.getTokenAddress(tokenNetworks["spaceid"][i]);
		}

		// ton
		tokenRanges["ton"] = Ton.getTokenRanges();
		tokenNetworks["ton"] = Ton.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["ton"].length; i++) {
			tokenAddresses["ton"][tokenNetworks["ton"][i]] = Ton
				.getTokenAddress(tokenNetworks["ton"][i]);
		}

		// trx
		tokenRanges["trx"] = Trx.getTokenRanges();
		tokenNetworks["trx"] = Trx.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["trx"].length; i++) {
			tokenAddresses["trx"][tokenNetworks["trx"][i]] = Trx
				.getTokenAddress(tokenNetworks["trx"][i]);
		}

		// tusd
		tokenRanges["tusd"] = Tusd.getTokenRanges();
		tokenNetworks["tusd"] = Tusd.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["tusd"].length; i++) {
			tokenAddresses["tusd"][tokenNetworks["tusd"][i]] = Tusd
				.getTokenAddress(tokenNetworks["tusd"][i]);
		}

		// uni
		tokenRanges["uni"] = Uni.getTokenRanges();
		tokenNetworks["uni"] = Uni.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["uni"].length; i++) {
			tokenAddresses["uni"][tokenNetworks["uni"][i]] = Uni
				.getTokenAddress(tokenNetworks["uni"][i]);
		}

		// usdc
		tokenRanges["usdc"] = Usdc.getTokenRanges();
		tokenNetworks["usdc"] = Usdc.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["usdc"].length; i++) {
			tokenAddresses["usdc"][tokenNetworks["usdc"][i]] = Usdc
				.getTokenAddress(tokenNetworks["usdc"][i]);
		}

		// usdd
		tokenRanges["usdd"] = Usdd.getTokenRanges();
		tokenNetworks["usdd"] = Usdd.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["usdd"].length; i++) {
			tokenAddresses["usdd"][tokenNetworks["usdd"][i]] = Usdd
				.getTokenAddress(tokenNetworks["usdd"][i]);
		}

		// usdt
		tokenRanges["usdt"] = Usdt.getTokenRanges();
		tokenNetworks["usdt"] = Usdt.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["usdt"].length; i++) {
			tokenAddresses["usdt"][tokenNetworks["usdt"][i]] = Usdt
				.getTokenAddress(tokenNetworks["usdt"][i]);
		}

		// wbtc
		tokenRanges["wbtc"] = Wbtc.getTokenRanges();
		tokenNetworks["wbtc"] = Wbtc.getTokenNetworks();
		for (uint32 i = 0; i < tokenNetworks["wbtc"].length; i++) {
			tokenAddresses["wbtc"][tokenNetworks["wbtc"][i]] = Wbtc
				.getTokenAddress(tokenNetworks["wbtc"][i]);
		}
	}
}
