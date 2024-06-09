import {executeCommand, isDestinationExist, createTmpDirectory, directoryContainsSubstring} from "../src/helper";
import { paths } from '../src/entities';
import * as os from 'os';
import * as fs from 'fs';

describe("Output dir", () => {
    const zkvyperCommand = 'zkvyper';

    //id1976:I
    describe("Run a contract with --debug-output-dir", () => {
        const tmpDirZkVyper = createTmpDirectory();
        const args = [`${paths.pathToBasicVyContract}`, `--debug-output-dir`, `${tmpDirZkVyper.name}`];
        const result = executeCommand(zkvyperCommand, args);

        it("Exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Output dir is created", () => {
            expect(isDestinationExist(tmpDirZkVyper.name)).toBe(true);
        });

        it("It should contain filenames with specified substrings", () => {
            const expectedSubstrings = [
                paths.illExtension,
                paths.illOptimizedExtension,
                paths.illUnOptimizedExtension,
                paths.asmExtension
            ];
            const filenames = fs.readdirSync(tmpDirZkVyper.name);

            const allSubstringsFound = directoryContainsSubstring(filenames, expectedSubstrings);
            expect(allSubstringsFound).toBeTruthy();
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
            tmpDirZkVyper.removeCallback()
        });

    });

    //id1976:II
    describe("Run without a contract with --debug-output-dir", () => {
        const tmpDirZkVyper = createTmpDirectory();
        const args = [`--debug-output-dir`, `${tmpDirZkVyper.name}`];
        const result = executeCommand(zkvyperCommand, args);

        it("Exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error in the output", () => {
            expect(result.output).toMatch(/(the following arguments are required)/i);
            tmpDirZkVyper.removeCallback()
        });
    });

    //id1976:III
    describe("Run with --debug-output-dir no folder arg", () => {
        const tmpDirZkVyper = createTmpDirectory();
        const args = [`${paths.pathToBasicVyContract}`, `--debug-output-dir`];
        const result = executeCommand(zkvyperCommand, args);

        it("Exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error in the output", () => {
            expect(result.output).toMatch(/(requires a value)/i);
            tmpDirZkVyper.removeCallback()
        });
    });

    //id1977
    describe("Run with double --debug-output-dir option", () => {
        const tmpDirZkVyper = createTmpDirectory();
        const args = [`${paths.pathToBasicVyContract}`, `--debug-output-dir`, `${tmpDirZkVyper.name}`, `--debug-output-dir`, `${tmpDirZkVyper.name}`];
        const result = executeCommand(zkvyperCommand, args);

        it("Exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error in the output", () => {
            expect(result.output).toMatch(/(cannot be used multiple times)/i);
            tmpDirZkVyper.removeCallback()
        });
    });
});
