import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --optimization tests", () => {
    const zkvyperCommand = 'zkvyper';
    const optimization_args: string[] = [`0`, `1`, `2`, `3`, `s`, `z`];


    for (let i = 0; i < optimization_args.length; i++) {
        //id1937
        describe(`Run ${zkvyperCommand} with -O${optimization_args[i]}`, () => {
            const args = [`${paths.pathToBasicVyContract}`, `-O${optimization_args[i]}`];
            const result = executeCommand(zkvyperCommand, args);

            it("Valid command exit code = 0", () => {
                expect(result.exitCode).toBe(0);
            });

            it("--metadata-hash info is presented", () => {
                expect(result.output).toMatch(/(0x)/i);
            });
        });

        //id1937:I
        describe(`Run only with -O${optimization_args[i]}`, () => {
            const args = [`-O${optimization_args[i]}`];
            const result = executeCommand(zkvyperCommand, args);

            it("Valid command exit code = 1", () => {
                expect(result.exitCode).toBe(1);
            });

            it("Error info is presented", () => {
                expect(result.output).toMatch(/(the following arguments are required)/i);
            });
        });

        //id1978
        describe(`Run  ${zkvyperCommand}  with double -O${optimization_args[i]}`, () => {
            const args = [`${paths.pathToBasicVyContract}`, `-O${optimization_args[i]}`, `-O${optimization_args[i]}`];
            const result = executeCommand(zkvyperCommand, args);

            it("Valid command exit code = 1", () => {
                expect(result.exitCode).toBe(1);
            });

            it("Error info is presented", () => {
                expect(result.output).toMatch(/(provided more than once)/i);
            });
        });
    }
});
