import "@polkadot/api-augment";
import { ApiPromise } from "@polkadot/api";

async function foo() {
    const api = ApiPromise.create();
    const val = (await api).createType;
}
