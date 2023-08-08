import { Command } from 'commander';
import path from 'path';
import { generateSchemas, LilSchemyOptions } from '../generator';
import { getRootFiles } from '../utils';

export const generateOpenApi = (cwd: string, config: LilSchemyOptions) => {
    if (config?.openApi) {
        const { openApi } = config;

        const files = getRootFiles(cwd, openApi.entry);

        console.debug("Searching for api paths in files %o", files);

        const result = generateSchemas({
            openApi: {
                base: JSON.stringify(openApi.base),
                entry: files,
                output: openApi.output || undefined
            }
        });

        if (result.openApi?.schema) {
            console.info(result.openApi.schema);
        } else if (result.openApi?.filepath) {
            console.info("OpenApi schema written to %s", result.openApi.filepath);
        }
    }
};

export default new Command('generate')
    .description('Generate one or more schemas')
    .option('-c, --config <config>', 'configuration module', 'schemy-config')
    .action(async (_, command: Command) => {
        let parentOptions = command.parent?.opts();
        const config = await import(path.resolve(parentOptions?.cwd, command.getOptionValue('config')));
        generateOpenApi(parentOptions?.cwd, config.default ?? config);
    });