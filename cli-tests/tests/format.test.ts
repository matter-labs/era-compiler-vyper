import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --format json tests", () => {
    const zkvyperCommand = 'zkvyper';
    const vyperCommand = 'vyper';
    const format_args: string[] = [
        `bytecode`,
        `bytecode_runtime`,
        `blueprint_bytecode`,
        `abi`,
        `abi_python`,
        `source_map`,
        `method_identifiers`,
        `userdoc`,
        `devdoc`,
        `combined_json`,
        `layout`,
        `ast`,
        `interface`,
        `external_interface`,
        `opcodes`,
        `opcodes_runtime`,
        `ir`,
        `ir_json`,
        `ir_runtime`,
        `asm`
        // `hex-ir` // does not work
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
    describe(`Run ${zkvyperCommand} with Unsupported format and --format option`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `-f`, `llvm`];
        const result = executeCommand(zkvyperCommand, args);

        it("command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/([Uu]nsupported format)/i);
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
