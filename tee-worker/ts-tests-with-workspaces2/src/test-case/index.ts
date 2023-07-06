import { TypeRegistry } from '../sidechain/index'
import { LitentryPrimitivesIdentitySubstrateNetwork } from '../sidechain/index'
import { Assertion } from '../parachain/index'
const sidechainRegistry = new TypeRegistry();



const network: LitentryPrimitivesIdentitySubstrateNetwork['type'] = 'Litentry'

const assertion: Assertion['type'] = 'A1'
console.log(network, assertion);
