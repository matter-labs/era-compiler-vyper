import * as path from 'path';

const outputDir = 'artifacts';
const binExtension = '.zbin';
const asmExtension = '.zasm';
const illExtension = '.ill';
const illOptimizedExtension = '.optimized.ill';
const illUnOptimizedExtension = '.unoptimized.ill';
const contractVyFilename = 'contract.vy';
const contractEcRecover = 'ecrecover.vy';
const contractExtCode = 'extcode.vy';
const contractTxOrigin = 'txorigin.vy';
const contractZkasmFilename = 'contract.zkasm';
const contractJSONFilename = 'contract.json';
const pathToOutputDir = path.join( __dirname, '..', outputDir);
const pathToContracts = path.join( __dirname, '..', 'src', 'contracts');
const pathToBasicVyContract = path.join(pathToContracts, 'vyper', contractVyFilename);
const pathToBasicZkasmContract = path.join(pathToContracts, 'zkasm', contractZkasmFilename);
const pathToEcRecoverContract = path.join(pathToContracts, 'vyper', contractEcRecover);
const pathToExtCodeContract = path.join(pathToContracts, 'vyper', contractExtCode);
const pathToTxOriginContract = path.join(pathToContracts, 'vyper', contractTxOrigin);
const pathToJSONContract = path.join(pathToContracts, 'json', contractJSONFilename);
const pathToVyBinOutputFile = path.join(pathToOutputDir, contractVyFilename + binExtension);
const pathToVyAsmOutputFile = path.join(pathToOutputDir, contractVyFilename + asmExtension);

export const paths = {
  outputDir: outputDir,
  binExtension: binExtension,
  asmExtension: asmExtension,
  illExtension: illExtension,
  illOptimizedExtension: illOptimizedExtension,
  illUnOptimizedExtension: illUnOptimizedExtension,
  contractVyFilename: contractVyFilename,
  pathToOutputDir: pathToOutputDir,
  pathToContracts: pathToContracts,
  pathToBasicVyContract: pathToBasicVyContract,
  pathToBasicZkasmContract: pathToBasicZkasmContract,
  pathToTxOriginContract: pathToTxOriginContract,
  pathToExtCodeContract: pathToExtCodeContract,
  pathToEcRecoverContract: pathToEcRecoverContract,
  pathToJSONContract: pathToJSONContract,
  pathToVyBinOutputFile: pathToVyBinOutputFile,
  pathToVyAsmOutputFile: pathToVyAsmOutputFile,
};
