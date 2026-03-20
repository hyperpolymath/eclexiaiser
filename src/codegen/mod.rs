// SPDX-License-Identifier: PMPL-1.0-or-later
// Copyright (c) 2026 Jonathan D.A. Jewell <j.d.a.jewell@open.ac.uk>
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use crate::manifest::Manifest;

pub fn generate_all(manifest: &Manifest, output_dir: &str) -> Result<()> {
    fs::create_dir_all(output_dir).context("Failed to create output dir")?;
    println!("  [stub] Eclexia codegen for '{}' — implementation pending", manifest.workload.name);
    Ok(())
}

pub fn build(manifest: &Manifest, _release: bool) -> Result<()> {
    println!("Building eclexiaiser workload: {}", manifest.workload.name);
    Ok(())
}

pub fn run(manifest: &Manifest, _args: &[String]) -> Result<()> {
    println!("Running eclexiaiser workload: {}", manifest.workload.name);
    Ok(())
}
