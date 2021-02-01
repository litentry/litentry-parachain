"use strict";

var _interopRequireDefault = require("@babel/runtime/helpers/interopRequireDefault");

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.progress = progress;

var _defineProperty2 = _interopRequireDefault(require("@babel/runtime/helpers/defineProperty"));

var _rxjs = require("rxjs");

var _operators = require("rxjs/operators");

var _util = require("@polkadot/util");

var _util2 = require("../util");

function ownKeys(object, enumerableOnly) { var keys = Object.keys(object); if (Object.getOwnPropertySymbols) { var symbols = Object.getOwnPropertySymbols(object); if (enumerableOnly) symbols = symbols.filter(function (sym) { return Object.getOwnPropertyDescriptor(object, sym).enumerable; }); keys.push.apply(keys, symbols); } return keys; }

function _objectSpread(target) { for (var i = 1; i < arguments.length; i++) { var source = arguments[i] != null ? arguments[i] : {}; if (i % 2) { ownKeys(Object(source), true).forEach(function (key) { (0, _defineProperty2.default)(target, key, source[key]); }); } else if (Object.getOwnPropertyDescriptors) { Object.defineProperties(target, Object.getOwnPropertyDescriptors(source)); } else { ownKeys(Object(source)).forEach(function (key) { Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key)); }); } } return target; }

function createDerive(api, info, [currentSlot, epochIndex, epochOrGenesisStartSlot, activeEraStartSessionIndex]) {
  const epochStartSlot = epochIndex.mul(info.sessionLength).iadd(epochOrGenesisStartSlot);
  const sessionProgress = currentSlot.sub(epochStartSlot);
  const eraProgress = info.currentIndex.sub(activeEraStartSessionIndex).imul(info.sessionLength).iadd(sessionProgress);
  return _objectSpread(_objectSpread({}, info), {}, {
    eraProgress: api.registry.createType('BlockNumber', eraProgress),
    sessionProgress: api.registry.createType('BlockNumber', sessionProgress)
  });
}

function queryAura(api) {
  return api.derive.session.info().pipe((0, _operators.map)(info => _objectSpread(_objectSpread({}, info), {}, {
    eraProgress: api.registry.createType('BlockNumber'),
    sessionProgress: api.registry.createType('BlockNumber')
  })));
}

function queryBabe(api) {
  return api.derive.session.info().pipe((0, _operators.switchMap)(info => (0, _rxjs.combineLatest)([(0, _rxjs.of)(info), api.queryMulti([api.query.babe.currentSlot, api.query.babe.epochIndex, api.query.babe.genesisSlot, [api.query.staking.erasStartSessionIndex, info.activeEra]])])), (0, _operators.map)(([info, [currentSlot, epochIndex, genesisSlot, optStartIndex]]) => [info, [currentSlot, epochIndex, genesisSlot, optStartIndex.unwrapOr(api.registry.createType('SessionIndex', 1))]]));
}

function queryBabeNoHistory(api) {
  return (0, _rxjs.combineLatest)([api.derive.session.info(), api.queryMulti([api.query.babe.currentSlot, api.query.babe.epochIndex, api.query.babe.genesisSlot, api.query.staking.currentEraStartSessionIndex])]);
}
/**
 * @description Retrieves all the session and era query and calculates specific values on it as the length of the session and eras
 */


function progress(instanceId, api) {
  return (0, _util2.memo)(instanceId, () => api.consts.babe ? ((0, _util.isFunction)(api.query.staking.erasStartSessionIndex) ? queryBabe(api) // 2.x with Babe
  : queryBabeNoHistory(api)).pipe((0, _operators.map)(([info, slots]) => createDerive(api, info, slots))) : queryAura(api));
}