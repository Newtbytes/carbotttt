use std::fmt::Display;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

use crate::error::CompilerError;
use crate::parser;
use crate::parser::ast;
use dialect::x86;
use lorax::rewrite_ops;

const CC: &str = "gcc";

#[derive(Clone, PartialEq)]
pub enum ProcFileKind {
    Source,
    Preprocessed,
    Assembly,
    Binary,
}

impl From<&str> for ProcFileKind {
    fn from(ext: &str) -> Self {
        match ext {
            "c" => ProcFileKind::Source,
            "i" => ProcFileKind::Preprocessed,
            "S" => ProcFileKind::Assembly,
            _ => ProcFileKind::Binary,
        }
    }
}

impl ProcFileKind {
    fn get_ext(&self) -> &str {
        match self {
            ProcFileKind::Source => ".c",
            ProcFileKind::Preprocessed => ".i",
            ProcFileKind::Assembly => ".S",
            ProcFileKind::Binary => "",
        }
    }
}

impl Display for ProcFileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &str = match self {
            ProcFileKind::Source => "Source",
            ProcFileKind::Preprocessed => "Preprocessed",
            ProcFileKind::Assembly => "Assembly",
            ProcFileKind::Binary => "Binary",
        };

        write!(f, "{}", name)
    }
}

#[derive(Clone)]
pub struct ProcFile<'a> {
    pub name: String,
    pub path: &'a Path,
    pub kind: ProcFileKind,
}

impl<'a> ProcFile<'a> {
    pub fn from_path(path: &'a Path) -> Option<Self> {
        let parent = path.parent().unwrap_or_else(|| Path::new(""));
        let name = path.file_stem()?.to_str()?.to_owned();
        let kind = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(ProcFileKind::from)
            .unwrap_or(ProcFileKind::Binary);

        Some(Self {
            name,
            path: parent,
            kind,
        })
    }

    pub fn from_fn(filename: &'a str) -> Option<Self> {
        Self::from_path(Path::new(filename))
    }

    fn get_fn(&self) -> PathBuf {
        self.path.join(self.name.clone() + self.kind.get_ext())
    }

    pub fn to_kind(&self, kind: ProcFileKind) -> Self {
        let mut cpy = self.clone();
        cpy.kind = kind;
        cpy
    }

    pub fn write(&self, src: String) -> io::Result<()> {
        fs::write(self.get_fn(), src)?;
        Ok(())
    }

    // Consumes self
    pub fn read(self) -> io::Result<String> {
        fs::read_to_string(self.get_fn())
    }
}

impl Drop for ProcFile<'_> {
    fn drop(&mut self) {
        if self.kind != ProcFileKind::Source && self.kind != ProcFileKind::Binary {
            fs::remove_file(self.get_fn()).ok();
        }
    }
}

pub fn preprocess(src: ProcFile) -> io::Result<ProcFile> {
    let mut dst = src.clone();
    dst.kind = ProcFileKind::Preprocessed;

    Command::new(CC)
        .arg("-E")
        .arg("-P")
        .arg(src.get_fn())
        .arg("-o")
        .arg(dst.get_fn())
        .output()?;

    Ok(dst)
}

pub fn assemble(src: ProcFile) -> io::Result<ProcFile> {
    let mut dst = src.clone();
    dst.kind = ProcFileKind::Binary;

    Command::new(CC)
        .arg(src.get_fn())
        .arg("-o")
        .arg(dst.get_fn())
        .output()?;

    Ok(dst)
}

pub fn tokenize(src: &str) -> Result<Vec<ast::Token>, CompilerError> {
    parser::tokenize(src)
}

pub fn parser(tokens: Vec<ast::Token>) -> Result<ast::Program, CompilerError> {
    parser::parse(&mut tokens.into_iter()).map_err(CompilerError::Parser)
}

#[derive(clap::Parser)]
pub struct Cli {
    input: String,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    lex: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    parse: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    tacky: bool,

    #[arg(long, action = clap::ArgAction::SetTrue)]
    codegen: bool,
}

pub fn run_compiler(cli: Cli) -> Result<(), CompilerError> {
    let input_fn = cli.input;

    let file = ProcFile::from_fn(&input_fn)
        .ok_or_else(|| CompilerError::Parser("Invalid source file".to_string()))?;

    let src_file = preprocess(file)?;
    let asm_file = src_file.to_kind(ProcFileKind::Assembly);
    let src = src_file.read()?;

    // tokenization
    let tokens = tokenize(&src)?;
    if cli.lex {
        dbg!(&tokens);
        return Ok(());
    }

    // parsing
    let ast = parser(tokens)?;
    if cli.parse {
        dbg!(ast);
        return Ok(());
    }

    // 'tacky' is the option to generate IR
    let ir = &mut parser::lower_program(&ast);
    if cli.tacky {
        println!("{}", ir);
        return Ok(());
    }

    // codegen

    // TODO: put this somewhere else

    rewrite_ops(ir, x86::rules());

    println!("{}", ir);

    if cli.codegen {
        println!("{}", ir);
        return Ok(());
    }

    todo!()

    // asm_file.write(asm)?;
    // assemble(asm_file)?;

    // Ok(())
}
