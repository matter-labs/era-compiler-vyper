//!
//! The Vyper contract build.
//!

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::project::contract::vyper::ast::AST;
use crate::project::contract::vyper::expression::Expression as IR;
use crate::vyper::combined_json::contract::warning::Warning as CombinedJsonContractWarning;
use crate::vyper::combined_json::contract::Contract as CombinedJsonContract;
use crate::vyper::selector::Selector as VyperSelector;

///
/// The Vyper contract build.
///
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Contract {
    /// The LLVM module build.
    pub build: era_compiler_llvm_context::EraVMBuild,
    /// The LLL IR parsed from JSON.
    pub ir_json: Option<IR>,
    /// The contract AST.
    pub ast: Option<AST>,
    /// The `vyper` ABI output.
    pub abi: Option<serde_json::Value>,
    /// The `vyper` method identifiers output.
    pub method_identifiers: Option<BTreeMap<String, String>>,
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
        ir_json: Option<IR>,
        ast: Option<AST>,
        abi: Option<serde_json::Value>,
        method_identifiers: Option<BTreeMap<String, String>>,
        layout: Option<serde_json::Value>,
        userdoc: Option<serde_json::Value>,
        devdoc: Option<serde_json::Value>,
        warnings: Vec<CombinedJsonContractWarning>,
    ) -> Self {
        Self {
            build,
            ir_json,
            ast,
            abi,
            method_identifiers,
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
            Some(IR::default()),
            Some(AST::default()),
            Some(serde_json::json!([])),
            Some(BTreeMap::new()),
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
        let mut build = crate::r#const::MINIMAL_PROXY_BUILD.to_owned();
        if output_assembly {
            build.assembly = Some(crate::r#const::MINIMAL_PROXY_CONTRACT_ASSEMBLY.to_owned());
        }
        Self::new_inner(build)
    }

    ///
    /// Writes the contract to the terminal.
    ///
    pub fn write_to_terminal(
        self,
        path: String,
        selection: &[VyperSelector],
    ) -> anyhow::Result<()> {
        for warning in self.warnings.iter() {
            writeln!(std::io::stderr(), "\n{warning}")?;
        }

        writeln!(std::io::stderr(), "Contract `{path}`:")?;
        writeln!(std::io::stdout(), "0x{}", hex::encode(self.build.bytecode))?;

        for flag in selection.iter() {
            match flag {
                VyperSelector::IRJson => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.ir_json.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelector::AST => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.ast.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelector::ABI => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.abi.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelector::MethodIdentifiers => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.method_identifiers.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelector::Layout => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.layout.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelector::UserDocumentation => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.userdoc.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }
                VyperSelector::DeveloperDocumentation => {
                    serde_json::to_writer(
                        std::io::stdout(),
                        self.devdoc.as_ref().expect("Always exists"),
                    )?;
                    writeln!(std::io::stdout())?;
                }

                VyperSelector::EraVMAssembly => {
                    writeln!(std::io::stderr(), "Contract `{path}` assembly:")?;
                    writeln!(
                        std::io::stdout(),
                        "{}",
                        self.build.assembly.as_ref().expect("Always exists")
                    )?;
                }
                VyperSelector::ProjectMetadata => {}

                VyperSelector::CombinedJson => {
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
        selection: &[VyperSelector],
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
        std::fs::write(
            &binary_file_path,
            format!("0x{}", hex::encode(self.build.bytecode.as_slice())).as_bytes(),
        )
        .map_err(|error| anyhow::anyhow!("File {binary_file_path:?} writing error: {error}"))?;

        if selection.is_empty() {
            return Ok(());
        }
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
                VyperSelector::IRJson => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.ir_json.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelector::AST => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.ast.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelector::ABI => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.abi.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelector::MethodIdentifiers => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.method_identifiers.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelector::Layout => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.layout.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelector::UserDocumentation => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.userdoc.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }
                VyperSelector::DeveloperDocumentation => {
                    serde_json::to_writer(
                        &extra_output_file,
                        self.devdoc.as_ref().expect("Always exists"),
                    )?;
                    writeln!(&extra_output_file)?;
                }

                VyperSelector::EraVMAssembly => {
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
                    std::fs::write(
                        &assembly_file_path,
                        self.build
                            .assembly
                            .as_ref()
                            .expect("Always exists")
                            .as_bytes(),
                    )
                    .map_err(|error| {
                        anyhow::anyhow!("File {assembly_file_path:?} writing error: {error}")
                    })?;
                }
                VyperSelector::ProjectMetadata => {}

                VyperSelector::CombinedJson => {
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

            ir_json: self
                .ir_json
                .map(|ir_json| serde_json::to_value(ir_json).expect("Always valid")),
            ast: self
                .ast
                .map(|ast| serde_json::to_value(ast).expect("Always valid")),
            abi: self.abi,
            method_identifiers: self.method_identifiers,
            layout: self.layout,
            userdoc: self.userdoc,
            devdoc: self.devdoc,

            assembly: self.build.assembly,
            factory_deps: Some(self.build.factory_dependencies),
            warnings: Some(self.warnings),
        }
    }
}
