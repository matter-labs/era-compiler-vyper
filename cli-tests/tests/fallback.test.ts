import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';

describe("Set of --fallback-Oz tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1938
    describe(`Run ${zkvyperCommand} with fallback-Oz option`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--fallback-Oz`];
        const result = executeCommand(zkvyperCommand, args);

        it("command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Info is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1938:I
    describe(`Run only with fallback-Oz option`, () => {
        const args = [`--fallback-Oz`];
        const result = executeCommand(zkvyperCommand, args);

        it("command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("error is presented", () => {
            expect(result.output).toMatch(/(the following arguments are required)/i);
        });
    });
});
