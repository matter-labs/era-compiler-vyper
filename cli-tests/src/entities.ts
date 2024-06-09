import * as path from 'path';

const outputDir = 'artifacts';
const outputFile = 'out.bin';
const binExtension = '.zbin';
const eraVMAssemblyExtension = '.zasm';
const llvmIRExtension = '.ll';
const llvmIROptimizedExtension = '.optimized.ll';
const llvmIRUnoptimizedExtension = '.unoptimized.ll';
const contractVyFilename = 'contract.vy';
const contractEcRecover = 'ecrecover.vy';
const contractExtCode = 'extcode.vy';
const contractTxOrigin = 'txorigin.vy';
const contractEraVMAssemblyFilename = 'contract.zasm';
const contractJSONFilename = 'contract.json';
const contractLLVMFilename = 'contract.ll';
const pathToOutputDir = path.join( __dirname, '..', outputDir);
const pathToOutputFile = path.join( __dirname, '..', outputFile);
const pathToContracts = path.join( __dirname, '..', 'src', 'contracts');
const pathToBasicVyContract = path.join(pathToContracts, 'vyper', contractVyFilename);
const pathToBasicEraVMAssemblyContract = path.join(pathToContracts, 'eravm', contractEraVMAssemblyFilename);
const pathToEcRecoverContract = path.join(pathToContracts, 'vyper', contractEcRecover);
const pathToExtCodeContract = path.join(pathToContracts, 'vyper', contractExtCode);
const pathToTxOriginContract = path.join(pathToContracts, 'vyper', contractTxOrigin);
const pathToJSONContract = path.join(pathToContracts, 'json', contractJSONFilename);
const pathToLLVMContract = path.join(pathToContracts, 'llvm', contractLLVMFilename);
const pathToBinOutputFile = path.join(pathToOutputDir, contractVyFilename + binExtension);
const pathToEraVMAssemblyOutputFile = path.join(pathToOutputDir, contractVyFilename + eraVMAssemblyExtension);

export const paths = {
  outputDir: outputDir,
  binExtension: binExtension,
  eraVMAssemblyExtension: eraVMAssemblyExtension,
  llvmIRExtension: llvmIRExtension,
  llvmIROptimizedExtension: llvmIROptimizedExtension,
  llvmIRUnoptimizedExtension: llvmIRUnoptimizedExtension,
  contractVyFilename: contractVyFilename,
  pathToOutputDir: pathToOutputDir,
  pathToOutputFile: pathToOutputFile,
  pathToContracts: pathToContracts,
  pathToBasicVyContract: pathToBasicVyContract,
  pathToBasicEraVMAssemblyContract: pathToBasicEraVMAssemblyContract,
  pathToTxOriginContract: pathToTxOriginContract,
  pathToExtCodeContract: pathToExtCodeContract,
  pathToEcRecoverContract: pathToEcRecoverContract,
  pathToJSONContract: pathToJSONContract,
  pathToLLVMContract: pathToLLVMContract,
  pathToBinOutputFile: pathToBinOutputFile,
  pathToEraVMAssemblyOutputFile: pathToEraVMAssemblyOutputFile,
};
