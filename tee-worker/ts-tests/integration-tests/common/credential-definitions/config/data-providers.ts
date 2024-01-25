export type DataProvider = {
    id: string;
    name: string;
    url: string;
};

export const litentry: DataProvider = {
    id: 'litentry-tee-worker',
    name: 'Litentry TEE Worker',
    url: 'https://litentry.com/',
};

export const litentryIndexer: DataProvider = {
    id: 'litentry-indexer',
    name: 'Litentry Indexer',
    url: 'https://litentry.com/',
};

export const discord: DataProvider = {
    id: 'discord-api',
    name: 'Discord Official API',
    url: 'https://discord.com/build',
};

export const achainable: DataProvider = {
    id: 'achainable-api',
    name: 'Achainable',
    url: 'https://www.achainable.com/',
};

export const twitter: DataProvider = {
    id: 'twitter-api',
    name: 'Twitter Official API',
    url: 'https://developer.twitter.com/',
};

export const oneBlock: DataProvider = {
    id: 'one-block',
    name: 'OneBlock+',
    url: 'https://linktr.ee/oneblock_',
};

export const vip3: DataProvider = {
    id: 'vip3',
    name: 'VIP3',
    url: 'https://dappapi.vip3.io/',
};
