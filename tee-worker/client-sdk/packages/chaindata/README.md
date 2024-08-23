# @litentry/chaindata

This library contains information about the available networks at Litentry, including its testnets.

## Installation

1. Install from NPM

   ```
   npm install @litentry/chaindata
   ```

2. Explore

   ```ts
   import { all, byId } from '@litentry/chaindata`;

   console.log(all);

   console.log(`Litentry's production RPC URL is: ${byId['litentry-prod'].rpcs[0].url}`);
   ```
