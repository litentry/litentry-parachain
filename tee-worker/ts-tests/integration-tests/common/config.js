// eslint-disable-next-line @typescript-eslint/no-var-requires, no-undef
import dotenv from 'dotenv';

dotenv.config({ path: `.env.${process.env.NODE_ENV}` });
