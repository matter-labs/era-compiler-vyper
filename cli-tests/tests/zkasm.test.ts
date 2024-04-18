import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --zkasm tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1961
    describe(`Run ${zkvyperCommand} with --zkasm by default`, () => {
        const args = [`${paths.pathToBasicZkasmContract}`, `--zkasm`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("--zkasm output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1962
    describe(`Run ${zkvyperCommand} with double zkasm options`, () => {
        const args = [`${paths.pathToBasicZkasmContract}`, `--zkasm`, `--zkasm`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("--zkasm error is presented", () => {
            expect(result.output).toMatch(/(The argument '--zkasm' was provided more than once)/i);
        });
    });
});
