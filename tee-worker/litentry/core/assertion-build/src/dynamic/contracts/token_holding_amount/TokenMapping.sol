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
import { BRC20 } from "./brc20/BRC20.sol";

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

// btc
import { Btc } from "./Btc.sol";
contract TokenMapping is TokenQueryLogic {
    constructor() {
        // btcs
        tokenRanges["btcs"] = Btcs.getTokenRanges();
        for (uint8 i = 0; i < BRC20.getBrc20TokenInfo().length; i++) {
            tokenInfo["btcs"].push(BRC20.getBrc20TokenInfo()[i]);
        }

        // cats
        tokenRanges["cats"] = Cats.getTokenRanges();
        for (uint8 i = 0; i < BRC20.getBrc20TokenInfo().length; i++) {
            tokenInfo["cats"].push(BRC20.getBrc20TokenInfo()[i]);
        }

        // long
        tokenRanges["long"] = Long.getTokenRanges();
        for (uint8 i = 0; i < BRC20.getBrc20TokenInfo().length; i++) {
            tokenInfo["long"].push(BRC20.getBrc20TokenInfo()[i]);
        }

        // mmss
        tokenRanges["mmss"] = Mmss.getTokenRanges();
        for (uint8 i = 0; i < BRC20.getBrc20TokenInfo().length; i++) {
            tokenInfo["mmss"].push(BRC20.getBrc20TokenInfo()[i]);
        }

        // ordi
        tokenRanges["ordi"] = Ordi.getTokenRanges();
        for (uint8 i = 0; i < BRC20.getBrc20TokenInfo().length; i++) {
            tokenInfo["ordi"].push(BRC20.getBrc20TokenInfo()[i]);
        }

        // rats
        tokenRanges["rats"] = Rats.getTokenRanges();
        for (uint8 i = 0; i < BRC20.getBrc20TokenInfo().length; i++) {
            tokenInfo["rats"].push(BRC20.getBrc20TokenInfo()[i]);
        }

        // sats
        tokenRanges["sats"] = Sats.getTokenRanges();
        for (uint8 i = 0; i < BRC20.getBrc20TokenInfo().length; i++) {
            tokenInfo["sats"].push(BRC20.getBrc20TokenInfo()[i]);
        }

        // Btc
        tokenRanges["btc"] = Btc.getTokenRanges();
        for (uint8 i = 0; i < Btc.getTokenInfo().length; i++) {
            tokenInfo["btc"].push(Btc.getTokenInfo()[i]);
        }

        // ada
        tokenRanges["ada"] = Ada.getTokenRanges();
        for (uint8 i = 0; i < Ada.getTokenInfo().length; i++) {
            tokenInfo["ada"].push(Ada.getTokenInfo()[i]);
        }

        // amp
        tokenRanges["amp"] = Amp.getTokenRanges();
        for (uint8 i = 0; i < Amp.getTokenInfo().length; i++) {
            tokenInfo["amp"].push(Amp.getTokenInfo()[i]);
        }

        // atom
        tokenRanges["atom"] = Atom.getTokenRanges();
        for (uint8 i = 0; i < Atom.getTokenInfo().length; i++) {
            tokenInfo["atom"].push(Atom.getTokenInfo()[i]);
        }

        // bch
        tokenRanges["bch"] = Bch.getTokenRanges();
        for (uint8 i = 0; i < Bch.getTokenInfo().length; i++) {
            tokenInfo["bch"].push(Bch.getTokenInfo()[i]);
        }

        // bean
        tokenRanges["bean"] = Bean.getTokenRanges();
        for (uint8 i = 0; i < Bean.getTokenInfo().length; i++) {
            tokenInfo["bean"].push(Bean.getTokenInfo()[i]);
        }

        // bnb
        tokenRanges["bnb"] = Bnb.getTokenRanges();
        for (uint8 i = 0; i < Bnb.getTokenInfo().length; i++) {
            tokenInfo["bnb"].push(Bnb.getTokenInfo()[i]);
        }

        // comp
        tokenRanges["comp"] = Comp.getTokenRanges();
        for (uint8 i = 0; i < Comp.getTokenInfo().length; i++) {
            tokenInfo["comp"].push(Comp.getTokenInfo()[i]);
        }

        // cro
        tokenRanges["cro"] = Cro.getTokenRanges();
        for (uint8 i = 0; i < Cro.getTokenInfo().length; i++) {
            tokenInfo["cro"].push(Cro.getTokenInfo()[i]);
        }

        // crv
        tokenRanges["crv"] = Crv.getTokenRanges();
        for (uint8 i = 0; i < Crv.getTokenInfo().length; i++) {
            tokenInfo["crv"].push(Crv.getTokenInfo()[i]);
        }

        // dai
        tokenRanges["dai"] = Dai.getTokenRanges();
        for (uint8 i = 0; i < Dai.getTokenInfo().length; i++) {
            tokenInfo["dai"].push(Dai.getTokenInfo()[i]);
        }

        // doge
        tokenRanges["doge"] = Doge.getTokenRanges();
        for (uint8 i = 0; i < Doge.getTokenInfo().length; i++) {
            tokenInfo["doge"].push(Doge.getTokenInfo()[i]);
        }

        // dydx
        tokenRanges["dydx"] = Dydx.getTokenRanges();
        for (uint8 i = 0; i < Dydx.getTokenInfo().length; i++) {
            tokenInfo["dydx"].push(Dydx.getTokenInfo()[i]);
        }

        // etc
        tokenRanges["etc"] = Etc.getTokenRanges();
        for (uint8 i = 0; i < Etc.getTokenInfo().length; i++) {
            tokenInfo["etc"].push(Etc.getTokenInfo()[i]);
        }

        // eth
        tokenRanges["eth"] = Eth.getTokenRanges();
        for (uint8 i = 0; i < Eth.getTokenInfo().length; i++) {
            tokenInfo["eth"].push(Eth.getTokenInfo()[i]);
        }

        // fil
        tokenRanges["fil"] = Fil.getTokenRanges();
        for (uint8 i = 0; i < Fil.getTokenInfo().length; i++) {
            tokenInfo["fil"].push(Fil.getTokenInfo()[i]);
        }

        // grt
        tokenRanges["grt"] = Grt.getTokenRanges();
        for (uint8 i = 0; i < Grt.getTokenInfo().length; i++) {
            tokenInfo["grt"].push(Grt.getTokenInfo()[i]);
        }

        // gtc
        tokenRanges["gtc"] = Gtc.getTokenRanges();
        for (uint8 i = 0; i < Gtc.getTokenInfo().length; i++) {
            tokenInfo["gtc"].push(Gtc.getTokenInfo()[i]);
        }

        // gusd
        tokenRanges["gusd"] = Gusd.getTokenRanges();
        for (uint8 i = 0; i < Gusd.getTokenInfo().length; i++) {
            tokenInfo["gusd"].push(Gusd.getTokenInfo()[i]);
        }

        // imx
        tokenRanges["imx"] = Imx.getTokenRanges();
        for (uint8 i = 0; i < Imx.getTokenInfo().length; i++) {
            tokenInfo["imx"].push(Imx.getTokenInfo()[i]);
        }

        // inj
        tokenRanges["inj"] = Inj.getTokenRanges();
        for (uint8 i = 0; i < Inj.getTokenInfo().length; i++) {
            tokenInfo["inj"].push(Inj.getTokenInfo()[i]);
        }

        // leo
        tokenRanges["leo"] = Leo.getTokenRanges();
        for (uint8 i = 0; i < Leo.getTokenInfo().length; i++) {
            tokenInfo["leo"].push(Leo.getTokenInfo()[i]);
        }

        // link
        tokenRanges["link"] = Link.getTokenRanges();
        for (uint8 i = 0; i < Link.getTokenInfo().length; i++) {
            tokenInfo["link"].push(Link.getTokenInfo()[i]);
        }

        // lit
        tokenRanges["lit"] = Lit.getTokenRanges();
        for (uint8 i = 0; i < Lit.getTokenInfo().length; i++) {
            tokenInfo["lit"].push(Lit.getTokenInfo()[i]);
        }

        // matic
        tokenRanges["matic"] = Matic.getTokenRanges();
        for (uint8 i = 0; i < Matic.getTokenInfo().length; i++) {
            tokenInfo["matic"].push(Matic.getTokenInfo()[i]);
        }

        // mcrt
        tokenRanges["mcrt"] = Mcrt.getTokenRanges();
        for (uint8 i = 0; i < Mcrt.getTokenInfo().length; i++) {
            tokenInfo["mcrt"].push(Mcrt.getTokenInfo()[i]);
        }

        // nfp
        tokenRanges["nfp"] = Nfp.getTokenRanges();
        for (uint8 i = 0; i < Nfp.getTokenInfo().length; i++) {
            tokenInfo["nfp"].push(Nfp.getTokenInfo()[i]);
        }

        // people
        tokenRanges["people"] = People.getTokenRanges();
        for (uint8 i = 0; i < People.getTokenInfo().length; i++) {
            tokenInfo["people"].push(People.getTokenInfo()[i]);
        }

        // shib
        tokenRanges["shib"] = Shib.getTokenRanges();
        for (uint8 i = 0; i < Shib.getTokenInfo().length; i++) {
            tokenInfo["shib"].push(Shib.getTokenInfo()[i]);
        }

        // sol
        tokenRanges["sol"] = Sol.getTokenRanges();
        for (uint8 i = 0; i < Sol.getTokenInfo().length; i++) {
            tokenInfo["sol"].push(Sol.getTokenInfo()[i]);
        }

        // spaceid
        tokenRanges["spaceid"] = SpaceId.getTokenRanges();
        for (uint8 i = 0; i < SpaceId.getTokenInfo().length; i++) {
            tokenInfo["spaceid"].push(SpaceId.getTokenInfo()[i]);
        }

        // ton
        tokenRanges["ton"] = Ton.getTokenRanges();
        for (uint8 i = 0; i < Ton.getTokenInfo().length; i++) {
            tokenInfo["ton"].push(Ton.getTokenInfo()[i]);
        }

        // trx
        tokenRanges["trx"] = Trx.getTokenRanges();
        for (uint8 i = 0; i < Trx.getTokenInfo().length; i++) {
            tokenInfo["trx"].push(Trx.getTokenInfo()[i]);
        }

        // tusd
        tokenRanges["tusd"] = Tusd.getTokenRanges();
        for (uint8 i = 0; i < Tusd.getTokenInfo().length; i++) {
            tokenInfo["tusd"].push(Tusd.getTokenInfo()[i]);
        }

        // uni
        tokenRanges["uni"] = Uni.getTokenRanges();
        for (uint8 i = 0; i < Uni.getTokenInfo().length; i++) {
            tokenInfo["uni"].push(Uni.getTokenInfo()[i]);
        }

        // usdc
        tokenRanges["usdc"] = Usdc.getTokenRanges();
        for (uint8 i = 0; i < Usdc.getTokenInfo().length; i++) {
            tokenInfo["usdc"].push(Usdc.getTokenInfo()[i]);
        }

        // usdd
        tokenRanges["usdd"] = Usdd.getTokenRanges();
        for (uint8 i = 0; i < Usdd.getTokenInfo().length; i++) {
            tokenInfo["usdd"].push(Usdd.getTokenInfo()[i]);
        }

        // usdt
        tokenRanges["usdt"] = Usdt.getTokenRanges();
        for (uint8 i = 0; i < Usdt.getTokenInfo().length; i++) {
            tokenInfo["usdt"].push(Usdt.getTokenInfo()[i]);
        }

        // wbtc
        tokenRanges["wbtc"] = Wbtc.getTokenRanges();
        for (uint8 i = 0; i < Wbtc.getTokenInfo().length; i++) {
            tokenInfo["wbtc"].push(Wbtc.getTokenInfo()[i]);
        }
    }
}
