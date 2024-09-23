import { byId } from '@litentry/chaindata';

// Change this to the environment you want to test
const chain = byId['litentry-prod'];

export const nodeEndpoint: string = chain.rpcs[0].url;
export const enclaveEndpoint: string = chain.enclaveRpcs[0].url;
