#![allow(unused)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tristate {
    Unknown,
    False,
    True,
}

impl Tristate {
    pub fn is_true(&self) -> bool {
        matches!(self, Tristate::True)
    }

    pub fn is_false(&self) -> bool {
        matches!(self, Tristate::False)
    }
}

impl Default for Tristate {
    fn default() -> Self {
        Tristate::Unknown
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerOptions {
    #[serde(rename = "allowJs")]
    pub allow_js: Tristate,
    #[serde(rename = "allowArbitraryExtensions")]
    pub allow_arbitrary_extensions: Tristate,
    #[serde(rename = "allowSyntheticDefaultImports")]
    pub allow_synthetic_default_imports: Tristate,
    #[serde(rename = "allowImportingTsExtensions")]
    pub allow_importing_ts_extensions: Tristate,
    #[serde(rename = "allowNonTsExtensions")]
    pub allow_non_ts_extensions: Tristate,
    #[serde(rename = "allowUmdGlobalAccess")]
    pub allow_umd_global_access: Tristate,
    #[serde(rename = "allowUnreachableCode")]
    pub allow_unreachable_code: Tristate,
    #[serde(rename = "allowUnusedLabels")]
    pub allow_unused_labels: Tristate,
    #[serde(rename = "assumeChangesOnlyAffectDirectDependencies")]
    pub assume_changes_only_affect_direct_dependencies: Tristate,
    #[serde(rename = "alwaysStrict")]
    pub always_strict: Tristate,
    #[serde(rename = "baseUrl")]
    pub base_url: Option<String>,
    pub build: Tristate,
    #[serde(rename = "checkJs")]
    pub check_js: Tristate,
    #[serde(rename = "customConditions")]
    pub custom_conditions: Option<Vec<String>>,
    pub composite: Tristate,
    #[serde(rename = "emitDeclarationOnly")]
    pub emit_declaration_only: Tristate,
    #[serde(rename = "emitBOM")]
    pub emit_bom: Tristate,
    #[serde(rename = "emitDecoratorMetadata")]
    pub emit_decorator_metadata: Tristate,
    #[serde(rename = "downlevelIteration")]
    pub downlevel_iteration: Tristate,
    pub declaration: Tristate,
    #[serde(rename = "declarationDir")]
    pub declaration_dir: Option<String>,
    #[serde(rename = "declarationMap")]
    pub declaration_map: Tristate,
    #[serde(rename = "disableSizeLimit")]
    pub disable_size_limit: Tristate,
    #[serde(rename = "disableSourceOfProjectReferenceRedirect")]
    pub disable_source_of_project_reference_redirect: Tristate,
    #[serde(rename = "disableSolutionSearching")]
    pub disable_solution_searching: Tristate,
    #[serde(rename = "disableReferencedProjectLoad")]
    pub disable_referenced_project_load: Tristate,
    #[serde(rename = "esModuleInterop")]
    pub es_module_interop: Tristate,
    #[serde(rename = "exactOptionalPropertyTypes")]
    pub exact_optional_property_types: Tristate,
    #[serde(rename = "experimentalDecorators")]
    pub experimental_decorators: Tristate,
    #[serde(rename = "forceConsistentCasingInFileNames")]
    pub force_consistent_casing_in_file_names: Tristate,
    #[serde(rename = "isolatedModules")]
    pub isolated_modules: Tristate,
    #[serde(rename = "isolatedDeclarations")]
    pub isolated_declarations: Tristate,
    #[serde(rename = "ignoreDeprecations")]
    pub ignore_deprecations: Option<String>,
    #[serde(rename = "importHelpers")]
    pub import_helpers: Tristate,
    #[serde(rename = "inlineSourceMap")]
    pub inline_source_map: Tristate,
    #[serde(rename = "inlineSources")]
    pub inline_sources: Tristate,
    pub init: Tristate,
    pub incremental: Tristate,
    pub jsx: JsxEmit,
    #[serde(rename = "jsxFactory")]
    pub jsx_factory: Option<String>,
    #[serde(rename = "jsxFragmentFactory")]
    pub jsx_fragment_factory: Option<String>,
    #[serde(rename = "jsxImportSource")]
    pub jsx_import_source: Option<String>,
    #[serde(rename = "keyofStringsOnly")]
    pub keyof_strings_only: Tristate,
    pub lib: Option<Vec<String>>,
    pub locale: Option<String>,
    #[serde(rename = "mapRoot")]
    pub map_root: Option<String>,
    #[serde(rename = "module")]
    pub module_kind: ModuleKind,
    #[serde(rename = "moduleResolution")]
    pub module_resolution: ModuleResolutionKind,
    #[serde(rename = "moduleSuffixes")]
    pub module_suffixes: Option<Vec<String>>,
    #[serde(rename = "moduleDetectionKind")]
    pub module_detection: ModuleDetectionKind,
    #[serde(rename = "newLine")]
    pub new_line: NewLineKind,
    #[serde(rename = "noEmit")]
    pub no_emit: Tristate,
    #[serde(rename = "noCheck")]
    pub no_check: Tristate,
    #[serde(rename = "noErrorTruncation")]
    pub no_error_truncation: Tristate,
    #[serde(rename = "noFallthroughCasesInSwitch")]
    pub no_fallthrough_cases_in_switch: Tristate,
    #[serde(rename = "noImplicitAny")]
    pub no_implicit_any: Tristate,
    #[serde(rename = "noImplicitThis")]
    pub no_implicit_this: Tristate,
    #[serde(rename = "noImplicitReturns")]
    pub no_implicit_returns: Tristate,
    #[serde(rename = "noEmitHelpers")]
    pub no_emit_helpers: Tristate,
    #[serde(rename = "noLib")]
    pub no_lib: Tristate,
    #[serde(rename = "noPropertyAccessFromIndexSignature")]
    pub no_property_access_from_index_signature: Tristate,
    #[serde(rename = "noUncheckedIndexedAccess")]
    pub no_unchecked_indexed_access: Tristate,
    #[serde(rename = "noEmitOnError")]
    pub no_emit_on_error: Tristate,
    #[serde(rename = "noUnusedLocals")]
    pub no_unused_locals: Tristate,
    #[serde(rename = "noUnusedParameters")]
    pub no_unused_parameters: Tristate,
    #[serde(rename = "noResolve")]
    pub no_resolve: Tristate,
    #[serde(rename = "noImplicitOverride")]
    pub no_implicit_override: Tristate,
    #[serde(rename = "noUncheckedSideEffectImports")]
    pub no_unchecked_side_effect_imports: Tristate,
    pub out: Option<String>,
    #[serde(rename = "outDir")]
    pub out_dir: Option<String>,
    #[serde(rename = "outFile")]
    pub out_file: Option<String>,
    pub paths: Option<BTreeMap<String, Vec<String>>>,
    #[serde(rename = "preserveConstEnums")]
    pub preserve_const_enums: Tristate,
    #[serde(rename = "preserveSymlinks")]
    pub preserve_symlinks: Tristate,
    pub project: Option<String>,
    #[serde(rename = "resolveJsonModule")]
    pub resolve_json_module: Tristate,
    #[serde(rename = "resolvePackageJsonExports")]
    pub resolve_package_json_exports: Tristate,
    #[serde(rename = "resolvePackageJsonImports")]
    pub resolve_package_json_imports: Tristate,
    #[serde(rename = "removeComments")]
    pub remove_comments: Tristate,
    #[serde(rename = "rewriteRelativeImportExtensions")]
    pub rewrite_relative_import_extensions: Tristate,
    #[serde(rename = "reactNamespace")]
    pub react_namespace: Option<String>,
    #[serde(rename = "rootDir")]
    pub root_dir: Option<String>,
    #[serde(rename = "rootDirs")]
    pub root_dirs: Option<Vec<String>>,
    #[serde(rename = "skipLibCheck")]
    pub skip_lib_check: Tristate,
    pub strict: Tristate,
    #[serde(rename = "strictBindCallApply")]
    pub strict_bind_call_apply: Tristate,
    #[serde(rename = "strictBuiltinIteratorReturn")]
    pub strict_builtin_iterator_return: Tristate,
    #[serde(rename = "strictFunctionTypes")]
    pub strict_function_types: Tristate,
    #[serde(rename = "strictNullChecks")]
    pub strict_null_checks: Tristate,
    #[serde(rename = "strictPropertyInitialization")]
    pub strict_property_initialization: Tristate,
    #[serde(rename = "stripInternal")]
    pub strip_internal: Tristate,
    #[serde(rename = "skipDefaultLibCheck")]
    pub skip_default_lib_check: Tristate,
    #[serde(rename = "sourceMap")]
    pub source_map: Tristate,
    #[serde(rename = "sourceRoot")]
    pub source_root: Option<String>,
    #[serde(rename = "suppressOutputPathCheck")]
    pub suppress_output_path_check: Tristate,
    pub target: ScriptTarget,
    #[serde(rename = "traceResolution")]
    pub trace_resolution: Tristate,
    #[serde(rename = "tsBuildInfoFile")]
    pub ts_build_info_file: Option<String>,
    #[serde(rename = "typeRoots")]
    pub type_roots: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
    #[serde(rename = "useDefineForClassFields")]
    pub use_define_for_class_fields: Tristate,
    #[serde(rename = "useUnknownInCatchVariables")]
    pub use_unknown_in_catch_variables: Tristate,
    #[serde(rename = "verbatimModuleSyntax")]
    pub verbatim_module_syntax: Tristate,
    #[serde(rename = "maxNodeModuleJsDepth")]
    pub max_node_module_js_depth: Option<i32>,

    // Internal fields
    #[serde(rename = "configFilePath")]
    pub config_file_path: Option<String>,
    #[serde(rename = "noDtsResolution")]
    pub no_dts_resolution: Tristate,
    #[serde(rename = "pathsBasePath")]
    pub paths_base_path: Option<String>,
    pub diagnostics: Tristate,
    #[serde(rename = "extendedDiagnostics")]
    pub extended_diagnostics: Tristate,
    #[serde(rename = "generateCpuProfile")]
    pub generate_cpu_profile: Option<String>,
    #[serde(rename = "generateTrace")]
    pub generate_trace: Option<String>,
    #[serde(rename = "listEmittedFiles")]
    pub list_emitted_files: Tristate,
    #[serde(rename = "listFiles")]
    pub list_files: Tristate,
    #[serde(rename = "explainFiles")]
    pub explain_files: Tristate,
    #[serde(rename = "listFilesOnly")]
    pub list_files_only: Tristate,
    #[serde(rename = "noEmitForJsFiles")]
    pub no_emit_for_js_files: Tristate,
    #[serde(rename = "preserveWatchOutput")]
    pub preserve_watch_output: Tristate,
    pub pretty: Tristate,
    pub version: Tristate,
    pub watch: Tristate,
    #[serde(rename = "showConfig")]
    pub show_config: Tristate,
    #[serde(rename = "tscBuild")]
    pub tsc_build: Tristate,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        CompilerOptions {
            allow_js: Tristate::Unknown,
            allow_arbitrary_extensions: Tristate::Unknown,
            allow_synthetic_default_imports: Tristate::Unknown,
            allow_importing_ts_extensions: Tristate::Unknown,
            allow_non_ts_extensions: Tristate::Unknown,
            allow_umd_global_access: Tristate::Unknown,
            allow_unreachable_code: Tristate::Unknown,
            allow_unused_labels: Tristate::Unknown,
            assume_changes_only_affect_direct_dependencies: Tristate::Unknown,
            always_strict: Tristate::Unknown,
            base_url: None,
            build: Tristate::Unknown,
            check_js: Tristate::Unknown,
            custom_conditions: None,
            composite: Tristate::Unknown,
            emit_declaration_only: Tristate::Unknown,
            emit_bom: Tristate::Unknown,
            emit_decorator_metadata: Tristate::Unknown,
            downlevel_iteration: Tristate::Unknown,
            declaration: Tristate::Unknown,
            declaration_dir: None,
            declaration_map: Tristate::Unknown,
            disable_size_limit: Tristate::Unknown,
            disable_source_of_project_reference_redirect: Tristate::Unknown,
            disable_solution_searching: Tristate::Unknown,
            disable_referenced_project_load: Tristate::Unknown,
            es_module_interop: Tristate::Unknown,
            exact_optional_property_types: Tristate::Unknown,
            experimental_decorators: Tristate::Unknown,
            force_consistent_casing_in_file_names: Tristate::Unknown,
            isolated_modules: Tristate::Unknown,
            isolated_declarations: Tristate::Unknown,
            ignore_deprecations: None,
            import_helpers: Tristate::Unknown,
            inline_source_map: Tristate::Unknown,
            inline_sources: Tristate::Unknown,
            init: Tristate::Unknown,
            incremental: Tristate::Unknown,
            jsx: JsxEmit::None,
            jsx_factory: None,
            jsx_fragment_factory: None,
            jsx_import_source: None,
            keyof_strings_only: Tristate::Unknown,
            lib: None,
            locale: None,
            map_root: None,
            module_kind: ModuleKind::None,
            module_resolution: ModuleResolutionKind::Unknown,
            module_suffixes: None,
            module_detection: ModuleDetectionKind::None,
            new_line: NewLineKind::None,
            no_emit: Tristate::Unknown,
            no_check: Tristate::Unknown,
            no_error_truncation: Tristate::Unknown,
            no_fallthrough_cases_in_switch: Tristate::Unknown,
            no_implicit_any: Tristate::Unknown,
            no_implicit_this: Tristate::Unknown,
            no_implicit_returns: Tristate::Unknown,
            no_emit_helpers: Tristate::Unknown,
            no_lib: Tristate::Unknown,
            no_property_access_from_index_signature: Tristate::Unknown,
            no_unchecked_indexed_access: Tristate::Unknown,
            no_emit_on_error: Tristate::Unknown,
            no_unused_locals: Tristate::Unknown,
            no_unused_parameters: Tristate::Unknown,
            no_resolve: Tristate::Unknown,
            no_implicit_override: Tristate::Unknown,
            no_unchecked_side_effect_imports: Tristate::Unknown,
            out: None,
            out_dir: None,
            out_file: None,
            paths: None,
            preserve_const_enums: Tristate::Unknown,
            preserve_symlinks: Tristate::Unknown,
            project: None,
            resolve_json_module: Tristate::Unknown,
            resolve_package_json_exports: Tristate::Unknown,
            resolve_package_json_imports: Tristate::Unknown,
            remove_comments: Tristate::Unknown,
            rewrite_relative_import_extensions: Tristate::Unknown,
            react_namespace: None,
            root_dir: None,
            root_dirs: None,
            skip_lib_check: Tristate::Unknown,
            strict: Tristate::Unknown,
            strict_bind_call_apply: Tristate::Unknown,
            strict_builtin_iterator_return: Tristate::Unknown,
            strict_function_types: Tristate::Unknown,
            strict_null_checks: Tristate::Unknown,
            strict_property_initialization: Tristate::Unknown,
            strip_internal: Tristate::Unknown,
            skip_default_lib_check: Tristate::Unknown,
            source_map: Tristate::Unknown,
            source_root: None,
            suppress_output_path_check: Tristate::Unknown,
            target: ScriptTarget::None,
            trace_resolution: Tristate::Unknown,
            ts_build_info_file: None,
            type_roots: None,
            types: None,
            use_define_for_class_fields: Tristate::Unknown,
            use_unknown_in_catch_variables: Tristate::Unknown,
            verbatim_module_syntax: Tristate::Unknown,
            max_node_module_js_depth: None,
            config_file_path: None,
            no_dts_resolution: Tristate::Unknown,
            paths_base_path: None,
            diagnostics: Tristate::Unknown,
            extended_diagnostics: Tristate::Unknown,
            generate_cpu_profile: None,
            generate_trace: None,
            list_emitted_files: Tristate::Unknown,
            list_files: Tristate::Unknown,
            explain_files: Tristate::Unknown,
            list_files_only: Tristate::Unknown,
            no_emit_for_js_files: Tristate::Unknown,
            preserve_watch_output: Tristate::Unknown,
            pretty: Tristate::Unknown,
            version: Tristate::Unknown,
            watch: Tristate::Unknown,
            show_config: Tristate::Unknown,
            tsc_build: Tristate::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleDetectionKind {
    #[serde(rename = "none")]
    None = 0,
    #[serde(rename = "auto")]
    Auto = 1,
    #[serde(rename = "legacy")]
    Legacy = 2,
    #[serde(rename = "force")]
    Force = 3,
}

impl Default for ModuleDetectionKind {
    fn default() -> Self {
        ModuleDetectionKind::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleKind {
    #[serde(rename = "none")]
    None = 0,
    #[serde(rename = "commonjs")]
    CommonJS = 1,
    #[serde(rename = "amd")]
    AMD = 2,
    #[serde(rename = "umd")]
    UMD = 3,
    #[serde(rename = "system")]
    System = 4,
    #[serde(rename = "es2015")]
    ES2015 = 5,
    #[serde(rename = "es2020")]
    ES2020 = 6,
    #[serde(rename = "es2022")]
    ES2022 = 7,
    #[serde(rename = "esnext")]
    ESNext = 99,
    #[serde(rename = "node16")]
    Node16 = 100,
    #[serde(rename = "nodenext")]
    NodeNext = 199,
    #[serde(rename = "preserve")]
    Preserve = 200,
}

impl Default for ModuleKind {
    fn default() -> Self {
        ModuleKind::None
    }
}

// Resolution mode type alias
pub type ResolutionMode = ModuleKind;

// Resolution mode constants
pub const RESOLUTION_MODE_NONE: ResolutionMode = ModuleKind::None;
pub const RESOLUTION_MODE_COMMON_JS: ResolutionMode = ModuleKind::CommonJS;
pub const RESOLUTION_MODE_ESM: ResolutionMode = ModuleKind::ESNext;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModuleResolutionKind {
    #[serde(rename = "unknown")]
    Unknown = 0,
    #[serde(rename = "node16")]
    Node16 = 3,
    #[serde(rename = "nodenext")]
    NodeNext = 99,
    #[serde(rename = "bundler")]
    Bundler = 100,
}

impl Default for ModuleResolutionKind {
    fn default() -> Self {
        ModuleResolutionKind::Unknown
    }
}

impl std::fmt::Display for ModuleResolutionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleResolutionKind::Unknown => {
                panic!("should not use zero value of ModuleResolutionKind")
            }
            ModuleResolutionKind::Node16 => write!(f, "Node16"),
            ModuleResolutionKind::NodeNext => write!(f, "NodeNext"),
            ModuleResolutionKind::Bundler => write!(f, "Bundler"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NewLineKind {
    #[serde(rename = "none")]
    None = 0,
    #[serde(rename = "crlf")]
    CRLF = 1,
    #[serde(rename = "lf")]
    LF = 2,
}

impl Default for NewLineKind {
    fn default() -> Self {
        NewLineKind::None
    }
}

impl NewLineKind {
    pub fn get_new_line_character(&self) -> &str {
        match self {
            NewLineKind::CRLF => "\r\n",
            _ => "\n",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum ScriptTarget {
    #[serde(rename = "none")]
    None = 0, // Also ES3 (deprecated)
    #[serde(rename = "es5")]
    ES5 = 1,
    #[serde(rename = "es2015")]
    ES2015 = 2,
    #[serde(rename = "es2016")]
    ES2016 = 3,
    #[serde(rename = "es2017")]
    ES2017 = 4,
    #[serde(rename = "es2018")]
    ES2018 = 5,
    #[serde(rename = "es2019")]
    ES2019 = 6,
    #[serde(rename = "es2020")]
    ES2020 = 7,
    #[serde(rename = "es2021")]
    ES2021 = 8,
    #[serde(rename = "es2022")]
    ES2022 = 9,
    #[serde(rename = "es2023")]
    ES2023 = 10,
    #[serde(rename = "esnext")]
    ESNext = 99,
    #[serde(rename = "json")]
    JSON = 100,
}

impl Default for ScriptTarget {
    fn default() -> Self {
        ScriptTarget::None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JsxEmit {
    #[serde(rename = "none")]
    None = 0,
    #[serde(rename = "preserve")]
    Preserve = 1,
    #[serde(rename = "react-native")]
    ReactNative = 2,
    #[serde(rename = "react")]
    React = 3,
    #[serde(rename = "react-jsx")]
    ReactJSX = 4,
    #[serde(rename = "react-jsxdev")]
    ReactJSXDev = 5,
}

impl Default for JsxEmit {
    fn default() -> Self {
        JsxEmit::None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceFileAffectingCompilerOptions {
    pub target: ScriptTarget,
    pub jsx: JsxEmit,
    #[serde(rename = "jsxImportSource")]
    pub jsx_import_source: Option<String>,
    #[serde(rename = "importHelpers")]
    pub import_helpers: Tristate,
    #[serde(rename = "alwaysStrict")]
    pub always_strict: Tristate,
    #[serde(rename = "moduleDetection")]
    pub module_detection: ModuleDetectionKind,
    #[serde(rename = "allowUnreachableCode")]
    pub allow_unreachable_code: Tristate,
    #[serde(rename = "allowUnusedLabels")]
    pub allow_unused_labels: Tristate,
    #[serde(rename = "preserveConstEnums")]
    pub preserve_const_enums: Tristate,
    #[serde(rename = "isolatedModules")]
    pub isolated_modules: Tristate,
}

impl CompilerOptions {
    pub fn get_emit_script_target(&self) -> ScriptTarget {
        if self.target != ScriptTarget::None {
            return self.target;
        }
        ScriptTarget::ES5
    }

    pub fn get_emit_module_kind(&self) -> ModuleKind {
        if self.module_kind != ModuleKind::None {
            return self.module_kind;
        }
        if self.target >= ScriptTarget::ES2015 {
            return ModuleKind::ES2015;
        }
        ModuleKind::CommonJS
    }

    pub fn get_module_resolution_kind(&self) -> ModuleResolutionKind {
        if self.module_resolution != ModuleResolutionKind::Unknown {
            return self.module_resolution;
        }
        match self.get_emit_module_kind() {
            ModuleKind::Node16 => ModuleResolutionKind::Node16,
            ModuleKind::NodeNext => ModuleResolutionKind::NodeNext,
            _ => ModuleResolutionKind::Bundler,
        }
    }

    pub fn get_es_module_interop(&self) -> bool {
        if self.es_module_interop != Tristate::Unknown {
            return self.es_module_interop.is_true();
        }
        matches!(
            self.get_emit_module_kind(),
            ModuleKind::Node16 | ModuleKind::NodeNext | ModuleKind::Preserve
        )
    }

    pub fn get_allow_synthetic_default_imports(&self) -> bool {
        if self.allow_synthetic_default_imports != Tristate::Unknown {
            return self.allow_synthetic_default_imports.is_true();
        }
        self.get_es_module_interop()
            || self.get_emit_module_kind() == ModuleKind::System
            || self.get_module_resolution_kind() == ModuleResolutionKind::Bundler
    }

    pub fn get_resolve_json_module(&self) -> bool {
        if self.resolve_json_module != Tristate::Unknown {
            return self.resolve_json_module.is_true();
        }
        self.get_module_resolution_kind() == ModuleResolutionKind::Bundler
    }

    pub fn should_preserve_const_enums(&self) -> bool {
        self.preserve_const_enums.is_true() || self.isolated_modules.is_true()
    }

    pub fn get_allow_js(&self) -> bool {
        if self.allow_js != Tristate::Unknown {
            return self.allow_js.is_true();
        }
        self.check_js.is_true()
    }

    pub fn get_jsx_transform_enabled(&self) -> bool {
        matches!(
            self.jsx,
            JsxEmit::React | JsxEmit::ReactJSX | JsxEmit::ReactJSXDev
        )
    }

    pub fn get_effective_type_roots(
        &self,
        current_directory: &str,
    ) -> Result<(Vec<String>, bool), error::Error> {
        if let Some(type_roots) = &self.type_roots {
            return Ok((type_roots.clone(), true));
        }

        let base_dir = if let Some(config_path) = &self.config_file_path {
            Path::new(config_path)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| String::new())
        } else {
            if current_directory.is_empty() {
                return Err(error::Error::NoConfigFileOrCurrentDirectory);
            }
            current_directory.to_string()
        };

        let mut type_roots = Vec::new();
        let mut dir = PathBuf::from(&base_dir);

        while let Some(parent) = dir.parent() {
            let node_types = parent.join("node_modules").join("@types");
            type_roots.push(node_types.to_string_lossy().to_string());
            dir = parent.to_path_buf();
        }

        Ok((type_roots, false))
    }

    pub fn get_isolated_modules(&self) -> bool {
        self.isolated_modules.is_true() || self.verbatim_module_syntax.is_true()
    }

    pub fn get_emit_standard_class_fields(&self) -> bool {
        !self.use_define_for_class_fields.is_false()
            && self.get_emit_script_target() >= ScriptTarget::ES2022
    }

    pub fn get_emit_declarations(&self) -> bool {
        // Implement according to your needs
        false
    }

    pub fn get_are_declaration_maps_enabled(&self) -> bool {
        // Implement according to your needs
        false
    }

    pub fn has_json_module_emit_enabled(&self) -> bool {
        match self.get_emit_module_kind() {
            ModuleKind::None | ModuleKind::System | ModuleKind::UMD => false,
            _ => true,
        }
    }

    pub fn source_file_affecting(&self) -> SourceFileAffectingCompilerOptions {
        SourceFileAffectingCompilerOptions {
            target: self.target,
            jsx: self.jsx,
            jsx_import_source: self.jsx_import_source.clone(),
            import_helpers: self.import_helpers,
            always_strict: self.always_strict,
            module_detection: self.module_detection,
            allow_unreachable_code: self.allow_unreachable_code,
            allow_unused_labels: self.allow_unused_labels,
            preserve_const_enums: self.preserve_const_enums,
            isolated_modules: self.isolated_modules,
        }
    }
}
