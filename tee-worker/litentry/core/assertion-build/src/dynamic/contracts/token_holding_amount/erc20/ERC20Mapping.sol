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

import { ERC20 } from "../ERC20.sol";

import { Ada } from "./Ada.sol";
import { Amp } from "./Amp.sol";
import { Atom } from "./Atom.sol";
import { Bch } from "./Bch.sol";
import { Bean } from "./Bean.sol";
import { Bnb } from "./Bnb.sol";
import { Comp } from "./Comp.sol";
import { Cro } from "./Cro.sol";
import { Crv } from "./Crv.sol";
import { Dai } from "./Dai.sol";
import { Doge } from "./Doge.sol";
import { Dydx } from "./Dydx.sol";
import { Etc } from "./Etc.sol";
import { Eth } from "./Eth.sol";
import { Fil } from "./Fil.sol";
import { Grt } from "./Grt.sol";
import { Gtc } from "./Gtc.sol";
import { Gusd } from "./Gusd.sol";
import { Imx } from "./Imx.sol";
import { Inj } from "./Inj.sol";
import { Leo } from "./Leo.sol";
import { Link } from "./Link.sol";
import { Lit } from "./Lit.sol";
import { Matic } from "./Matic.sol";
import { Mcrt } from ".//Mcrt.sol";
import { Nfp } from "./Nfp.sol";
import { People } from "./People.sol";
import { Shib } from ".//Shib.sol";
import { Sol } from "./Sol.sol";
import { SpaceId } from "./SpaceId.sol";
import { Ton } from "./Ton.sol";
import { Trx } from "./Trx.sol";
import { Tusd } from "./Tusd.sol";
import { Uni } from "./Uni.sol";
import { Usdc } from "./Usdc.sol";
import { Usdt } from "./Usdt.sol";
import { Wbtc } from ".//Wbtc.sol";
import { Cvx } from "./Cvx.sol";
import { Usdd } from "./Usdd.sol";

contract ERC20Mapping is ERC20 {
	constructor() {
		// ada
		tokenNames["ada"] = Ada.getTokenName();
		tokenRanges["ada"] = Ada.getTokenRanges();
		tokenBscAddress["ada"] = Ada.getTokenBscAddress();
		tokenEthereumAddress["ada"] = Ada.getTokenEthereumAddress();

		// amp
		tokenNames["amp"] = Amp.getTokenName();
		tokenRanges["amp"] = Amp.getTokenRanges();
		tokenBscAddress["amp"] = Amp.getTokenBscAddress();
		tokenEthereumAddress["amp"] = Amp.getTokenEthereumAddress();

		// atom
		tokenNames["atom"] = Atom.getTokenName();
		tokenRanges["atom"] = Atom.getTokenRanges();
		tokenBscAddress["atom"] = Atom.getTokenBscAddress();
		tokenEthereumAddress["atom"] = Atom.getTokenEthereumAddress();

		// bch
		tokenNames["bch"] = Bch.getTokenName();
		tokenRanges["bch"] = Bch.getTokenRanges();
		tokenBscAddress["bch"] = Bch.getTokenBscAddress();
		tokenEthereumAddress["bch"] = Bch.getTokenEthereumAddress();

		// bean
		tokenNames["bean"] = Bean.getTokenName();
		tokenRanges["bean"] = Bean.getTokenRanges();
		tokenBscAddress["bean"] = Bean.getTokenBscAddress();
		tokenEthereumAddress["bean"] = Bean.getTokenEthereumAddress();

		// bnb
		tokenNames["bnb"] = Bnb.getTokenName();
		tokenRanges["bnb"] = Bnb.getTokenRanges();
		tokenBscAddress["bnb"] = Bnb.getTokenBscAddress();
		tokenEthereumAddress["bnb"] = Bnb.getTokenEthereumAddress();

		// comp
		tokenNames["comp"] = Comp.getTokenName();
		tokenRanges["comp"] = Comp.getTokenRanges();
		tokenBscAddress["comp"] = Comp.getTokenBscAddress();
		tokenEthereumAddress["comp"] = Comp.getTokenEthereumAddress();

		// cro
		tokenNames["cro"] = Cro.getTokenName();
		tokenRanges["cro"] = Cro.getTokenRanges();
		tokenBscAddress["cro"] = Cro.getTokenBscAddress();
		tokenEthereumAddress["cro"] = Cro.getTokenEthereumAddress();

		// crv
		tokenNames["crv"] = Crv.getTokenName();
		tokenRanges["crv"] = Crv.getTokenRanges();
		tokenBscAddress["crv"] = Crv.getTokenBscAddress();
		tokenEthereumAddress["crv"] = Crv.getTokenEthereumAddress();

		// cvx
		tokenNames["cvx"] = Cvx.getTokenName();
		tokenRanges["cvx"] = Cvx.getTokenRanges();
		tokenBscAddress["cvx"] = Cvx.getTokenBscAddress();
		tokenEthereumAddress["cvx"] = Cvx.getTokenEthereumAddress();

		// dai
		tokenNames["dai"] = Dai.getTokenName();
		tokenRanges["dai"] = Dai.getTokenRanges();
		tokenBscAddress["dai"] = Dai.getTokenBscAddress();
		tokenEthereumAddress["dai"] = Dai.getTokenEthereumAddress();

		// doge
		tokenNames["doge"] = Doge.getTokenName();
		tokenRanges["doge"] = Doge.getTokenRanges();
		tokenBscAddress["doge"] = Doge.getTokenBscAddress();
		tokenEthereumAddress["doge"] = Doge.getTokenEthereumAddress();

		// dydx
		tokenNames["dydx"] = Dydx.getTokenName();
		tokenRanges["dydx"] = Dydx.getTokenRanges();
		tokenBscAddress["dydx"] = Dydx.getTokenBscAddress();
		tokenEthereumAddress["dydx"] = Dydx.getTokenEthereumAddress();

		// etc
		tokenNames["etc"] = Etc.getTokenName();
		tokenRanges["etc"] = Etc.getTokenRanges();
		tokenBscAddress["etc"] = Etc.getTokenBscAddress();
		tokenEthereumAddress["etc"] = Etc.getTokenEthereumAddress();

		// eth
		tokenNames["eth"] = Eth.getTokenName();
		tokenRanges["eth"] = Eth.getTokenRanges();
		tokenBscAddress["eth"] = Eth.getTokenBscAddress();
		tokenEthereumAddress["eth"] = Eth.getTokenEthereumAddress();

		// fil
		tokenNames["fil"] = Fil.getTokenName();
		tokenRanges["fil"] = Fil.getTokenRanges();
		tokenBscAddress["fil"] = Fil.getTokenBscAddress();
		tokenEthereumAddress["fil"] = Fil.getTokenEthereumAddress();

		// grt
		tokenNames["grt"] = Grt.getTokenName();
		tokenRanges["grt"] = Grt.getTokenRanges();
		tokenBscAddress["grt"] = Grt.getTokenBscAddress();
		tokenEthereumAddress["grt"] = Grt.getTokenEthereumAddress();

		// gtc
		tokenNames["gtc"] = Gtc.getTokenName();
		tokenRanges["gtc"] = Gtc.getTokenRanges();
		tokenBscAddress["gtc"] = Gtc.getTokenBscAddress();
		tokenEthereumAddress["gtc"] = Gtc.getTokenEthereumAddress();

		// gusd
		tokenNames["gusd"] = Gusd.getTokenName();
		tokenRanges["gusd"] = Gusd.getTokenRanges();
		tokenBscAddress["gusd"] = Gusd.getTokenBscAddress();
		tokenEthereumAddress["gusd"] = Gusd.getTokenEthereumAddress();

		// imx
		tokenNames["imx"] = Imx.getTokenName();
		tokenRanges["imx"] = Imx.getTokenRanges();
		tokenBscAddress["imx"] = Imx.getTokenBscAddress();
		tokenEthereumAddress["imx"] = Imx.getTokenEthereumAddress();

		// inj
		tokenNames["inj"] = Inj.getTokenName();
		tokenRanges["inj"] = Inj.getTokenRanges();
		tokenBscAddress["inj"] = Inj.getTokenBscAddress();
		tokenEthereumAddress["inj"] = Inj.getTokenEthereumAddress();

		// leo
		tokenNames["leo"] = Leo.getTokenName();
		tokenRanges["leo"] = Leo.getTokenRanges();
		tokenBscAddress["leo"] = Leo.getTokenBscAddress();
		tokenEthereumAddress["leo"] = Leo.getTokenEthereumAddress();

		// link
		tokenNames["link"] = Link.getTokenName();
		tokenRanges["link"] = Link.getTokenRanges();
		tokenBscAddress["link"] = Link.getTokenBscAddress();
		tokenEthereumAddress["link"] = Link.getTokenEthereumAddress();

		// lit
		tokenNames["lit"] = Lit.getTokenName();
		tokenRanges["lit"] = Lit.getTokenRanges();
		tokenBscAddress["lit"] = Lit.getTokenBscAddress();
		tokenEthereumAddress["lit"] = Lit.getTokenEthereumAddress();

		// matic
		tokenNames["matic"] = Matic.getTokenName();
		tokenRanges["matic"] = Matic.getTokenRanges();
		tokenBscAddress["matic"] = Matic.getTokenBscAddress();
		tokenEthereumAddress["matic"] = Matic.getTokenEthereumAddress();

		// mcrt
		tokenNames["mcrt"] = Mcrt.getTokenName();
		tokenRanges["mcrt"] = Mcrt.getTokenRanges();
		tokenBscAddress["mcrt"] = Mcrt.getTokenBscAddress();
		tokenEthereumAddress["mcrt"] = Mcrt.getTokenEthereumAddress();

		// nfp
		tokenNames["nfp"] = Nfp.getTokenName();
		tokenRanges["nfp"] = Nfp.getTokenRanges();
		tokenBscAddress["nfp"] = Nfp.getTokenBscAddress();
		tokenEthereumAddress["nfp"] = Nfp.getTokenEthereumAddress();

		// people
		tokenNames["people"] = People.getTokenName();
		tokenRanges["people"] = People.getTokenRanges();
		tokenBscAddress["people"] = People.getTokenBscAddress();
		tokenEthereumAddress["people"] = People.getTokenEthereumAddress();

		// shib
		tokenNames["shib"] = Shib.getTokenName();
		tokenRanges["shib"] = Shib.getTokenRanges();
		tokenBscAddress["shib"] = Shib.getTokenBscAddress();
		tokenEthereumAddress["shib"] = Shib.getTokenEthereumAddress();

		// sol
		tokenNames["sol"] = Sol.getTokenName();
		tokenRanges["sol"] = Sol.getTokenRanges();
		tokenBscAddress["sol"] = Sol.getTokenBscAddress();
		tokenEthereumAddress["sol"] = Sol.getTokenEthereumAddress();

		// spaceid
		tokenNames["spaceid"] = SpaceId.getTokenName();
		tokenRanges["spaceid"] = SpaceId.getTokenRanges();
		tokenBscAddress["spaceid"] = SpaceId.getTokenBscAddress();
		tokenEthereumAddress["spaceid"] = SpaceId.getTokenEthereumAddress();

		// ton
		tokenNames["ton"] = Ton.getTokenName();
		tokenRanges["ton"] = Ton.getTokenRanges();
		tokenBscAddress["ton"] = Ton.getTokenBscAddress();
		tokenEthereumAddress["ton"] = Ton.getTokenEthereumAddress();

		// trx
		tokenNames["trx"] = Trx.getTokenName();
		tokenRanges["trx"] = Trx.getTokenRanges();
		tokenBscAddress["trx"] = Trx.getTokenBscAddress();
		tokenEthereumAddress["trx"] = Trx.getTokenEthereumAddress();

		// tusd
		tokenNames["tusd"] = Tusd.getTokenName();
		tokenRanges["tusd"] = Tusd.getTokenRanges();
		tokenBscAddress["tusd"] = Tusd.getTokenBscAddress();
		tokenEthereumAddress["tusd"] = Tusd.getTokenEthereumAddress();

		// uni
		tokenNames["uni"] = Uni.getTokenName();
		tokenRanges["uni"] = Uni.getTokenRanges();
		tokenBscAddress["uni"] = Uni.getTokenBscAddress();
		tokenEthereumAddress["uni"] = Uni.getTokenEthereumAddress();

		// usdc
		tokenNames["usdc"] = Usdc.getTokenName();
		tokenRanges["usdc"] = Usdc.getTokenRanges();
		tokenBscAddress["usdc"] = Usdc.getTokenBscAddress();
		tokenEthereumAddress["usdc"] = Usdc.getTokenEthereumAddress();

		// usdd
		tokenNames["usdd"] = Usdd.getTokenName();
		tokenRanges["usdd"] = Usdd.getTokenRanges();
		tokenBscAddress["usdd"] = Usdd.getTokenBscAddress();
		tokenEthereumAddress["usdd"] = Usdd.getTokenEthereumAddress();

		// usdt
		tokenNames["usdt"] = Usdt.getTokenName();
		tokenRanges["usdt"] = Usdt.getTokenRanges();
		tokenBscAddress["usdt"] = Usdt.getTokenBscAddress();
		tokenEthereumAddress["usdt"] = Usdt.getTokenEthereumAddress();

		// wbtc
		tokenNames["wbtc"] = Wbtc.getTokenName();
		tokenRanges["wbtc"] = Wbtc.getTokenRanges();
		tokenBscAddress["wbtc"] = Wbtc.getTokenBscAddress();
		tokenEthereumAddress["wbtc"] = Wbtc.getTokenEthereumAddress();
	}
}
