export async function handleVcEvents(events: any[], method: 'VCIssued' | 'Failed'): Promise<any> {
    const results: any = [];
    for (let k = 0; k < events.length; k++) {
        switch (method) {
            case 'VCIssued':
                results.push({
                    identity: events[k].data.identity.toHex(),
                    index: events[k].data.index.toHex(),
                });
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

// https://github.com/litentry/litentry-parachain/tree/dev/tee-worker/litentry/core/assertion-build/src
export const defaultAssertions = [
    {
        description: 'Have identified at least one account/address in both Web2 and Web3.',
        assertion: {
            A1: 'A1',
        },
    },
    {
        description: 'The user is a member of Litentry Discord.',
        assertion: {
            A2: 'A2',
        },
    },

    {
        description:
            'Have commented in Litentry Discord #🪂id-hubber channel. Channel link: https://discord.com/channels/807161594245152800/1093886939746291882',
        assertion: {
            A3: ['A3', 'A3', 'A3'],
        },
    },

    {
        description: 'The length of time a user continues to hold LIT token',
        assertion: {
            A4: '10',
        },
    },
    {
        description: 'The length of time a user continues to hold DOT token',
        assertion: {
            A7: '10.01',
        },
    },
    {
        description:
            'The range of number of transactions a user has made for a specific token on all supported networks(Litentry)',
        assertion: {
            A8: ['Litentry'],
        },
    },
    {
        description:
            'The user is an early bird user of the IdentityHub EVM version and has generated at least 1 credential during 2023 Aug 14th ~ Aug 21st.',
        assertion: {
            A20: 'A20',
        },
    },

    // Achainable
    // https://www.notion.so/web3builders/Assertion-interface-9126ba85a925417a922f2c6ae5d62e87
    {
        description: `A trader or liquidity provider of Uniswap V2 or V3
                      Uniswap V2 Factory Contract: 0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f
                      Uniswap V3 Factory Contract: 0x1f98431c8ad98523631ae4a59f267346ea31f984`,
        assertion: {
            Achainable: {
                Basic: {
                    name: 'Uniswap V2/V3 user',
                    chain: ['Ethereum'],
                },
            },
        },
    },
    {
        description: 'The number of ETH tokens hold > 0',
        assertion: {
            Achainable: {
                Amount: {
                    name: 'Balance over {amount}',
                    chain: ['Ethereum'],

                    amount: '0',
                },
            },
        },
    },
    {
        description: 'The class of year that Ethereum account was created (must have on-chain records)',
        assertion: {
            Achainable: {
                ClassOfYear: {
                    name: 'Account created between {dates}',
                    chain: ['Ethereum'],
                },
            },
        },
    },
    {
        description: 'A deployer of a smart contract on Ethereum',
        assertion: {
            Achainable: {
                Amount: {
                    name: 'Created over {amount} contracts',
                    chain: ['Ethereum'],
                    amount: '0',
                },
            },
        },
    },

    // SORA
    {
        description:
            'Congratulations on your participation in our first quiz in collaboration with our partner, SORA. You have embarked on an exciting educational journey, exploring the world of DeFi & Web3 Identity, we truly appreciate your curiosity and dedication.',
        assertion: {
            GenericDiscordRole: {
                SoraQuiz: 'Attendee',
            },
        },
    },
    // VIP3
    {
        description: 'VIP3 Silver Card Holder',
        assertion: {
            VIP3MembershipCard: 'Silver',
        },
    },
];

// In both cases as below, it's sufficient to check if the condition is valid, should be invalid.
// For the 'oneblock' assertion, need to configure the Polkadot/Kusma address,
// and for 'bnb,' need to configure the NODEREAL_API_KEY
// We cannot submit these two types of data involving privacy(from @zhouhui), so we only need to test that their DI response is invalid and that the RequestVCFailed event is received, which should be tested separately from the defaultAssertions.
export const unconfiguredAssertions = [
    // Oneblock
    {
        description: 'A participant to the course co-created by OneBlock+ and Parity',
        assertion: {
            oneblock: 'CourseParticipation',
        },
    },

    // BNB domain
    {
        description: 'Holding a certain amount of bnb domain names',
        assertion: {
            BnbDomainHolding: 'BnbDomainHolding',
        },
    },
    {
        description: 'Holding a certain amount of 000-999.bnb domain names',
        assertion: {
            BnbDigitDomainClub: 'Bnb999ClubMember',
        },
    },
];
