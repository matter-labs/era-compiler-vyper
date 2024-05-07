import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --recursive-process tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1985
    describe(`Run ${zkvyperCommand} with recursive-process option`, () => {
        const args = [`--recursive-process`, `<`, `${paths.pathToJSONContract}`];
        const result = executeCommand(zkvyperCommand, args);

        it("command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("error is presented", () => {
            expect(result.output).toMatch(/(unknown variant ``, expected one of `Vyper`, `LLVMIR`, `ZKASM`)/i);
        });
    });
});
