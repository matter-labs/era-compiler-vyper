import {executeCommand, isFolderExist, isFileExist, isFileEmpty} from "../src/helper";
import { paths } from '../src/entities';


//
describe("Run zkvyper without any options", () => {
    const command = 'zkvyper';
    const result = executeCommand(command);

    it("Info with help is presented", () => {
        expect(result.output).toMatch(/(the following arguments are required: input_files)/i);
    });

    it("Exit code = 1", () => {
        expect(result.exitCode).toBe(1);
    });

    xit("vyper exit code == zkvyper exit code", () => {
        const command = 'vyper';
        const vyperResult = executeCommand(command);
        expect(vyperResult.exitCode).toBe(result.exitCode); // 2 for vyper and 1 for zkvyper
    });
});


//
describe("Default run a command from the help", () => {

    const command = `zkvyper "${paths.pathToBasicVyContract}" -o "${paths.pathToOutputDir}"`; // issue on windows
    const result = executeCommand(command); 

    it("Exit code = 0", () => {
        expect(result.exitCode).toBe(0);
    });
    it("Output dir is created", () => {
        expect(isFolderExist(paths.pathToOutputDir)).toBe(true);
    });

    if (process.platform !== "win32") {
        it("Output file is created", () => { // a bug on windows
            expect(isFileExist(paths.pathToOutputDir, paths.contractVyFilename, paths.binExtension)).toBe(true);
            expect(isFileExist(paths.pathToOutputDir, paths.contractVyFilename, paths.asmExtension)).toBe(true);
        });
        it("The output file is not empty", () => {
            expect(isFileEmpty(paths.pathToVyBinOutputFile)).toBe(false);
        });
    }
    
    it("No 'Error'/'Warning'/'Fail' in the output", () => {
        expect(result.output).not.toMatch(/([Ee]rror|[Ww]arning|[Ff]ail)/i);
    });
});
