import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --metadata-hash tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1941
    describe(`Run ${zkvyperCommand} with --metadata-hash by default`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--metadata-hash=none`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1941:II
    describe(`Run ${zkvyperCommand} with not full --metadata-hash options`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--metadata-hash`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(requires a value)/i);
        });
    });

    //id1941:III
    describe(`Run only with --metadata-hash options`, () => {
        const args = [`--metadata-hash=none`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(arguments are required)/i);
        });
    });

    //id1975
    describe(`Run ${zkvyperCommand} with double --metadata-hash options`, () => {
        const args = [`${paths.pathToBasicVyContract}`, `--metadata-hash=none`, `--metadata-hash`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error is presented", () => {
            expect(result.output).toMatch(/(was provided more than once)/i);
        });
    });

});
