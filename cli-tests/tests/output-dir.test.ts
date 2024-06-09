import {executeCommand, isDestinationExist, isFileEmpty, createTmpDirectory, pathToVyBinOutputFile, pathToVyAsmOutputFile} from "../src/helper";
import { paths } from '../src/entities';
import * as os from 'os';

describe("Output dir", () => {
    const zkvyperCommand = 'zkvyper';
    const vyperCommand = 'vyper';

    //id1983
    describe("Default run with output dir", () => {
        const tmpDirZkVyper = createTmpDirectory();
        const args = [`"${paths.pathToBasicVyContract}"`, `-o`, `"${tmpDirZkVyper.name}"`];
        const result = executeCommand(zkvyperCommand, args);

        it("Exit code = 0", () => {
            expect(result.output).toMatch(/(xxx)/i);
            expect(result.exitCode).toBe(0);
        });

        it("Output dir is created", () => {
            expect(isDestinationExist(tmpDirZkVyper.name)).toBe(true);
        });

        it("Output files are created", () => {
            expect(isDestinationExist(pathToVyBinOutputFile(tmpDirZkVyper.name))).toBe(true);
            expect(isDestinationExist(pathToVyAsmOutputFile(tmpDirZkVyper.name))).toBe(true);
        });

        it("The output files are not empty", () => {
            expect(isFileEmpty(pathToVyBinOutputFile(tmpDirZkVyper.name))).toBe(false);
            expect(isFileEmpty(pathToVyAsmOutputFile(tmpDirZkVyper.name))).toBe(false);
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
            tmpDirZkVyper.removeCallback()
        });

        xit("vyper exit code == zkvyper exit code", () => {
            const tmpDirVyper = createTmpDirectory();
            const args = [`"${paths.pathToBasicVyContract}"`, `-o`, `"${tmpDirVyper.name}"`];
            const vyperResult = executeCommand(vyperCommand, args);
            expect(vyperResult.exitCode).toBe(result.exitCode);
            tmpDirVyper.removeCallback()
        });
    });

    //id1984
    describe("Default run with dual output dir options", () => {
        const tmpDirZkVyper = createTmpDirectory();
        const args = [`"${paths.pathToBasicVyContract}"`, `-o`, `"${tmpDirZkVyper.name}"`, `-o`, `"${tmpDirZkVyper.name}"`];
        const result = executeCommand(zkvyperCommand, args);

        it("Exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });

        it("Error appears", () => {
            expect(result.output).toMatch(/([Ee]rror)/i);
            tmpDirZkVyper.removeCallback();
        });

        it("vyper exit code == zkvyper exit code", () => {
            const tmpDirVyper = createTmpDirectory();
            const args = [`"${paths.pathToBasicVyContract}"`, `-o`, `"${tmpDirVyper.name}"`, `-o`, `"${tmpDirVyper.name}"`];
            const vyperResult = executeCommand(vyperCommand, args);
            expect(vyperResult.exitCode).toBe(result.exitCode);
            tmpDirVyper.removeCallback();
        });
    });
});
