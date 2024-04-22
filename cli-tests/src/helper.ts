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
      output: result.stdout.trim() || result.stderr.trim()
  };
}

export const isDestinationExist = (destination: string): boolean  => {
    return fs.existsSync(destination);
};

export const isFileEmpty = (file: string): boolean  => {
    if (isDestinationExist(file)) {
        return (fs.readFileSync(file).length === 0);
    } 
};

export const createTmpDirectory = (name = 'tmp-XXXXXX'): tmp.DirResult => {
    if (!fs.existsSync(paths.pathToOutputDir)) {
        fs.mkdirSync(paths.pathToOutputDir, { recursive: true });
    }
    return tmp.dirSync({ template: name, tmpdir: paths.pathToOutputDir, unsafeCleanup: true });
};

export const pathToVyBinOutputFile = (destination: string): string  => {
    return path.join(destination, paths.contractVyFilename + paths.binExtension);
};

export const pathToVyAsmOutputFile = (destination: string): string  => {
    return path.join(destination, paths.contractVyFilename + paths.asmExtension);
};
