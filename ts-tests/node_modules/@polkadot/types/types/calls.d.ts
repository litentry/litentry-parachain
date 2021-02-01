import { FunctionMetadataLatest } from '../interfaces/metadata';
import { Call } from '../interfaces/runtime';
export interface CallBase {
    callIndex: Uint8Array;
    meta: FunctionMetadataLatest;
    method: string;
    section: string;
    toJSON: () => any;
}
export interface CallFunction extends CallBase {
    (...args: any[]): Call;
}
export declare type Calls = Record<string, CallFunction>;
export declare type ModulesWithCalls = Record<string, Calls>;
