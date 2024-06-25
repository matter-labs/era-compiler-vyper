import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --eravm-assembly tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1961
    describe(`Run ${zkvyperCommand} with --eravm-assembly by default`, () => {
        const args = [`${paths.pathToBasicEraVMAssemblyContract}`, `--eravm-assembly`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1963:II
    describe(`Run with only --eravm-assembly option`, () => {
        const args = [`--eravm-assembly`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(No input sources provided)/i);
        });
    });

    //id1962
    describe(`Run ${zkvyperCommand} with double EraVM assembly options`, () => {
        const args = [`${paths.pathToBasicEraVMAssemblyContract}`, `--eravm-assembly`, `--eravm-assembly`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("--eravm-assembly error is presented", () => {
            expect(result.output).toMatch(/(The argument '--eravm-assembly' was provided more than once)/i);
        });
    });
});
