import dotenv from 'dotenv';

// eslint-disable-next-line @typescript-eslint/no-var-requires, no-undef
dotenv.config({ path: `.env.${process.env.NODE_ENV || 'local'}` });
