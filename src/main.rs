use prettyplease;
use quote::format_ident;
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};
use syn::visit::{self, Visit};
use syn::{self, parse_file};
use syn::{fold::Fold, File};

struct BlockVarCounter {
    blocks: Vec<BTreeMap<String, usize>>,
    decls: BTreeMap<String, usize>,
    uses: BTreeMap<String, usize>,
}

struct SinglesAnalysis {
    to_single: HashSet<String>,
    to_plain: HashSet<String>,
}

struct SingleRenamer {
    analysis: SinglesAnalysis,
}

impl Fold for SingleRenamer {
    fn fold_pat_ident(&mut self, mut pat: syn::PatIdent) -> syn::PatIdent {
        let name = pat.ident.to_string();
        let base = name.trim_start_matches('_');

        match (
            name.starts_with('_'),
            self.analysis.to_single.contains(base),
            self.analysis.to_plain.contains(base),
        ) {
            // familie soll _base heißen → sicherstellen, dass genau ein underscore davor ist
            (_, true, false) => {
                pat.ident = format_ident!("_{}", base);
            }

            // familie soll base heißen → alle underscores weg
            (_, false, true) => {
                pat.ident = format_ident!("{base}");
            }

            _ => {}
        }

        syn::fold::fold_pat_ident(self, pat)
    }

    fn fold_expr_path(&mut self, mut expr: syn::ExprPath) -> syn::ExprPath {
        if let Some(segment) = expr.path.segments.first_mut() {
            let name = segment.ident.to_string();
            let base = name.trim_start_matches('_');

            match (
                name.starts_with('_'),
                self.analysis.to_single.contains(base),
                self.analysis.to_plain.contains(base),
            ) {
                // komplette familie soll _base heißen
                (_, true, false) => {
                    segment.ident = format_ident!("_{}", base);
                }

                // komplette familie soll base heißen
                (_, false, true) => {
                    segment.ident = format_ident!("{base}");
                }

                _ => {}
            }
        }

        syn::fold::fold_expr_path(self, expr)
    }
}

impl BlockVarCounter {
    fn new() -> Self {
        Self {
            blocks: vec![],
            decls: BTreeMap::new(),
            uses: BTreeMap::new(),
        }
    }

    fn push_block(&mut self) {
        self.blocks.push(BTreeMap::new());
    }

    fn pop_block(&mut self) {}

    fn analyze(&self) -> SinglesAnalysis {
        let mut decls_base: HashMap<String, usize> = HashMap::new();
        let mut uses_base: HashMap<String, usize> = HashMap::new();

        // deklarationen pro basis-name sammeln
        for (name, decl_count) in &self.decls {
            let base = name.trim_start_matches('_').to_string();
            *decls_base.entry(base).or_insert(0) += *decl_count;
        }

        // uses pro basis-name sammeln
        for (name, use_count) in &self.uses {
            let base = name.trim_start_matches('_').to_string();
            *uses_base.entry(base).or_insert(0) += *use_count;
        }

        let mut to_single = HashSet::new();
        let mut to_plain = HashSet::new();

        for (base, decls) in decls_base {
            let uses = uses_base.get(&base).copied().unwrap_or(0);

            if decls > 0 && uses == 0 {
                // nie benutzt → komplette familie soll _base heißen
                to_single.insert(base);
            } else if decls > 0 && uses > 0 {
                // benutzt → komplette familie soll base heißen
                to_plain.insert(base);
            }
        }

        SinglesAnalysis {
            to_single,
            to_plain,
        }
    }
}

impl<'ast> Visit<'ast> for BlockVarCounter {
    fn visit_local(&mut self, local: &'ast syn::Local) {
        if let syn::Pat::Ident(p) = &local.pat {
            let name = p.ident.to_string();

            if let Some(last) = self.blocks.last_mut() {
                *last.entry(name.clone()).or_insert(0) += 1;
            }

            *self.decls.entry(name).or_insert(0) += 1;
        }
        visit::visit_local(self, local);
    }

    fn visit_block(&mut self, b: &'ast syn::Block) {
        self.push_block();
        visit::visit_block(self, b);
        self.pop_block();
    }

    fn visit_expr_path(&mut self, expr: &'ast syn::ExprPath) {
        if let Some(ident) = expr.path.get_ident() {
            let name = ident.to_string();
            *self.uses.entry(name).or_insert(0) += 1;
        }
        visit::visit_expr_path(self, expr);
    }
}

fn process_file(path: &Path) -> Result<(), Box<dyn Error>> {
    let src = fs::read_to_string(path)?;
    let ast: File = parse_file(&src)?;

    let mut counter = BlockVarCounter::new();
    counter.visit_file(&ast);

    let analysis = counter.analyze();

    let mut renamer = SingleRenamer { analysis };
    let new_ast: File = renamer.fold_file(ast);

    let new_src = prettyplease::unparse(&new_ast);

    fs::write(path, new_src)?;

    Ok(())
}

fn main() {
    let arg = env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: syr <file.rs>");
        std::process::exit(1);
    });

    let path = PathBuf::from(arg);

    if let Err(err) = process_file(&path) {
        eprintln!("syr error: {err}");
        std::process::exit(1);
    }
}
