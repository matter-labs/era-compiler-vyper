import * as fs from 'fs';
import { spawnSync } from "child_process";


export function executeCommand(command: string, args: string[]) {
  const result = spawnSync(command, args, { encoding: 'utf-8', shell: true, stdio: 'pipe' });
  
  if (result.stderr) {
    return {
      exitCode: result.status || 1, error: result.error,
      output: result.stderr.trim()
    };
  } else {
    return {
      exitCode: result.status || 0,
      output: result.stdout.trim() || result.stderr.trim()
    };
  }
}

export const isDestinationExist = (destination: string): boolean  => {
    return fs.existsSync(destination);
};

export const isFileEmpty = (file: string): boolean  => {
    if (isDestinationExist(file)) {
        return (fs.readFileSync(file).length === 0);
    } 
};
