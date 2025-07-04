use std::path::PathBuf;

use aderyn::{
    aderyn_is_currently_running_newest_version, birdsong, create_aderyn_toml_file_at,
    initialize_niceties, lsp::spin_up_language_server, print_all_detectors_view, print_detail_view,
};
use aderyn_driver::driver::{self, kick_off_report_creation, Args};
use clap::{ArgGroup, Parser, Subcommand};
use indoc::indoc;

mod gemini;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = indoc!{
        r#"Aderyn - Rust based Solidity Static analyzer.

        Quickstart:
        cd my-solidity-project/
        aderyn

        It outputs report.md if the solidity project is foundry/hardhat/soldeer.

        In the case that it's not, it's important to create a config file via the
        command `aderyn init` in the workspace root.

        For more examples, visit docs: https://cyfrin.gitbook.io/cyfrin-docs/aderyn-cli
        Also ask questions via command line: `aderyn docs "how to configure scan options?"`

        Help Aderyn stay open source by giving us a star on Github.
        Repository: https://github.com/cyfrin/aderyn
    "#},
    group(ArgGroup::new("stdout_dependent").requires("stdout")),
)]
pub struct CommandLineArgs {
    /// Commands to initialize a config file and docs help
    #[clap(subcommand)]
    subcommand: Option<MainSubcommand>,

    /// Solidity project root directory
    #[arg(default_value = ".")]
    root: String,

    /// Path to the contracts source directory (relative to the root)
    /// By default, it is auto detected in most projects.
    #[clap(short, long, use_value_delimiter = true, verbatim_doc_comment)]
    src: Option<String>,

    /// List of path fragments to include, delimited by comma (no spaces)
    /// By default, it is auto detected.
    ///
    /// Use this to include only specified source files in the analysis:
    /// Examples:
    ///     -i src/MyContract.sol
    ///     -i src/MyContract.sol,src/MyOtherContract.sol
    #[clap(short = 'i', long, use_value_delimiter = true, verbatim_doc_comment)]
    path_includes: Option<Vec<String>>,

    /// List of path fragments to exclude, delimited by comma (no spaces)
    /// By default, it is auto detected.
    ///
    /// Use this to exclude only specified source files in the analysis:
    /// Examples:
    ///     -x src/MyContract.sol
    ///     -x src/MyContract.sol,src/MyOtherContract.sol
    #[clap(short = 'x', long, use_value_delimiter = true, verbatim_doc_comment)]
    path_excludes: Option<Vec<String>>,

    /// Desired file path for the final report
    /// Output file extension (.json/.md/.sarif) decides the format.
    ///
    /// NOTE: Allowed formats: JSON, Markdown, Sarif
    /// NOTE: Overwrites existing file if found in the same path.
    #[arg(short, long, default_value = "report.md", verbatim_doc_comment)]
    output: String,

    /// Start Aderyn's LSP server on stdout. (Must be accompanied with `--stdout`)
    #[arg(short, long, group = "stdout_dependent")]
    lsp: bool,

    /// Only use the high detectors
    #[arg(long)]
    highs_only: bool,

    /// Serialize the reports to stdout, don't write to files.
    #[arg(long, name = "stdout", hide = true)]
    stdout: bool,

    /// Skip counting number of lines of code.
    #[arg(long, hide = true)]
    skip_cloc: bool,

    /// After generating report, skip checking if a new version of Aderyn is available.
    #[arg(long)]
    skip_update_check: bool,

    /// Run in Auditor mode, which only outputs manual audit helpers
    #[arg(long, hide = true)]
    auditor_mode: bool,

    /// Do not include code snippets in the report (reduces markdown report size in large repos)
    #[arg(long, hide = true)]
    no_snippets: bool,
}

#[derive(Debug, Subcommand)]
enum MainSubcommand {
    /// Browse detector registry
    Registry {
        /// all    - View all available detectors
        ///
        /// <n> - Detail view of a single detector
        #[arg(default_value = "all", verbatim_doc_comment)]
        detector: String,
    },
    /// Initializes aderyn.toml. Required when solidity project root is not the workspace root
    Init {
        /// Optional path inside root where aderyn.toml will be created
        path: Option<String>,
    },
    /// Browse Aderyn documentation
    /// Chat with AI for help - aderyn docs "how to exclude files from scan?"
    Docs {
        /// Ask question
        question: Option<String>,
    },
    /// Analyze contracts using Gemini AI
    /// Flattens contracts and sends to Gemini API for AI-powered analysis
    Gemini {
        /// Optional output file for the analysis report
        #[arg(short, long, default_value = "gemini_analysis.md")]
        output: String,
    },
}

fn main() {
    initialize_niceties();
    let cmd_args = CommandLineArgs::parse();

    if let Some(subcommand) = cmd_args.subcommand {
        match subcommand {
            MainSubcommand::Registry { detector } => {
                if detector == "all" {
                    print_all_detectors_view();
                } else {
                    print_detail_view(&detector);
                }
            }
            MainSubcommand::Init { path } => {
                let creation_path = match path {
                    Some(optional_path) => {
                        let mut target_dir = PathBuf::from(&cmd_args.root);
                        target_dir.push(optional_path);

                        let can_initialize = target_dir.exists()
                            && std::fs::metadata(&target_dir).is_ok_and(|p| p.is_dir());

                        if !can_initialize {
                            eprintln!("Error: Cannot initialize aderyn.toml at the given path");
                            std::process::exit(1);
                        }

                        target_dir
                    }
                    None => PathBuf::from(&cmd_args.root),
                };

                if let Err(e) = create_aderyn_toml_file_at(&creation_path) {
                    eprintln!("Error: {}", e);
                    std::process::exit(1);
                }
            }
            MainSubcommand::Docs { question } => {
                birdsong(question);
            }
            MainSubcommand::Gemini { output } => {
                tokio::runtime::Runtime::new().unwrap().block_on(async {
                    handle_gemini_analysis(&cmd_args.root, &output).await;
                });
                return;
            }
        }
        return;
    }

    if cmd_args.lsp {
        spin_up_language_server();
        return;
    }

    let args = Args {
        input_config: driver::CliArgsInputConfig {
            root: cmd_args.root,
            src: cmd_args.src,
            path_excludes: cmd_args.path_excludes,
            path_includes: cmd_args.path_includes,
        },
        output_config: driver::CliArgsOutputConfig {
            output: cmd_args.output,
            stdout: cmd_args.stdout,
            no_snippets: cmd_args.no_snippets,
        },
        common_config: driver::CliArgsCommonConfig {
            lsp: cmd_args.lsp,
            skip_cloc: cmd_args.skip_cloc,
            highs_only: cmd_args.highs_only,
        },
    };

    if cmd_args.auditor_mode {
        driver::kick_off_audit_mode(args);
    } else {
        kick_off_report_creation(args);
    }

    if !cmd_args.skip_update_check {
        aderyn_is_currently_running_newest_version();
    }
}

async fn handle_gemini_analysis(root_path: &str, output_file: &str) {
    println!("Starting Gemini AI Analysis...");
    println!("===============================");
    
    // Step 1: Flatten contracts
    println!("Flattening contracts from: {}", root_path);
    let (flattened_code, project_name) = match gemini::flatten_contracts(root_path) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error flattening contracts: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("Found and flattened contracts for project: {}", project_name);
    println!("Total code size: {} characters", flattened_code.len());
    
    // Step 2: Get API credentials
    let (api_key, model) = match gemini::prompt_for_api_credentials() {
        Ok(credentials) => credentials,
        Err(e) => {
            eprintln!("Error getting API credentials: {}", e);
            std::process::exit(1);
        }
    };
    
    // Step 3: Analyze with Gemini
    println!("\nSending code to Gemini AI for analysis...");
    println!("This may take a few moments...");
    
    let analyzer = gemini::GeminiAnalyzer::new(api_key, model);
    
    let analysis_report = match analyzer.analyze_contracts(&flattened_code, &project_name).await {
        Ok(report) => report,
        Err(e) => {
            eprintln!("Error during Gemini analysis: {}", e);
            std::process::exit(1);
        }
    };
    
    // Step 4: Save report
    match std::fs::write(output_file, &analysis_report) {
        Ok(_) => {
            println!("Analysis complete!");
            println!("Report saved to: {}", output_file);
            println!("Open the file to view detailed AI analysis results");
        }
        Err(e) => {
            eprintln!("Error saving report: {}", e);
            eprintln!("Analysis content:");
            println!("{}", analysis_report);
        }
    }
}