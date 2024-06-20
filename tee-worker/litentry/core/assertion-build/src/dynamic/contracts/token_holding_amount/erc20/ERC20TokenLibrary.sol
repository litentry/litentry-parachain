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
import { Mcrt } from "./Mcrt.sol";
import { Nfp } from "./Nfp.sol";
import { People } from "./People.sol";
import { Shib } from "./Shib.sol";
import { Sol } from "./Sol.sol";
import { SpaceId } from "./SpaceId.sol";
import { Ton } from "./Ton.sol";
import { Trx } from "./Trx.sol";
import { Tusd } from "./Tusd.sol";
import { Uni } from "./Uni.sol";
import { Usdc } from "./Usdc.sol";
import { Usdt } from "./Usdt.sol";
import { Wbtc } from "./Wbtc.sol";
library ERC20TokenLibrary {
	function getAdaInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Ada.getTokenName();
		uint256[] memory ranges = Ada.getTokenRanges();
		string memory bscAddress = Ada.getTokenBscAddress();
		string memory ethereumAddress = Ada.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getAmpInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Amp.getTokenName();
		uint256[] memory ranges = Amp.getTokenRanges();
		string memory bscAddress = Amp.getTokenBscAddress();
		string memory ethereumAddress = Amp.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}

	function getAtomInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Atom.getTokenName();
		uint256[] memory ranges = Atom.getTokenRanges();
		string memory bscAddress = Atom.getTokenBscAddress();
		string memory ethereumAddress = Atom.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getBchInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Bch.getTokenName();
		uint256[] memory ranges = Bch.getTokenRanges();
		string memory bscAddress = Bch.getTokenBscAddress();
		string memory ethereumAddress = Bch.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getBeanInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Bean.getTokenName();
		uint256[] memory ranges = Bean.getTokenRanges();
		string memory bscAddress = Bean.getTokenBscAddress();
		string memory ethereumAddress = Bean.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getBnbInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Bnb.getTokenName();
		uint256[] memory ranges = Bnb.getTokenRanges();
		string memory bscAddress = Bnb.getTokenBscAddress();
		string memory ethereumAddress = Bnb.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getCompInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Comp.getTokenName();
		uint256[] memory ranges = Comp.getTokenRanges();
		string memory bscAddress = Comp.getTokenBscAddress();
		string memory ethereumAddress = Comp.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getCroInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Cro.getTokenName();
		uint256[] memory ranges = Cro.getTokenRanges();
		string memory bscAddress = Cro.getTokenBscAddress();
		string memory ethereumAddress = Cro.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getCrvInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Crv.getTokenName();
		uint256[] memory ranges = Crv.getTokenRanges();
		string memory bscAddress = Crv.getTokenBscAddress();
		string memory ethereumAddress = Crv.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getDaiInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Dai.getTokenName();
		uint256[] memory ranges = Dai.getTokenRanges();
		string memory bscAddress = Dai.getTokenBscAddress();
		string memory ethereumAddress = Dai.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getDogeInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Doge.getTokenName();
		uint256[] memory ranges = Doge.getTokenRanges();
		string memory bscAddress = Doge.getTokenBscAddress();
		string memory ethereumAddress = Doge.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}

	function getDydxInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Dydx.getTokenName();
		uint256[] memory ranges = Dydx.getTokenRanges();
		string memory bscAddress = Dydx.getTokenBscAddress();
		string memory ethereumAddress = Dydx.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getEtcInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Etc.getTokenName();
		uint256[] memory ranges = Etc.getTokenRanges();
		string memory bscAddress = Etc.getTokenBscAddress();
		string memory ethereumAddress = Etc.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getEthInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Eth.getTokenName();
		uint256[] memory ranges = Eth.getTokenRanges();
		string memory bscAddress = Eth.getTokenBscAddress();
		string memory ethereumAddress = Eth.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getFilInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Fil.getTokenName();
		uint256[] memory ranges = Fil.getTokenRanges();
		string memory bscAddress = Fil.getTokenBscAddress();
		string memory ethereumAddress = Fil.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getGrtInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Grt.getTokenName();
		uint256[] memory ranges = Grt.getTokenRanges();
		string memory bscAddress = Grt.getTokenBscAddress();
		string memory ethereumAddress = Grt.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getGtcInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Gtc.getTokenName();
		uint256[] memory ranges = Gtc.getTokenRanges();
		string memory bscAddress = Gtc.getTokenBscAddress();
		string memory ethereumAddress = Gtc.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getGusdInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Gusd.getTokenName();
		uint256[] memory ranges = Gusd.getTokenRanges();
		string memory bscAddress = Gusd.getTokenBscAddress();
		string memory ethereumAddress = Gusd.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getImxInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Imx.getTokenName();
		uint256[] memory ranges = Imx.getTokenRanges();
		string memory bscAddress = Imx.getTokenBscAddress();
		string memory ethereumAddress = Imx.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getInjInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Inj.getTokenName();
		uint256[] memory ranges = Inj.getTokenRanges();
		string memory bscAddress = Inj.getTokenBscAddress();
		string memory ethereumAddress = Inj.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getLeoInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Leo.getTokenName();
		uint256[] memory ranges = Leo.getTokenRanges();
		string memory bscAddress = Leo.getTokenBscAddress();
		string memory ethereumAddress = Leo.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getLinkInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Link.getTokenName();
		uint256[] memory ranges = Link.getTokenRanges();
		string memory bscAddress = Link.getTokenBscAddress();
		string memory ethereumAddress = Link.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getLitInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Lit.getTokenName();
		uint256[] memory ranges = Lit.getTokenRanges();
		string memory bscAddress = Lit.getTokenBscAddress();
		string memory ethereumAddress = Lit.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getMaticInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Matic.getTokenName();
		uint256[] memory ranges = Matic.getTokenRanges();
		string memory bscAddress = Matic.getTokenBscAddress();
		string memory ethereumAddress = Matic.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getMcrtInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Mcrt.getTokenName();
		uint256[] memory ranges = Mcrt.getTokenRanges();
		string memory bscAddress = Mcrt.getTokenBscAddress();
		string memory ethereumAddress = Mcrt.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getNfpInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Nfp.getTokenName();
		uint256[] memory ranges = Nfp.getTokenRanges();
		string memory bscAddress = Nfp.getTokenBscAddress();
		string memory ethereumAddress = Nfp.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getPeopleInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = People.getTokenName();
		uint256[] memory ranges = People.getTokenRanges();
		string memory bscAddress = People.getTokenBscAddress();
		string memory ethereumAddress = People.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getShibInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Shib.getTokenName();
		uint256[] memory ranges = Shib.getTokenRanges();
		string memory bscAddress = Shib.getTokenBscAddress();
		string memory ethereumAddress = Shib.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getSolInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Sol.getTokenName();
		uint256[] memory ranges = Sol.getTokenRanges();
		string memory bscAddress = Sol.getTokenBscAddress();
		string memory ethereumAddress = Sol.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getSpaceIdInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = SpaceId.getTokenName();
		uint256[] memory ranges = SpaceId.getTokenRanges();
		string memory bscAddress = SpaceId.getTokenBscAddress();
		string memory ethereumAddress = SpaceId.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getTonInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Ton.getTokenName();
		uint256[] memory ranges = Ton.getTokenRanges();
		string memory bscAddress = Ton.getTokenBscAddress();
		string memory ethereumAddress = Ton.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getTrxInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Trx.getTokenName();
		uint256[] memory ranges = Trx.getTokenRanges();
		string memory bscAddress = Trx.getTokenBscAddress();
		string memory ethereumAddress = Trx.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getTusdInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Tusd.getTokenName();
		uint256[] memory ranges = Tusd.getTokenRanges();
		string memory bscAddress = Tusd.getTokenBscAddress();
		string memory ethereumAddress = Tusd.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getUniInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Uni.getTokenName();
		uint256[] memory ranges = Uni.getTokenRanges();
		string memory bscAddress = Uni.getTokenBscAddress();
		string memory ethereumAddress = Uni.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getUsdcInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Usdc.getTokenName();
		uint256[] memory ranges = Usdc.getTokenRanges();
		string memory bscAddress = Usdc.getTokenBscAddress();
		string memory ethereumAddress = Usdc.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getUsdtInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Usdt.getTokenName();
		uint256[] memory ranges = Usdt.getTokenRanges();
		string memory bscAddress = Usdt.getTokenBscAddress();
		string memory ethereumAddress = Usdt.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
	function getWbtcInfo()
		internal
		pure
		returns (string memory, uint256[] memory, string memory, string memory)
	{
		string memory tokenName = Wbtc.getTokenName();
		uint256[] memory ranges = Wbtc.getTokenRanges();
		string memory bscAddress = Wbtc.getTokenBscAddress();
		string memory ethereumAddress = Wbtc.getTokenEthereumAddress();
		return (tokenName, ranges, bscAddress, ethereumAddress);
	}
}
