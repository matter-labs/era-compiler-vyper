//!
//! The Vyper contract build.
//!

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::metadata::Metadata as SourceMetadata;
use crate::project::contract::vyper::ast::AST;
use crate::project::contract::vyper::expression::Expression as IR;
use crate::vyper::combined_json::contract::warning::Warning as CombinedJsonContractWarning;
use crate::vyper::combined_json::contract::Contract as CombinedJsonContract;
use crate::vyper::selection::Selection as VyperSelection;

///
/// The Vyper contract build.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The LLVM module build.
    pub build: era_compiler_llvm_context::EraVMBuild,
    /// The LLL IR parsed from JSON.
    pub ir: Option<IR>,
    /// The source metadata.
    pub source_metadata: Option<SourceMetadata>,
    /// The contract AST.
    pub ast: Option<AST>,
    /// The `vyper` method identifiers output.
    pub method_identifiers: Option<BTreeMap<String, String>>,
    /// The `vyper` ABI output.
    pub abi: Option<serde_json::Value>,
    /// The `vyper` layout output.
    pub layout: Option<serde_json::Value>,
    /// The `vyper` userdoc output.
    pub userdoc: Option<serde_json::Value>,
    /// The `vyper` devdoc output.
    pub devdoc: Option<serde_json::Value>,
    /// The compilation warnings.
    pub warnings: Vec<CombinedJsonContractWarning>,
}

impl Contract {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(
        build: era_compiler_llvm_context::EraVMBuild,
        ir: Option<IR>,
        source_metadata: Option<SourceMetadata>,
        ast: Option<AST>,
        method_identifiers: Option<BTreeMap<String, String>>,
        abi: Option<serde_json::Value>,
        layout: Option<serde_json::Value>,
        userdoc: Option<serde_json::Value>,
        devdoc: Option<serde_json::Value>,
        warnings: Vec<CombinedJsonContractWarning>,
    ) -> Self {
        Self {
            build,
            ir,
            source_metadata,
            ast,
            method_identifiers,
            abi,
            layout,
            userdoc,
            devdoc,
            warnings,
        }
    }

    ///
    /// A shortcut constructor.
    ///
    pub fn new_inner(build: era_compiler_llvm_context::EraVMBuild) -> Self {
        Self::new(
            build,
            None,
            None,
            None,
            Some(BTreeMap::new()),
            Some(serde_json::json!([])),
            Some(serde_json::json!({})),
            Some(serde_json::json!({})),
            Some(serde_json::json!({})),
            vec![],
        )
    }

    ///
    /// A shortcut constructor for minimal proxy.
    ///
    pub fn new_minimal_proxy(output_assembly: bool) -> Self {
        let build = era_compiler_llvm_context::EraVMBuild::new(
            crate::r#const::MINIMAL_PROXY_CONTRACT_BYTECODE.clone(),
            Some(*crate::r#const::MINIMAL_PROXY_CONTRACT_HASH),
            None,
            if output_assembly {
                Some(crate::r#const::MINIMAL_PROXY_CONTRACT_ASSEMBLY.to_owned())
            } else {
                None
            },
        );

        Self::new_inner(build)
    }

    ///
    /// Writes the contract to the terminal.
    ///
    pub fn write_to_terminal(
        self,
        path: String,
        selection: &[VyperSelection],
    ) -> anyhow::Result<()> {
        for warning in self.warnings.iter() {
            writeln!(std::io::stderr(), "\n{warning}")?;
        }

        writeln!(std::io::stderr(), "Contract `{path}`:")?;
        writeln!(std::io::stdout(), "0x{}", hex::encode(self.build.bytecode))?;

        for flag in selection.iter() {
            match flag {
                VyperSelection::IRJson => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.ir.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelection::Metadata => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.source_metadata.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelection::AST => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.ast.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelection::ABI => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.abi.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelection::MethodIdentifiers => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.method_identifiers.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelection::StorageLayout => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.layout.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelection::UserDocumentation => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.userdoc.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelection::DeveloperDocumentation => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.devdoc.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }

                VyperSelection::EraVMAssembly => {
                    writeln!(std::io::stderr(), "Contract `{path}` assembly:")?;
                    writeln!(
                        std::io::stdout(),
                        "{}",
                        self.build.assembly.as_ref().expect("Always exists")
                    )?;
                }
                VyperSelection::ProjectMetadata => {}

                VyperSelection::CombinedJson => {
                    panic!("Combined JSON is printed with another pipeline");
                }
            }
        }

        Ok(())
    }

    ///
    /// Writes the contract output to the directory.
    ///
    pub fn write_to_directory(
        self,
        selection: &[VyperSelection],
        output_directory: &Path,
        contract_path: &Path,
        overwrite: bool,
    ) -> anyhow::Result<()> {
        for warning in self.warnings.iter() {
            writeln!(std::io::stderr(), "\n{warning}")?;
        }

        let contract_path = crate::path_to_posix(contract_path)?;
        let file_name = contract_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("File name not found in path {contract_path:?}"))?
            .to_string_lossy();

        let binary_file_name = format!(
            "{}.{}",
            file_name,
            era_compiler_common::EXTENSION_ERAVM_BINARY
        );
        let mut binary_file_path = output_directory.to_owned();
        binary_file_path.push(binary_file_name);
        if binary_file_path.exists() && !overwrite {
            anyhow::bail!(
                "Refusing to overwrite an existing file {binary_file_path:?} (use --overwrite to force).",
            );
        }
        File::create(&binary_file_path)
            .map_err(|error| {
                anyhow::anyhow!("File {:?} creating error: {}", binary_file_path, error)
            })?
            .write_all(format!("0x{}", hex::encode(self.build.bytecode.as_slice())).as_bytes())
            .map_err(|error| {
                anyhow::anyhow!("File {:?} writing error: {}", binary_file_path, error)
            })?;

        let mut extra_output_file_path = output_directory.to_owned();
        extra_output_file_path.push(file_name.as_ref());
        if extra_output_file_path.exists() && !overwrite {
            anyhow::bail!(
                "Refusing to overwrite an existing file {extra_output_file_path:?} (use --overwrite to force).",
            );
        }
        let extra_output_file =
            File::create(extra_output_file_path.as_path()).map_err(|error| {
                anyhow::anyhow!("File {extra_output_file_path:?} creating: {error}")
            })?;
        for flag in selection.iter() {
            match flag {
                VyperSelection::IRJson => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.ir.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelection::Metadata => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.source_metadata.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelection::AST => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.ast.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelection::ABI => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.abi.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelection::MethodIdentifiers => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.method_identifiers.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelection::StorageLayout => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.layout.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelection::UserDocumentation => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.userdoc.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelection::DeveloperDocumentation => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.devdoc.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }

                VyperSelection::EraVMAssembly => {
                    let assembly_file_name = format!(
                        "{}.{}",
                        file_name,
                        era_compiler_common::EXTENSION_ERAVM_ASSEMBLY
                    );
                    let mut assembly_file_path = output_directory.to_owned();
                    assembly_file_path.push(assembly_file_name);
                    if assembly_file_path.exists() && !overwrite {
                        anyhow::bail!(
                            "Refusing to overwrite an existing file {assembly_file_path:?} (use --overwrite to force).",
                        );
                    }
                    File::create(&assembly_file_path)
                        .map_err(|error| {
                            anyhow::anyhow!(
                                "File {:?} creating error: {}",
                                assembly_file_path,
                                error
                            )
                        })?
                        .write_all(
                            self.build
                                .assembly
                                .as_ref()
                                .expect("Always exists")
                                .as_bytes(),
                        )
                        .map_err(|error| {
                            anyhow::anyhow!(
                                "File {:?} writing error: {}",
                                assembly_file_path,
                                error
                            )
                        })?;
                }
                VyperSelection::ProjectMetadata => {}

                VyperSelection::CombinedJson => {
                    panic!("Combined JSON is printed with another pipeline");
                }
            }
        }

        Ok(())
    }

    ///
    /// Converts the contract to the combined JSON.
    ///
    pub fn into_combined_json(self) -> CombinedJsonContract {
        let bytecode = format!("0x{}", hex::encode(self.build.bytecode));
        CombinedJsonContract {
            bytecode: bytecode.clone(),
            bytecode_runtime: bytecode,

            method_identifiers: self.method_identifiers,
            abi: self.abi,
            layout: self.layout,
            userdoc: self.userdoc,
            devdoc: self.devdoc,

            assembly: self.build.assembly,
            warnings: Some(self.warnings),
            factory_deps: Some(self.build.factory_dependencies),
        }
    }
}
