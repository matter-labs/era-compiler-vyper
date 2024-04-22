import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --suppress-warnings tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1961
    describe(`Run ${zkvyperCommand} with --suppress-warnings ecrecover`, () => {
        const args = [`${paths.pathToBasicZkasmContract}`, `--suppress-warnings`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("--zkasm output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });
});
