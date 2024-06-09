import * as fs from 'fs';
import { spawnSync } from "child_process";
import * as tmp from 'tmp';
import { paths } from './entities';
import * as path from 'path';

tmp.setGracefulCleanup();

export function executeCommand(command: string, args: string[]) {
  const result = spawnSync(command, args, { encoding: 'utf-8', shell: true, stdio: 'pipe' });
  return {
      exitCode: result.status,
      output: result.stdout.trim() + result.stderr.trim()
  };
}

export const directoryContainsSubstring = (filenames: string[], substrings: string[]): boolean => {
    let allSubstringsFound = true;
    const missedSubstrings: string[] = [];

    // check if each expected substring is present in at least one filename
    for (let substring of substrings) {
        let substringFound = false;
        for (let file_name of filenames) {
            if (file_name.includes(substring)) {
                substringFound = true;
                break;
            }
        }
        if (!substringFound) {
            allSubstringsFound = false;
            missedSubstrings.push(substring);
        }
    }
    if (!allSubstringsFound) {
        for (let file_name of filenames) {
            console.log("filename: ", file_name, `\n`);
        }
        console.log("Missed substrings:", missedSubstrings.join(', '));
    }
    return allSubstringsFound;
}

export const isDestinationExist = (destination: string): boolean  => {
    return fs.existsSync(destination);
};

export const isFileEmpty = (file: string): boolean  => {
    if (isDestinationExist(file)) {
        return (fs.readFileSync(file).length === 0);
    } 
};

export const createFiles = (absolutePath: string, files: string[]) => {

        for (let file_name of files) {
            if (file_name != '') {
                const full_path = path.join(absolutePath, file_name);
                fs.writeFileSync(full_path, '');
            }
        }
}

export const createTmpDirectory = (name = 'tmp-XXXXXX'): tmp.DirResult => {
    if (!fs.existsSync(paths.pathToOutputDir)) {
        fs.mkdirSync(paths.pathToOutputDir, { recursive: true });
    }
    return tmp.dirSync({ template: name, tmpdir: paths.pathToOutputDir, unsafeCleanup: true });
};

export const pathToBinOutputFile = (destination: string): string  => {
    return path.join(destination, paths.contractVyFilename + paths.binExtension);
};

export const pathToEraVMAssemblyOutputFile = (destination: string): string  => {
    return path.join(destination, paths.contractVyFilename + paths.eraVMAssemblyExtension);
};

export const pathToVyIllOutputFile = (destination: string): string  => {
    return path.join(destination, paths.contractVyFilename + paths.llvmIRExtension);
};
export const pathToVyOptimIllOutputFile = (destination: string): string  => {
    return path.join(destination, paths.contractVyFilename + paths.llvmIROptimizedExtension);
};
export const pathToVyUnOptimIllOutputFile = (destination: string): string  => {
    return path.join(destination, paths.contractVyFilename + paths.llvmIRUnoptimizedExtension);
};