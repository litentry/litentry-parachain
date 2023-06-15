"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
exports.create = void 0;
require("@polkadot/api/augment");
require("@polkadot/types/augment");
const definitions_1 = require("../parachain-interfaces/definitions");
const api_1 = require("@polkadot/api");
async function create(provider) {
    return await api_1.ApiPromise.create({ provider, types: definitions_1.identity.types });
}
exports.create = create;
