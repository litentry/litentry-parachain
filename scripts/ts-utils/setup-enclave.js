"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
//run:npx ts-node setup-enclave.ts $enclaveAccount $mrenclave $accountPassword
//example:npx ts-node setup-enclave.ts 2KWd4sEmYj2VW42L2WUDDRKA4JwnKg76uoQ2keUBUwFHU9Dx a552654d1733c4054a3c7e5e86adf26b5d65c072b57b2550fe763821ebac54c6 123456
var Keyring = require("@polkadot/api").Keyring;
var initApis_1 = require("./initApis");
var hexToU8a = require("@polkadot/util").hexToU8a;
var colors_1 = __importDefault(require("colors"));
//100 token
var transferAmount = "100000000000000";
var enclaveAccount = process.argv[2];
var mrenclave = process.argv[3];
var block = process.argv[4];
// const accountPassword = process.argv[4];
//put account in private.json
// const private_account_pair = require("./private.json");
function transfer(api, Alice) {
    return __awaiter(this, void 0, void 0, function () {
        var _this = this;
        return __generator(this, function (_a) {
            console.log(colors_1.default.green("transfer start..."));
            return [2 /*return*/, new Promise(function (resolve, reject) { return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, api.tx.balances
                                    .transfer(enclaveAccount, transferAmount)
                                    .signAndSend(Alice, function (_a) {
                                    var status = _a.status, events = _a.events, dispatchError = _a.dispatchError;
                                    if (status.isInBlock || status.isFinalized) {
                                        events.forEach(function (_a) {
                                            var phase = _a.phase, _b = _a.event, data = _b.data, method = _b.method, section = _b.section;
                                            if (method === "Transfer" && section === "balances") {
                                                console.log(colors_1.default.green("transfer completed"));
                                                resolve("transfer done");
                                                return;
                                            }
                                        });
                                    }
                                })];
                            case 1:
                                _a.sent();
                                return [2 /*return*/];
                        }
                    });
                }); })];
        });
    });
}
function setTeerexAdmin(api, Alice) {
    return __awaiter(this, void 0, void 0, function () {
        var _this = this;
        return __generator(this, function (_a) {
            return [2 /*return*/, new Promise(function (resolve, reject) { return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, api.tx.sudo
                                    .sudo(api.tx.teerex.setAdmin("esqZdrqhgH8zy1wqYh1aLKoRyoRWLFbX9M62eKfaTAoK67pJ5"))
                                    .signAndSend(Alice, function (_a) {
                                    var status = _a.status, events = _a.events, dispatchError = _a.dispatchError;
                                    if (status.isInBlock || status.isFinalized) {
                                        if (dispatchError) {
                                            if (dispatchError.isModule) {
                                                // for module errors, we have the section indexed, lookup
                                                var decoded = api.registry.findMetaError(dispatchError.asModule);
                                                var docs = decoded.docs, name_1 = decoded.name, section = decoded.section;
                                                console.log(colors_1.default.red("".concat(section, ".").concat(name_1, ": ").concat(docs.join(" "))));
                                                reject("updateScheduledEnclave failed");
                                            }
                                            else {
                                                // Other, CannotLookup, BadOrigin, no extra info
                                                console.log(dispatchError.toString());
                                                reject("updateScheduledEnclave failed");
                                            }
                                        }
                                        else {
                                            console.log(colors_1.default.green("updateScheduledEnclave completed"));
                                            resolve("updateScheduledEnclave done");
                                        }
                                    }
                                })];
                            case 1:
                                _a.sent();
                                return [2 /*return*/];
                        }
                    });
                }); })];
        });
    });
}
function updateScheduledEnclave(api, Alice, block) {
    return __awaiter(this, void 0, void 0, function () {
        var _this = this;
        return __generator(this, function (_a) {
            return [2 /*return*/, new Promise(function (resolve, reject) { return __awaiter(_this, void 0, void 0, function () {
                    return __generator(this, function (_a) {
                        switch (_a.label) {
                            case 0: return [4 /*yield*/, api.tx.teerex.updateScheduledEnclave(block, hexToU8a("0x".concat(mrenclave)))
                                    .signAndSend(Alice, function (_a) {
                                    var status = _a.status, events = _a.events, dispatchError = _a.dispatchError;
                                    if (status.isInBlock || status.isFinalized) {
                                        if (dispatchError) {
                                            if (dispatchError.isModule) {
                                                // for module errors, we have the section indexed, lookup
                                                var decoded = api.registry.findMetaError(dispatchError.asModule);
                                                var docs = decoded.docs, name_2 = decoded.name, section = decoded.section;
                                                console.log(colors_1.default.red("".concat(section, ".").concat(name_2, ": ").concat(docs.join(" "))));
                                                reject("updateScheduledEnclave failed");
                                            }
                                            else {
                                                // Other, CannotLookup, BadOrigin, no extra info
                                                console.log(dispatchError.toString());
                                                reject("updateScheduledEnclave failed");
                                            }
                                        }
                                        else {
                                            console.log(colors_1.default.green("updateScheduledEnclave completed"));
                                            resolve("updateScheduledEnclave done");
                                        }
                                    }
                                })];
                            case 1:
                                _a.sent();
                                return [2 /*return*/];
                        }
                    });
                }); })];
        });
    });
}
function main() {
    return __awaiter(this, void 0, void 0, function () {
        var keyring, defaultAPI, Alice;
        return __generator(this, function (_a) {
            switch (_a.label) {
                case 0:
                    keyring = new Keyring({ type: "sr25519" });
                    // let signAccount = keyring.addFromJson(private_account_pair);
                    console.log(colors_1.default.green("account unlock..."));
                    return [4 /*yield*/, (0, initApis_1.initApi)()];
                case 1:
                    defaultAPI = (_a.sent()).defaultAPI;
                    Alice = keyring.addFromUri("//Alice", { name: "Alice default" });
                    return [4 /*yield*/, transfer(defaultAPI, Alice)];
                case 2:
                    _a.sent();
                    return [4 /*yield*/, setTeerexAdmin(defaultAPI, Alice)];
                case 3:
                    _a.sent();
                    return [4 /*yield*/, updateScheduledEnclave(defaultAPI, Alice, block)];
                case 4:
                    _a.sent();
                    console.log(colors_1.default.green("done"));
                    process.exit();
                    return [2 /*return*/];
            }
        });
    });
}
main();
