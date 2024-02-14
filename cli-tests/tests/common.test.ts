import {executeCommand, isDestinationExist, isFileEmpty} from "../src/helper";
import { paths } from '../src/entities';


describe("Common tests", () => {
    const zkvyperCommand = 'zkvyper';
    const vyperCommand = 'vyper';

    //id1936
    describe("Run zkvyper without any options", () => {
        const args = [''];
        const result = executeCommand(zkvyperCommand, args);

        it("Info with help is presented", () => {
            expect(result.output).toMatch(/(the following arguments are required: input_files|Disable the system request memoization)/i);
        });

        it("Exit code = 1", () => {
            expect(result.exitCode).toBe(1);
        });
        xit("vyper exit code == zkvyper exit code", () => {
            const vyperResult = executeCommand(vyperCommand, args);
            expect(vyperResult.exitCode).toBe(result.exitCode); // 2 for vyper and 1 for zkvyper
        });
    });

    //id1978
    describe("Default run with a contract only", () => {
        const args = [`"${paths.pathToBasicVyContract}"`]; // issue on windows
        const result = executeCommand(zkvyperCommand, args);

        it("Exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Info with a tx hash is presented", () => {
            expect(result.output).toMatch(/(0x)/i);
        });

        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
        });

        it("vyper exit code == zkvyper exit code", () => {
            const vyperResult = executeCommand(vyperCommand, args);
            expect(vyperResult.exitCode).toBe(result.exitCode);
        });
    });

    //id1983
    describe("Default run a command from the help", () => {
        const args = [`"${paths.pathToBasicVyContract}"`, `-o`, `"${paths.pathToOutputDir}"`]; // issue on windows
        const result = executeCommand(zkvyperCommand, args); 

        it("Exit code = 0", () => {
            expect(result.exitCode).toBe(0);
        });

        it("Output dir is created", () => {
            expect(isDestinationExist(paths.pathToOutputDir)).toBe(true);
        });

        it("Output file is created", () => { // a bug on windows
            expect(isDestinationExist(paths.pathToVyBinOutputFile)).toBe(true);
            expect(isDestinationExist(paths.pathToVyAsmOutputFile)).toBe(true);
        });
        it("The output file is not empty", () => {
            expect(isFileEmpty(paths.pathToVyBinOutputFile)).toBe(false);
        });
        
        it("No 'Error'/'Warning'/'Fail' in the output", () => {
            expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
        });

        it("vyper exit code == zkvyper exit code", () => {
            const vyperResult = executeCommand(vyperCommand, args);
            expect(vyperResult.exitCode).toBe(result.exitCode);
        });
    });
});
