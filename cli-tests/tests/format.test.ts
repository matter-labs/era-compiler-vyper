import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --format json tests", () => {
    const zkvyperCommand = 'zkvyper';
    const vyperCommand = 'vyper';
    const format_args: string[] = [
        `combined_json`,
        `ir`,
        `ir_json`,
        `metadata`,
        `ast`,
        `abi`,
        `method_identifiers`,
        `layout`,
        `interface`,
        `external_interface`,
        `userdoc`,
        `devdoc`,
    ];

    //id1988
    for (let i = 0; i < format_args.length; i++) {
        describe(`Run ${zkvyperCommand} with --format ${format_args[i]}`, () => {
            const args = [`${paths.pathToBasicVyContract}`, `-f`, `${format_args[i]}`];
            const result = executeCommand(zkvyperCommand, args);

            it("command exit code = 0", () => {
                expect(result.exitCode).toBe(0);
            });

            it("No 'Error'/'Warning'/'Fail' in the output", () => {
                expect(result.output).not.toMatch(/\b([Ee]rror|[Ww]arning|[Ff]ail)\b/i);
            });

            it("vyper exit code == zkvyper exit code", () => {
                const vyperResult = executeCommand(vyperCommand, args);
                expect(vyperResult.exitCode).toBe(result.exitCode);
            });
        });
    }

    //id1989
    describe(`Run ${zkvyperCommand} with unsupported format and --format option`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `-f`, `llvm`];
        const result = executeCommand(zkvyperCommand, args);

        it("command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(Unsupported format type)/i);
        });
    });

    //id1990
    describe(`Run ${zkvyperCommand} with double --format option`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `-f`, `${format_args[0]}`, `-f`, `${format_args[1]}`];
        const result = executeCommand(zkvyperCommand, args);

        it("command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(cannot be used multiple times)/i);
        });
    });
});
