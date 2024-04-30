import {executeCommand, isDestinationExist, isFileEmpty, createTmpDirectory, pathToVyBinOutputFile, pathToVyAsmOutputFile, pathToVyIllOutputFile, pathToVyOptimIllOutputFile, pathToVyUnOptimIllOutputFile} from "../src/helper";
import { paths } from '../src/entities';
import * as os from 'os';

if (os.platform() !== 'win32') { //bugs on windows
    describe("Output dir", () => {
        const zkvyperCommand = 'zkvyper';

        //id1976:I
        describe("Run a contract with --debug-output-dir", () => {
            const tmpDirZkVyper = createTmpDirectory();
            const args = [`${paths.pathToBasicVyContract}`, `--debug-output-dir`, `${tmpDirZkVyper.name}`]; // issue on windows
            const result = executeCommand(zkvyperCommand, args);

            it("Exit code = 0", () => {
                expect(result.exitCode).toBe(0);
            });

            it("Output dir is created", () => {
                expect(isDestinationExist(tmpDirZkVyper.name)).toBe(true);
            });

            //TODO: bug with creation filenames QA-853
            xit("Output files are created", () => {
                // Remove if () {} after the bugfix on win
                console.log("paths.pathToBasicVyContract: " + paths.pathToBasicVyContract);
                console.log("pathToVyAsmOutputFile(tmpDirZkVyper.name): " + pathToVyAsmOutputFile(tmpDirZkVyper.name));

                if (os.platform() === 'win32') {
                    console.log("Expected file: " + pathToVyBinOutputFile(tmpDirZkVyper.name))
                    console.log("Actual file: " + executeCommand('dir', [tmpDirZkVyper.name, '/B']).output)
                } else {
                    expect(isDestinationExist(pathToVyAsmOutputFile(tmpDirZkVyper.name))).toBe(true);
                    expect(isDestinationExist(pathToVyIllOutputFile(tmpDirZkVyper.name))).toBe(true);
                    expect(isDestinationExist(pathToVyOptimIllOutputFile(tmpDirZkVyper.name))).toBe(true);
                    expect(isDestinationExist(pathToVyUnOptimIllOutputFile(tmpDirZkVyper.name))).toBe(true);
                }
            });

            it("No 'Error'/'Warning'/'Fail' in the output", () => {
                expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
                tmpDirZkVyper.removeCallback()
            });

        });

        //id1976:II
        describe("Run without a contract with --debug-output-dir", () => {
            const tmpDirZkVyper = createTmpDirectory();
            const args = [`--debug-output-dir`, `${tmpDirZkVyper.name}`]; // issue on windows
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
            const args = [`${paths.pathToBasicVyContract}`, `--debug-output-dir`]; // issue on windows
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
}