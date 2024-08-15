/**
 * Reference:
 * @see Parachain ts-tests https://github.com/litentry/litentry-parachain/blob/dev/tee-worker/ts-tests/interfaces
 * @see Polkadot.js Docs https://polkadot.js.org/docs/api/start/typescript.user
 */
export default {
  types: {
    LitentryIdentity: {
      _enum: {
        Twitter: 'Text',
        Discord: 'Text',
        Github: 'Text',
        Substrate: 'AccountId',
        Evm: 'AccountId20',
        Bitcoin: 'AccountId33',
        Solana: 'AccountId',
      },
    },

    /// Validation Data
    LitentryValidationData: {
      _enum: {
        Web2Validation: 'Web2ValidationData',
        Web3Validation: 'Web3ValidationData',
      },
    },
    Web2ValidationData: {
      _enum: {
        Twitter: 'TwitterValidationData',
        Discord: 'DiscordValidationData',
      },
    },
    TwitterValidationData: {
      _enum: {
        PublicTweet: 'PublicTweet',
        OAuth2: 'TwitterOAuth2',
      },
    },
    PublicTweet: {
      tweet_id: 'Vec<u8>',
    },
    TwitterOAuth2: {
      code: 'Vec<u8>',
      state: 'Vec<u8>',
      redirect_uri: 'Vec<u8>',
    },
    DiscordValidationData: {
      _enum: {
        PublicMessage: 'PublicMessage',
        OAuth2: 'DiscordOAuth2',
      },
    },
    PublicMessage: {
      channel_id: 'Vec<u8>',
      message_id: 'Vec<u8>',
      guild_id: 'Vec<u8>',
    },
    DiscordOAuth2: {
      code: 'Vec<u8>',
      redirect_uri: 'Vec<u8>',
    },
    Web3ValidationData: {
      _enum: {
        Substrate: 'Web3CommonValidationData',
        Evm: 'Web3CommonValidationData',
        Bitcoin: 'Web3CommonValidationData',
        Solana: 'Web3CommonValidationData',
      },
    },
    Web3CommonValidationData: {
      message: 'Vec<u8>',
      signature: 'LitentryMultiSignature',
    },

    LitentryMultiSignature: {
      _enum: {
        Ed25519: 'ed25519::Signature',
        Sr25519: 'sr25519::Signature',
        Ecdsa: 'ecdsa::Signature',
        Ethereum: 'EthereumSignature',
        Bitcoin: '([u8;65])',
      },
    },

    IdentityGenericEvent: {
      who: 'AccountId',
      identity: 'LitentryIdentity',
      id_graph: 'Vec<(LitentryIdentity, IdentityContext)>',
    },

    IdentityContext: {
      linkBlock: 'BlockNumber',
      web3networks: 'Vec<Web3Network>',
      status: 'IdentityStatus',
    },
    Web3Network: {
      _enum: [
        'Polkadot',
        'Kusama',
        'Litentry',
        'Litmus',
        'LitentryRococo',
        'Khala',
        'SubstrateTestnet',
        'Ethereum',
        'Bsc',
        'BitcoinP2tr',
        'BitcoinP2pkh',
        'BitcoinP2sh',
        'BitcoinP2wpkh',
        'BitcoinP2wsh',
        'Polygon',
        'Arbitrum',
        'Solana',
        'Combo',
      ],
    },
    IdentityStatus: {
      _enum: ['Active', 'Inactive'],
    },
  },
};
