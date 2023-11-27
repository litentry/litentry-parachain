export async function handleVcEvents(
    events: any[],
    method: 'VCIssued' | 'VCDisabled' | 'VCRevoked' | 'Failed'
): Promise<any> {
    const results: any = [];
    for (let k = 0; k < events.length; k++) {
        switch (method) {
            case 'VCIssued':
                results.push({
                    account: events[k].data.account.toHex(),
                    index: events[k].data.index.toHex(),
                });
                break;
            case 'VCDisabled':
                results.push(events[k].data.index.toHex());
                break;
            case 'VCRevoked':
                results.push(events[k].data.index.toHex());
                break;
            case 'Failed':
                results.push(events[k].data.detail.toHuman());
                break;
            default:
                break;
        }
    }
    return [...results];
}

export const jsonSchema = {
    type: 'object',
    properties: {
        id: {
            type: 'string',
        },
        type: {
            type: 'array',
        },
        issuer: {
            type: 'object',
            properties: {
                id: {
                    type: 'string',
                },
                name: {
                    type: 'string',
                },
                shard: {
                    type: 'string',
                },
            },
        },
        issuanceDate: {
            type: 'string',
        },
        credentialSubject: {
            type: 'object',
            properties: {
                id: {
                    type: 'string',
                },
                description: {
                    type: 'string',
                },
                type: {
                    type: 'string',
                },
                tag: {
                    type: 'array',
                },
                assertions: {
                    type: 'array',
                    items: {
                        type: 'object',
                    },
                },
                values: {
                    type: 'array',
                    items: {
                        type: 'boolean',
                    },
                },
                endpoint: {
                    type: 'string',
                },
            },
            required: ['id', 'description', 'type', 'assertions', 'values', 'endpoint'],
        },
        proof: {
            type: 'object',
            properties: {
                created: {
                    type: 'string',
                },
                type: {
                    enum: ['Ed25519Signature2020'],
                },
                proofPurpose: {
                    enum: ['assertionMethod'],
                },
                proofValue: {
                    type: 'string',
                },
                verificationMethod: {
                    type: 'string',
                },
            },
        },
    },
    required: ['id', 'type', 'credentialSubject', 'issuer', 'issuanceDate', 'proof'],
};
