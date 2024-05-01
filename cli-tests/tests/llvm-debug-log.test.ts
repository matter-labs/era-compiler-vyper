import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --llvm-debug-logging tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1980
    describe(`Run ${zkvyperCommand} with --llvm-debug-logging by default`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--llvm-debug-logging`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1980:II
    describe(`Run only with --llvm-debug-logging options`, () => {
        const args = [`--llvm-debug-logging`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(arguments are required)/i);
        });
    });

    //id1981
    describe(`Run ${zkvyperCommand} with double --llvm-debug-logging options`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--llvm-debug-logging`, `--llvm-debug-logging`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(was provided more than once)/i);
        });
    });

    //id1965
    describe(`Run ${zkvyperCommand} with incompatible contract and --llvm-debug-logging option`, () => {
        const args = [`${paths.pathToLLVMContract}`, `--llvm-debug-logging`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(vyper error)/i);
        });
    });
});
