import { Consumer } from './util/consumer';

type ContextFactory<Context> = () => Promise<{
    context: Context;
    exit: Consumer<void>;
}>;

export class ContextManager<Context extends {}> {
    readonly enter: ContextFactory<Context>;
    constructor(enter: ContextFactory<Context>) {
        this.enter = enter;
    }

    async do<Result>(task: (context: Context) => Promise<Result>) {
        const { context, exit } = await this.enter();
        try {
            return await task(context);
        } finally {
            await exit();
        }
    }

    static blank(): ContextManager<{}> {
        return new ContextManager(async () => ({ context: {}, exit: async () => {} }));
    }

    map<NewContext extends {}>(mapping: (context: Context) => Promise<NewContext>): ContextManager<NewContext> {
        return new ContextManager(async () => {
            const { context, exit } = await this.enter();
            return { context: await mapping(context), exit };
        });
    }

    extend<Extension extends {}>(
        acquire: (context: Context) => Promise<Extension>,
        dispose: Consumer<Extension>
    ): ContextManager<Omit<Context, keyof Extension> & Extension> {
        return new ContextManager(async () => {
            const { context, exit } = await this.enter();
            try {
                const extension = await acquire(context);
                return {
                    context: { ...context, ...extension },
                    exit: async () => {
                        await dispose(extension);
                        await exit();
                    },
                };
            } catch (error) {
                await exit();
                throw error;
            }
        });
    }
}
