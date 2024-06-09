import * as path from 'path';

const outputDir = 'artifacts';
const outputFile = 'out.bin';
const binExtension = '.zbin';
const asmExtension = '.zasm';
const illExtension = '.lll';
const illOptimizedExtension = '.optimized.ll';
const illUnOptimizedExtension = '.unoptimized.ll';
const contractVyFilename = 'contract.vy';
const contractEcRecover = 'ecrecover.vy';
const contractExtCode = 'extcode.vy';
const contractTxOrigin = 'txorigin.vy';
const contractZkasmFilename = 'contract.zkasm';
const contractJSONFilename = 'contract.json';
const contractLLVMFilename = 'contract.ll';
const pathToOutputDir = path.join( __dirname, '..', outputDir);
const pathToOutputFile = path.join( __dirname, '..', outputFile);
const pathToContracts = path.join( __dirname, '..', 'src', 'contracts');
const pathToBasicVyContract = path.join(pathToContracts, 'vyper', contractVyFilename);
const pathToBasicZkasmContract = path.join(pathToContracts, 'zkasm', contractZkasmFilename);
const pathToEcRecoverContract = path.join(pathToContracts, 'vyper', contractEcRecover);
const pathToExtCodeContract = path.join(pathToContracts, 'vyper', contractExtCode);
const pathToTxOriginContract = path.join(pathToContracts, 'vyper', contractTxOrigin);
const pathToJSONContract = path.join(pathToContracts, 'json', contractJSONFilename);
const pathToLLVMContract = path.join(pathToContracts, 'llvm', contractLLVMFilename);
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
  pathToOutputFile: pathToOutputFile,
  pathToContracts: pathToContracts,
  pathToBasicVyContract: pathToBasicVyContract,
  pathToBasicZkasmContract: pathToBasicZkasmContract,
  pathToTxOriginContract: pathToTxOriginContract,
  pathToExtCodeContract: pathToExtCodeContract,
  pathToEcRecoverContract: pathToEcRecoverContract,
  pathToJSONContract: pathToJSONContract,
  pathToLLVMContract: pathToLLVMContract,
  pathToVyBinOutputFile: pathToVyBinOutputFile,
  pathToVyAsmOutputFile: pathToVyAsmOutputFile,
};
