import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --llvm-verify-each tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1972
    describe(`Run ${zkvyperCommand} with --llvm-verify-each by default`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--llvm-verify-each`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1972:II
    describe(`Run only with --llvm-verify-each options`, () => {
        const args = [`--llvm-verify-each`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(arguments are required)/i);
        });
    });

    //id1973
    describe(`Run ${zkvyperCommand} with double --llvm-verify-each options`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--llvm-verify-each`, `--llvm-verify-each`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(was provided more than once)/i);
        });
    });

    //id1974
    describe(`Run ${zkvyperCommand} with incompatible contract and --llvm-verify-each option`, () => {
        const args = [`${paths.pathToLLVMContract}`, `--llvm-verify-each`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(vyper error)/i);
        });
    });
});
