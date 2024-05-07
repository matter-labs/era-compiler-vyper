import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';
import * as os from 'os';

describe("Set of --solc tests", () => {
    const zkvyperCommand = 'zkvyper';

    let pathToCustomVyper = executeCommand('which', ['vyper']).output;
    if ( os.platform() === 'win32' ) {
        pathToCustomVyper = executeCommand('where', ['vyper']).output;
    }


    //id1942
    describe(`Run ${zkvyperCommand} with --vyper }`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--vyper`, `${pathToCustomVyper}`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("--metadata-hash info is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1942:II
    describe(`Run ${zkvyperCommand} with --vyper empty arg }`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--vyper`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("--metadata-hash info is presented", () => {
            expect(result.output).toMatch(/(requires a value but none was supplied)/i);
        });
    });

    //id1942:III
    describe(`Run ${zkvyperCommand} with --vyper wrong arg }`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--vyper`, `..`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code != 0", () => {
            expect(result.exitCode).not.toBe(0);
        });

        it("--metadata-hash info is presented", () => {
            expect(result.output).toMatch(/(error|not found)/i);
        });
    });
});
