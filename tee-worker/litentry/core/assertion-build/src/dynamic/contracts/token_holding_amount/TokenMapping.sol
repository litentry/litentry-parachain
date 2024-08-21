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
import "./Constants.sol";

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
        setTokenInfo("btcs", Btcs.getTokenRanges(), BRC20.getTokenNetworks());

        // cats
        setTokenInfo("cats", Cats.getTokenRanges(), BRC20.getTokenNetworks());

        // long
        setTokenInfo("long", Long.getTokenRanges(), BRC20.getTokenNetworks());

        // mmss
        setTokenInfo("mmss", Mmss.getTokenRanges(), BRC20.getTokenNetworks());

        // ordi
        setTokenInfo("ordi", Ordi.getTokenRanges(), BRC20.getTokenNetworks());

        // rats
        setTokenInfo("rats", Rats.getTokenRanges(), BRC20.getTokenNetworks());

        // sats
        setTokenInfo("sats", Sats.getTokenRanges(), BRC20.getTokenNetworks());

        // Btc
        setTokenInfo("btc", Btc.getTokenRanges(), Btc.getTokenNetworks());

        // ada
        setTokenInfo("ada", Ada.getTokenRanges(), Ada.getTokenNetworks());

        // amp
        setTokenInfo("amp", Amp.getTokenRanges(), Amp.getTokenNetworks());

        // atom
        setTokenInfo("atom", Atom.getTokenRanges(), Atom.getTokenNetworks());

        // bch
        setTokenInfo("bch", Bch.getTokenRanges(), Bch.getTokenNetworks());

        // bean
        setTokenInfo("bean", Bean.getTokenRanges(), Bean.getTokenNetworks());

        // bnb
        setTokenInfo("bnb", Bnb.getTokenRanges(), Bnb.getTokenNetworks());

        // comp
        setTokenInfo("comp", Comp.getTokenRanges(), Comp.getTokenNetworks());

        // cro
        setTokenInfo("cro", Cro.getTokenRanges(), Cro.getTokenNetworks());

        // crv
        setTokenInfo("crv", Crv.getTokenRanges(), Crv.getTokenNetworks());

        // dai
        setTokenInfo("dai", Dai.getTokenRanges(), Dai.getTokenNetworks());

        // doge
        setTokenInfo("doge", Doge.getTokenRanges(), Doge.getTokenNetworks());

        // dydx
        setTokenInfo("dydx", Dydx.getTokenRanges(), Dydx.getTokenNetworks());

        // etc
        setTokenInfo("etc", Etc.getTokenRanges(), Etc.getTokenNetworks());

        // eth
        setTokenInfo("eth", Eth.getTokenRanges(), Eth.getTokenNetworks());

        // fil
        setTokenInfo("fil", Fil.getTokenRanges(), Fil.getTokenNetworks());

        // grt
        setTokenInfo("grt", Grt.getTokenRanges(), Grt.getTokenNetworks());

        // gtc
        setTokenInfo("gtc", Gtc.getTokenRanges(), Gtc.getTokenNetworks());

        // gusd
        setTokenInfo("gusd", Gusd.getTokenRanges(), Gusd.getTokenNetworks());

        // imx
        setTokenInfo("imx", Imx.getTokenRanges(), Imx.getTokenNetworks());

        // inj
        setTokenInfo("inj", Inj.getTokenRanges(), Inj.getTokenNetworks());

        // leo
        setTokenInfo("leo", Leo.getTokenRanges(), Leo.getTokenNetworks());

        // link
        setTokenInfo("link", Link.getTokenRanges(), Link.getTokenNetworks());

        // lit
        setTokenInfo("lit", Lit.getTokenRanges(), Lit.getTokenNetworks());

        // matic
        setTokenInfo("matic", Matic.getTokenRanges(), Matic.getTokenNetworks());

        // mcrt
        setTokenInfo("mcrt", Mcrt.getTokenRanges(), Mcrt.getTokenNetworks());

        // nfp
        setTokenInfo("nfp", Nfp.getTokenRanges(), Nfp.getTokenNetworks());

        // people
        setTokenInfo(
            "people",
            People.getTokenRanges(),
            People.getTokenNetworks()
        );

        // shib
        setTokenInfo("shib", Shib.getTokenRanges(), Shib.getTokenNetworks());

        // sol
        setTokenInfo("sol", Sol.getTokenRanges(), Sol.getTokenNetworks());

        // spaceid
        setTokenInfo(
            "spaceid",
            SpaceId.getTokenRanges(),
            SpaceId.getTokenNetworks()
        );

        // ton
        setTokenInfo("ton", Ton.getTokenRanges(), Ton.getTokenNetworks());

        // trx
        setTokenInfo("trx", Trx.getTokenRanges(), Trx.getTokenNetworks());

        // tusd
        setTokenInfo("tusd", Tusd.getTokenRanges(), Tusd.getTokenNetworks());

        // uni
        setTokenInfo("uni", Uni.getTokenRanges(), Uni.getTokenNetworks());

        // usdc
        setTokenInfo("usdc", Usdc.getTokenRanges(), Usdc.getTokenNetworks());

        // usdd
        setTokenInfo("usdd", Usdd.getTokenRanges(), Usdd.getTokenNetworks());

        // usdt
        setTokenInfo("usdt", Usdt.getTokenRanges(), Usdt.getTokenNetworks());

        // wbtc
        setTokenInfo("wbtc", Wbtc.getTokenRanges(), Wbtc.getTokenNetworks());
    }

    function setTokenInfo(
        string memory tokenName,
        TokenInfoRanges memory ranges,
        TokenInfoNetwork[] memory networks
    ) private {
        TokenInfo storage tokenInfo = tokens[tokenName];

        delete tokenInfo.ranges;
        for (uint i = 0; i < ranges.ranges.length; i++) {
            tokenInfo.ranges.push(ranges.ranges[i]);
        }
        tokenInfo.rangeDecimals = ranges.rangeDecimals;

        delete tokenInfo.networks;
        uint8 maxDecimals = 0;
        for (uint i = 0; i < networks.length; i++) {
            if (maxDecimals < networks[i].decimals) {
                maxDecimals = networks[i].decimals;
            }
            tokenInfo.networks.push(networks[i]);
        }
        tokenInfo.maxDecimals = maxDecimals;
    }
}
