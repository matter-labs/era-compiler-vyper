import {executeCommand} from "../src/helper";
import { paths } from '../src/entities';


describe("Set of --suppress-warnings tests", () => {
    const zkvyperCommand = 'zkvyper';

    //id1954
    describe(`Run ${zkvyperCommand} with --suppress-warnings ecrecover`, () => {
        const args = [`${paths.pathToEcRecoverContract}`, `--suppress-warnings`, `ecrecover`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
        });

        it("ecrecover output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1954:I
    describe(`Run ecrecover ${zkvyperCommand} with --suppress-warnings`, () => {
        const args = [`${paths.pathToEcRecoverContract}`, `--suppress-warnings`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Warning in the output", () => {
            expect(result.output).toMatch(/(Warning:)/i);
        });

        it("ecrecover output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1960
    describe(`Run ${zkvyperCommand} with --suppress-warnings extcodesize`, () => {
        const args = [`${paths.pathToExtCodeContract}`, `--suppress-warnings`, `extcodesize`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
        });

        it("extcodesize output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1960:I
    describe(`Run extcodesize ${zkvyperCommand} with --suppress-warnings`, () => {
        const args = [`${paths.pathToExtCodeContract}`, `--suppress-warnings`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Warning in the output", () => {
            expect(result.output).toMatch(/(Warning:)/i);
        });

        it("extcodesize output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1959
    describe(`Run ${zkvyperCommand} with --suppress-warnings txorigin`, () => {
        const args = [`${paths.pathToTxOriginContract}`, `--suppress-warnings`, `txorigin`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
        });

        it("txorigin output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });

    //id1959:I
    describe(`Run txorigin ${zkvyperCommand} with --suppress-warnings`, () => {
        const args = [`${paths.pathToTxOriginContract}`, `--suppress-warnings`];
        const result = executeCommand(zkvyperCommand, args);

        it("Valid command exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Warning in the output", () => {
            expect(result.output).toMatch(/(Warning:)/i);
        });

        it("txorigin output is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });
    });
});
