#![allow(unused)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use napi_derive::napi;
use serde::{Deserialize, Serialize};

use crate::error;

#[napi(object, object_to_js = false)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerOptions {
    #[serde(rename = "allowJs")]
    pub allow_js: Option<bool>,
    #[serde(rename = "allowArbitraryExtensions")]
    pub allow_arbitrary_extensions: Option<bool>,
    #[serde(rename = "allowSyntheticDefaultImports")]
    pub allow_synthetic_default_imports: Option<bool>,
    #[serde(rename = "allowImportingTsExtensions")]
    pub allow_importing_ts_extensions: Option<bool>,
    #[serde(rename = "allowNonTsExtensions")]
    pub allow_non_ts_extensions: Option<bool>,
    #[serde(rename = "allowUmdGlobalAccess")]
    pub allow_umd_global_access: Option<bool>,
    #[serde(rename = "allowUnreachableCode")]
    pub allow_unreachable_code: Option<bool>,
    #[serde(rename = "allowUnusedLabels")]
    pub allow_unused_labels: Option<bool>,
    #[serde(rename = "assumeChangesOnlyAffectDirectDependencies")]
    pub assume_changes_only_affect_direct_dependencies: Option<bool>,
    #[serde(rename = "alwaysStrict")]
    pub always_strict: Option<bool>,
    #[serde(rename = "baseUrl")]
    pub base_url: Option<String>,
    pub build: Option<bool>,
    #[serde(rename = "checkJs")]
    pub check_js: Option<bool>,
    #[serde(rename = "customConditions")]
    pub custom_conditions: Option<Vec<String>>,
    pub composite: Option<bool>,
    #[serde(rename = "emitDeclarationOnly")]
    pub emit_declaration_only: Option<bool>,
    #[serde(rename = "emitBOM")]
    pub emit_bom: Option<bool>,
    #[serde(rename = "emitDecoratorMetadata")]
    pub emit_decorator_metadata: Option<bool>,
    #[serde(rename = "downlevelIteration")]
    pub downlevel_iteration: Option<bool>,
    pub declaration: Option<bool>,
    #[serde(rename = "declarationDir")]
    pub declaration_dir: Option<String>,
    #[serde(rename = "declarationMap")]
    pub declaration_map: Option<bool>,
    #[serde(rename = "disableSizeLimit")]
    pub disable_size_limit: Option<bool>,
    #[serde(rename = "disableSourceOfProjectReferenceRedirect")]
    pub disable_source_of_project_reference_redirect: Option<bool>,
    #[serde(rename = "disableSolutionSearching")]
    pub disable_solution_searching: Option<bool>,
    #[serde(rename = "disableReferencedProjectLoad")]
    pub disable_referenced_project_load: Option<bool>,
    #[serde(rename = "esModuleInterop")]
    pub es_module_interop: Option<bool>,
    #[serde(rename = "exactOptionalPropertyTypes")]
    pub exact_optional_property_types: Option<bool>,
    #[serde(rename = "experimentalDecorators")]
    pub experimental_decorators: Option<bool>,
    #[serde(rename = "forceConsistentCasingInFileNames")]
    pub force_consistent_casing_in_file_names: Option<bool>,
    #[serde(rename = "isolatedModules")]
    pub isolated_modules: Option<bool>,
    #[serde(rename = "isolatedDeclarations")]
    pub isolated_declarations: Option<bool>,
    #[serde(rename = "ignoreDeprecations")]
    pub ignore_deprecations: Option<String>,
    #[serde(rename = "importHelpers")]
    pub import_helpers: Option<bool>,
    #[serde(rename = "inlineSourceMap")]
    pub inline_source_map: Option<bool>,
    #[serde(rename = "inlineSources")]
    pub inline_sources: Option<bool>,
    pub init: Option<bool>,
    pub incremental: Option<bool>,
    pub jsx: JsxEmit,
    #[serde(rename = "jsxFactory")]
    pub jsx_factory: Option<String>,
    #[serde(rename = "jsxFragmentFactory")]
    pub jsx_fragment_factory: Option<String>,
    #[serde(rename = "jsxImportSource")]
    pub jsx_import_source: Option<String>,
    #[serde(rename = "keyofStringsOnly")]
    pub keyof_strings_only: Option<bool>,
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
    pub no_emit: Option<bool>,
    #[serde(rename = "noCheck")]
    pub no_check: Option<bool>,
    #[serde(rename = "noErrorTruncation")]
    pub no_error_truncation: Option<bool>,
    #[serde(rename = "noFallthroughCasesInSwitch")]
    pub no_fallthrough_cases_in_switch: Option<bool>,
    #[serde(rename = "noImplicitAny")]
    pub no_implicit_any: Option<bool>,
    #[serde(rename = "noImplicitThis")]
    pub no_implicit_this: Option<bool>,
    #[serde(rename = "noImplicitReturns")]
    pub no_implicit_returns: Option<bool>,
    #[serde(rename = "noEmitHelpers")]
    pub no_emit_helpers: Option<bool>,
    #[serde(rename = "noLib")]
    pub no_lib: Option<bool>,
    #[serde(rename = "noPropertyAccessFromIndexSignature")]
    pub no_property_access_from_index_signature: Option<bool>,
    #[serde(rename = "noUncheckedIndexedAccess")]
    pub no_unchecked_indexed_access: Option<bool>,
    #[serde(rename = "noEmitOnError")]
    pub no_emit_on_error: Option<bool>,
    #[serde(rename = "noUnusedLocals")]
    pub no_unused_locals: Option<bool>,
    #[serde(rename = "noUnusedParameters")]
    pub no_unused_parameters: Option<bool>,
    #[serde(rename = "noResolve")]
    pub no_resolve: Option<bool>,
    #[serde(rename = "noImplicitOverride")]
    pub no_implicit_override: Option<bool>,
    #[serde(rename = "noUncheckedSideEffectImports")]
    pub no_unchecked_side_effect_imports: Option<bool>,
    pub out: Option<String>,
    #[serde(rename = "outDir")]
    pub out_dir: Option<String>,
    #[serde(rename = "outFile")]
    pub out_file: Option<String>,
    pub paths: Option<BTreeMap<String, Vec<String>>>,
    #[serde(rename = "preserveConstEnums")]
    pub preserve_const_enums: Option<bool>,
    #[serde(rename = "preserveSymlinks")]
    pub preserve_symlinks: Option<bool>,
    pub project: Option<String>,
    #[serde(rename = "resolveJsonModule")]
    pub resolve_json_module: Option<bool>,
    #[serde(rename = "resolvePackageJsonExports")]
    pub resolve_package_json_exports: Option<bool>,
    #[serde(rename = "resolvePackageJsonImports")]
    pub resolve_package_json_imports: Option<bool>,
    #[serde(rename = "removeComments")]
    pub remove_comments: Option<bool>,
    #[serde(rename = "rewriteRelativeImportExtensions")]
    pub rewrite_relative_import_extensions: Option<bool>,
    #[serde(rename = "reactNamespace")]
    pub react_namespace: Option<String>,
    #[serde(rename = "rootDir")]
    pub root_dir: Option<String>,
    #[serde(rename = "rootDirs")]
    pub root_dirs: Option<Vec<String>>,
    #[serde(rename = "skipLibCheck")]
    pub skip_lib_check: Option<bool>,
    pub strict: Option<bool>,
    #[serde(rename = "strictBindCallApply")]
    pub strict_bind_call_apply: Option<bool>,
    #[serde(rename = "strictBuiltinIteratorReturn")]
    pub strict_builtin_iterator_return: Option<bool>,
    #[serde(rename = "strictFunctionTypes")]
    pub strict_function_types: Option<bool>,
    #[serde(rename = "strictNullChecks")]
    pub strict_null_checks: Option<bool>,
    #[serde(rename = "strictPropertyInitialization")]
    pub strict_property_initialization: Option<bool>,
    #[serde(rename = "stripInternal")]
    pub strip_internal: Option<bool>,
    #[serde(rename = "skipDefaultLibCheck")]
    pub skip_default_lib_check: Option<bool>,
    #[serde(rename = "sourceMap")]
    pub source_map: Option<bool>,
    #[serde(rename = "sourceRoot")]
    pub source_root: Option<String>,
    #[serde(rename = "suppressOutputPathCheck")]
    pub suppress_output_path_check: Option<bool>,
    pub target: ScriptTarget,
    #[serde(rename = "traceResolution")]
    pub trace_resolution: Option<bool>,
    #[serde(rename = "tsBuildInfoFile")]
    pub ts_build_info_file: Option<String>,
    #[serde(rename = "typeRoots")]
    pub type_roots: Option<Vec<String>>,
    pub types: Option<Vec<String>>,
    #[serde(rename = "useDefineForClassFields")]
    pub use_define_for_class_fields: Option<bool>,
    #[serde(rename = "useUnknownInCatchVariables")]
    pub use_unknown_in_catch_variables: Option<bool>,
    #[serde(rename = "verbatimModuleSyntax")]
    pub verbatim_module_syntax: Option<bool>,
    #[serde(rename = "maxNodeModuleJsDepth")]
    pub max_node_module_js_depth: Option<i32>,

    // Internal fields
    #[serde(rename = "configFilePath")]
    pub config_file_path: Option<String>,
    #[serde(rename = "noDtsResolution")]
    pub no_dts_resolution: Option<bool>,
    #[serde(rename = "pathsBasePath")]
    pub paths_base_path: Option<String>,
    pub diagnostics: Option<bool>,
    #[serde(rename = "extendedDiagnostics")]
    pub extended_diagnostics: Option<bool>,
    #[serde(rename = "generateCpuProfile")]
    pub generate_cpu_profile: Option<String>,
    #[serde(rename = "generateTrace")]
    pub generate_trace: Option<String>,
    #[serde(rename = "listEmittedFiles")]
    pub list_emitted_files: Option<bool>,
    #[serde(rename = "listFiles")]
    pub list_files: Option<bool>,
    #[serde(rename = "explainFiles")]
    pub explain_files: Option<bool>,
    #[serde(rename = "listFilesOnly")]
    pub list_files_only: Option<bool>,
    #[serde(rename = "noEmitForJsFiles")]
    pub no_emit_for_js_files: Option<bool>,
    #[serde(rename = "preserveWatchOutput")]
    pub preserve_watch_output: Option<bool>,
    pub pretty: Option<bool>,
    pub version: Option<bool>,
    pub watch: Option<bool>,
    #[serde(rename = "showConfig")]
    pub show_config: Option<bool>,
    #[serde(rename = "tscBuild")]
    pub tsc_build: Option<bool>,
}

impl Default for CompilerOptions {
    fn default() -> Self {
        CompilerOptions {
            allow_js: None,
            allow_arbitrary_extensions: None,
            allow_synthetic_default_imports: None,
            allow_importing_ts_extensions: None,
            allow_non_ts_extensions: None,
            allow_umd_global_access: None,
            allow_unreachable_code: None,
            allow_unused_labels: None,
            assume_changes_only_affect_direct_dependencies: None,
            always_strict: None,
            base_url: None,
            build: None,
            check_js: None,
            custom_conditions: None,
            composite: None,
            emit_declaration_only: None,
            emit_bom: None,
            emit_decorator_metadata: None,
            downlevel_iteration: None,
            declaration: None,
            declaration_dir: None,
            declaration_map: None,
            disable_size_limit: None,
            disable_source_of_project_reference_redirect: None,
            disable_solution_searching: None,
            disable_referenced_project_load: None,
            es_module_interop: None,
            exact_optional_property_types: None,
            experimental_decorators: None,
            force_consistent_casing_in_file_names: None,
            isolated_modules: None,
            isolated_declarations: None,
            ignore_deprecations: None,
            import_helpers: None,
            inline_source_map: None,
            inline_sources: None,
            init: None,
            incremental: None,
            jsx: JsxEmit::None,
            jsx_factory: None,
            jsx_fragment_factory: None,
            jsx_import_source: None,
            keyof_strings_only: None,
            lib: None,
            locale: None,
            map_root: None,
            module_kind: ModuleKind::None,
            module_resolution: ModuleResolutionKind::Unknown,
            module_suffixes: None,
            module_detection: ModuleDetectionKind::None,
            new_line: NewLineKind::None,
            no_emit: None,
            no_check: None,
            no_error_truncation: None,
            no_fallthrough_cases_in_switch: None,
            no_implicit_any: None,
            no_implicit_this: None,
            no_implicit_returns: None,
            no_emit_helpers: None,
            no_lib: None,
            no_property_access_from_index_signature: None,
            no_unchecked_indexed_access: None,
            no_emit_on_error: None,
            no_unused_locals: None,
            no_unused_parameters: None,
            no_resolve: None,
            no_implicit_override: None,
            no_unchecked_side_effect_imports: None,
            out: None,
            out_dir: None,
            out_file: None,
            paths: None,
            preserve_const_enums: None,
            preserve_symlinks: None,
            project: None,
            resolve_json_module: None,
            resolve_package_json_exports: None,
            resolve_package_json_imports: None,
            remove_comments: None,
            rewrite_relative_import_extensions: None,
            react_namespace: None,
            root_dir: None,
            root_dirs: None,
            skip_lib_check: None,
            strict: None,
            strict_bind_call_apply: None,
            strict_builtin_iterator_return: None,
            strict_function_types: None,
            strict_null_checks: None,
            strict_property_initialization: None,
            strip_internal: None,
            skip_default_lib_check: None,
            source_map: None,
            source_root: None,
            suppress_output_path_check: None,
            target: ScriptTarget::None,
            trace_resolution: None,
            ts_build_info_file: None,
            type_roots: None,
            types: None,
            use_define_for_class_fields: None,
            use_unknown_in_catch_variables: None,
            verbatim_module_syntax: None,
            max_node_module_js_depth: None,
            config_file_path: None,
            no_dts_resolution: None,
            paths_base_path: None,
            diagnostics: None,
            extended_diagnostics: None,
            generate_cpu_profile: None,
            generate_trace: None,
            list_emitted_files: None,
            list_files: None,
            explain_files: None,
            list_files_only: None,
            no_emit_for_js_files: None,
            preserve_watch_output: None,
            pretty: None,
            version: None,
            watch: None,
            show_config: None,
            tsc_build: None,
        }
    }
}

#[napi]
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

#[napi]
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

#[napi]
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

#[napi]
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

#[napi]
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

#[napi]
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
    pub import_helpers: Option<bool>,
    #[serde(rename = "alwaysStrict")]
    pub always_strict: Option<bool>,
    #[serde(rename = "moduleDetection")]
    pub module_detection: ModuleDetectionKind,
    #[serde(rename = "allowUnreachableCode")]
    pub allow_unreachable_code: Option<bool>,
    #[serde(rename = "allowUnusedLabels")]
    pub allow_unused_labels: Option<bool>,
    #[serde(rename = "preserveConstEnums")]
    pub preserve_const_enums: Option<bool>,
    #[serde(rename = "isolatedModules")]
    pub isolated_modules: Option<bool>,
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
        if self.es_module_interop != None {
            return self.es_module_interop.unwrap_or(false);
        }
        matches!(
            self.get_emit_module_kind(),
            ModuleKind::Node16 | ModuleKind::NodeNext | ModuleKind::Preserve
        )
    }

    pub fn get_allow_synthetic_default_imports(&self) -> bool {
        if self.allow_synthetic_default_imports != None {
            return self.allow_synthetic_default_imports.unwrap_or(false);
        }
        self.get_es_module_interop()
            || self.get_emit_module_kind() == ModuleKind::System
            || self.get_module_resolution_kind() == ModuleResolutionKind::Bundler
    }

    pub fn get_resolve_json_module(&self) -> bool {
        if self.resolve_json_module != None {
            return self.resolve_json_module.unwrap_or(false);
        }
        self.get_module_resolution_kind() == ModuleResolutionKind::Bundler
    }

    pub fn should_preserve_const_enums(&self) -> bool {
        self.preserve_const_enums.unwrap_or(false) || self.isolated_modules.unwrap_or(false)
    }

    pub fn get_allow_js(&self) -> bool {
        if self.allow_js != None {
            return self.allow_js.unwrap_or(false);
        }
        self.check_js.unwrap_or(false)
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
        self.isolated_modules.unwrap_or(false) || self.verbatim_module_syntax.unwrap_or(false)
    }

    pub fn get_emit_standard_class_fields(&self) -> bool {
        !self.use_define_for_class_fields.unwrap_or(false)
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
