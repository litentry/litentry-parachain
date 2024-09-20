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

import "./StringCleaner.sol";

library AssertionLogic {
    enum Op {
        GreaterThan,
        LessThan,
        GreaterEq,
        LessEq,
        Equal,
        NotEq
    }

    struct Condition {
        string src;
        Op op;
        string dst;
        CompositeCondition cc;
    }

    struct CompositeCondition {
        Condition[] conditions;
        bool isAnd; // true for 'And', false for 'Or'
    }

    function newConditionWithoutSubCc(
        string memory src,
        Op op,
        string memory dst
    ) internal pure returns (Condition memory) {
        CompositeCondition memory subCc;
        return Condition(src, op, dst, subCc);
    }

    function addCondition(
        CompositeCondition memory cc,
        uint256 i,
        string memory src,
        Op op,
        string memory dst
    ) internal pure {
        CompositeCondition memory subCc;
        cc.conditions[i] = Condition(src, op, dst, subCc);
    }

    function addCompositeCondition(
        CompositeCondition memory cc,
        uint256 i,
        CompositeCondition memory subCc
    ) internal pure {
        cc.conditions[i] = Condition("", Op.Equal, "", subCc);
    }

    function andOp(
        CompositeCondition memory cc,
        uint256 i,
        string memory src,
        Op op,
        string memory dst
    ) internal pure returns (CompositeCondition memory) {
        addCondition(cc, i, src, op, dst);
        cc.isAnd = true;
        return cc;
    }

    function orOp(
        CompositeCondition memory cc,
        uint256 i,
        string memory src,
        Op op,
        string memory dst
    ) internal pure returns (CompositeCondition memory) {
        addCondition(cc, i, src, op, dst);
        cc.isAnd = false;
        return cc;
    }

    function toString(
        CompositeCondition memory cc
    ) internal pure returns (string memory) {
        string memory result = "{";

        if (cc.conditions.length > 0) {
            result = string(
                abi.encodePacked(result, cc.isAnd ? '"and":[' : '"or":[')
            );
            for (uint256 i = 0; i < cc.conditions.length; i++) {
                Condition memory c = cc.conditions[i];
                if (i > 0) {
                    result = string(abi.encodePacked(result, ","));
                }
                if (c.cc.conditions.length > 0) {
                    result = string(abi.encodePacked(result, toString(c.cc)));
                } else {
                    result = string(abi.encodePacked(result, toString(c)));
                }
            }
            result = string(abi.encodePacked(result, "]"));
        }

        result = string(abi.encodePacked(result, "}"));

        // the assembled result may contain some invisible characters that cause the unit test failure, so we need to clear it here.
        return StringCleaner.cleanString(result);
    }

    function toString(
        Condition memory condition
    ) internal pure returns (string memory) {
        string memory result = string(
            abi.encodePacked(
                '{"src":"',
                condition.src,
                '","op":"',
                operatorToString(condition.op),
                '","dst":"',
                condition.dst,
                '"}'
            )
        );

        // the assembled result may contain some invisible characters that cause the unit test failure, so we need to clear it here.
        return StringCleaner.cleanString(result);
    }

    function operatorToString(Op op) internal pure returns (string memory) {
        if (op == Op.Equal) {
            return "==";
        } else if (op == Op.GreaterThan) {
            return ">";
        } else if (op == Op.LessThan) {
            return "<";
        } else if (op == Op.GreaterEq) {
            return ">=";
        } else if (op == Op.LessEq) {
            return "<=";
        } else if (op == Op.NotEq) {
            return "!=";
        }

        revert("Unsupported operator");
    }
}
