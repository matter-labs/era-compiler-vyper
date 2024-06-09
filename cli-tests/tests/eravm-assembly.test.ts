import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --eravm tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1961
    describe(`Run ${zkvyperCommand} with --eravm by default`, () => {
        const args = [`${paths.pathToBasicEraVMAssemblyContract}`, `--eravm`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("--eravm output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1962
    describe(`Run ${zkvyperCommand} with double EraVM assembly options`, () => {
        const args = [`${paths.pathToBasicEraVMAssemblyContract}`, `--eravm`, `--eravm`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("--eravm error is presented", () => {
            expect(result.output).toMatch(/(The argument '--eravm' was provided more than once)/i);
        });
    });
});
