import type { ApiPromise } from '@polkadot/api';
import { AccountId32 } from '@polkadot/types/interfaces';
import type {
  PalletTeebagEnclave,
  PalletTeebagWorkerType,
} from '@polkadot/types/lookup';

/**
 * Return the Enclave registry information of the latest registered TEE worker.
 */
export async function getLastRegisteredEnclave(
  api: ApiPromise,
  workerType: PalletTeebagWorkerType['type'] = 'Identity'
): Promise<{ account: AccountId32; enclave: PalletTeebagEnclave }> {
  const identifiers = await api.query.teebag.enclaveIdentifier(workerType);
  const latestEnclaveId = identifiers[identifiers.length - 1];

  if (latestEnclaveId.isEmpty) {
    throw new Error('[enclave-sdk] No Enclave identifiers found');
  }

  const maybeEnclave = await api.query.teebag.enclaveRegistry(latestEnclaveId);

  if (maybeEnclave.isEmpty) {
    throw new Error(
      `[enclave-sdk] No Enclave Worker with ${latestEnclaveId.toHuman()} found`
    );
  }

  const enclave = maybeEnclave.unwrap();

  return { enclave, account: latestEnclaveId };
}
