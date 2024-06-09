import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --disable-vyper-optimizer tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1939
    describe(`Run ${zkvyperCommand} with --disable-vyper-optimizer option`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--disable-vyper-optimizer`];
        const result = executeCommand(zkvyperCommand, args);

        it("command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Info is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
        });
    });

    //id1939:I
    describe(`Run only with --disable-vyper-optimizer option`, () => {
        const args = [`--disable-vyper-optimizer`];
        const result = executeCommand(zkvyperCommand, args);

        it("command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(the following arguments are required)/i);
        });
    });

    //id1979
    describe(`Run only with --disable-vyper-optimizer option`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--disable-vyper-optimizer`, `${paths.pathToBasicVyContract}`, `--disable-vyper-optimizer`];
        const result = executeCommand(zkvyperCommand, args);

        it("command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(cannot be used multiple times)/i);
        });
    });
});
