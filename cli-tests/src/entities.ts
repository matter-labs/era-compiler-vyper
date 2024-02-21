import * as path from 'path';

const outputDir = 'artifacts';
const binExtension = '.zbin';
const asmExtension = '.zasm';
const contractVyFilename = 'contract.vy';
const pathToOutputDir = path.join( __dirname, '..', outputDir);
const pathToContracts = path.join( __dirname, '..', 'src', 'contracts');
const pathToBasicVyContract = path.join(pathToContracts, 'vyper', contractVyFilename);
const pathToVyBinOutputFile = path.join(pathToOutputDir, contractVyFilename + binExtension);
const pathToVyAsmOutputFile = path.join(pathToOutputDir, contractVyFilename + asmExtension);

export const paths = {
  outputDir: outputDir,
  binExtension: binExtension,
  asmExtension: asmExtension,
  contractVyFilename: contractVyFilename,
  pathToOutputDir: pathToOutputDir,
  pathToContracts: pathToContracts,
  pathToBasicVyContract: pathToBasicVyContract,
  pathToVyBinOutputFile: pathToVyBinOutputFile,
  pathToVyAsmOutputFile: pathToVyAsmOutputFile,
};
