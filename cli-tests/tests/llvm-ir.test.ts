import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --llvm-ir tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1963
    describe(`Run ${zkvyperCommand} with --llvm-ir by default`, () => {
        const args = [`${paths.pathToLLVMContract}`, `--llvm-ir`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1963:II
    describe(`Run only with --llvm-ir options`, () => {
        const args = [`--llvm-ir`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(input file is missing)/i);
        });
    });

    //id1964
    describe(`Run ${zkvyperCommand} with double --llvm-ir options`, () => {
        const args = [`${paths.pathToLLVMContract}`, `--llvm-ir`, `--llvm-ir`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(was provided more than once)/i);
        });
    });

    //id1965
    describe(`Run ${zkvyperCommand} with incompatible contract and --llvm-ir option`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--llvm-ir`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(expected top-level entity)/i);
        });
    });
});
