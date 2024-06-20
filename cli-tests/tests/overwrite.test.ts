import {executeCommand, isDestinationExist, isFileEmpty, createTmpDirectory, pathToBinOutputFile, pathToEraVMAssemblyOutputFile, createFiles} from "../src/helper";
import { paths } from '../src/entities';

describe("Overwrite output dir", () => {
    const zkvyperCommand = 'zkvyper';

    //id1983
    describe("Default run with --overwrite output dir", () => {
        const tmpDirZkVyper = createTmpDirectory();

        it("Output dir is created", () => {
            expect(isDestinationExist(tmpDirZkVyper.name)).toBe(true);
        });

        // adding empty files to tmp dir
        createFiles(tmpDirZkVyper.name, [`${paths.contractVyFilename}${paths.binExtension}`, `${paths.contractVyFilename}${paths.eraVMAssemblyExtension}`])

        // trying to run a command to get a warning and verify an exit code
        const pre_args = [`"${paths.pathToBasicVyContract}"`, `-o`, `"${tmpDirZkVyper.name}"`];
        const pre_result = executeCommand(zkvyperCommand, pre_args);

        it("Refusing to overwrite in the output", () => {
            expect(pre_result.output).toMatch(/(Refusing to overwrite)/i);
        });

        // trying to add a flag and verify that command passed with 0 exit code
        const args = [`"${paths.pathToBasicVyContract}"`, `-o`, `"${tmpDirZkVyper.name}"`, `--overwrite`, `--output-assembly`];
        const result = executeCommand(zkvyperCommand, args);

        it("Output is empty", () => {
            expect(result.output).not.toMatch(/(Refusing to overwrite)/i);
        });

        it("Exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        // verify that files are not empty
        it("The output files are not empty", () => {
            expect(isFileEmpty(pathToBinOutputFile(tmpDirZkVyper.name))).toBe(false);
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
            tmpDirZkVyper.removeCallback()
        });
    });
});
