/*
 * Copyright © 2020-2022 The fjp Authors
 *
 * This file is part of fjp
 *
 * fjp is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * fjp is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */

use clap::{ArgEnum, Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Debug, Subcommand)]
pub enum Subcommands {
    Cat(CliCat),
    Diff(CliDiff),
    Disable(CliDisable),
    Edit(CliEdit),
    Enable(CliEnable),
    GenerateStandalone(CliGenerateStandalone),
    Has(CliHas),
    List(CliList),
    Rm(CliRm),
}

#[derive(Debug, Args)]
#[clap(about = "Show a profile, its .local and its redirect profile")]
pub struct CliCat {
    #[clap(long, help = "Do not show .local files.")]
    pub no_locals: bool,
    #[clap(long, help = "Do not pipe output into a pager.")]
    pub no_pager: bool,
    #[clap(long, help = "Do not show redirect profiles.")]
    pub no_redirects: bool,
    #[clap(help = "The name of the profile to show.")]
    pub profile_name: String,
}

#[derive(Debug, Args)]
#[clap(about = "Show the differences between two profiles")]
pub struct CliDiff {
    #[clap(
        short, long,
        arg_enum,
        help = "specify the diff format",
        long_help = concat!(
            "specify the diff format\n",
            " color: highlight unique lines\n",
            " simple: show unique lines\n",
        ),
    )]
    pub format: CliDiffFormat,
    pub profile_name1: String,
    pub profile_name2: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ArgEnum)]
pub enum CliDiffFormat {
    Color,
    Simple,
}

#[derive(Debug, Args)]
#[clap(about = "Disable profiles")]
pub struct CliDisable {
    #[clap(short, long, exclusive = true, help = "List all disabled profiles")]
    pub list: bool,
    #[clap(
        short,
        long,
        exclusive = true,
        help = "Disable ~/.config/firejail",
        long_help = "Disable ~/.config/firejail by renaming it to firejail.disabled"
    )]
    pub user: bool,
    #[clap(
        required_unless_present_any = &["list", "user"],
        help = "The name of the profile to disable",
    )]
    pub profile_name: Option<String>,
}

#[derive(Debug, Args)]
#[clap(about = "Edit profiles")]
pub struct CliEdit {
    #[clap(
        short,
        long,
        help = "Edit non-persistent",
        long_help = "Copy the profile if possible and discard all changes after editing."
    )]
    pub tmp: bool,
    #[clap(
        help = "The name of the profile to edit.",
        long_help = concat!(
            "The name of the profile to edit. If the profile does not exists,",
            "it is create except it is found in /etc/firejail, then it is copied from there.",
        ),
    )]
    pub profile_name: String,
}

#[derive(Debug, Args)]
#[clap(about = "Enable profiles")]
pub struct CliEnable {
    #[clap(short, long, exclusive = true, help = "Enable ~/.config/firejail")]
    pub user: bool,
    #[clap(
        required_unless_present = "user",
        help = "The name of the profile to enable"
    )]
    pub profile_name: Option<String>,
}

#[derive(Debug, Args)]
#[clap(about = "Copy the profile and all its includes into one file.")]
pub struct CliGenerateStandalone {
    #[clap(long, help = "Keep all includes of .inc's")]
    pub keep_inc: bool,
    #[clap(long, help = "Keep all includes of .local's")]
    pub keep_locals: bool,
    #[clap(short, long, help = "The name of the file to write results")]
    pub output_file: Option<String>,
    #[clap(help = "The name of the profile to generate a standalone version.")]
    pub profile_name: String,
}

#[derive(Debug, Args)]
#[clap(about = "Look if a profile exists")]
pub struct CliHas {
    #[clap(help = "The name of the program to look for a profile.")]
    pub profile_name: String,
}

#[derive(Debug, Args)]
#[clap(about = "List all user profile")]
pub struct CliList {
    #[clap(
        long,
        conflicts_with_all = &["locals", "profiles"],
        help = "List only .inc",
    )]
    pub incs: bool,
    #[clap(
        long,
        conflicts_with_all = &["incs", "profiles"],
        help = "List only .local",
    )]
    pub locals: bool,
    #[clap(
        long,
        conflicts_with_all = &["incs", "locals"],
        help = "List only .profile",
    )]
    pub profiles: bool,
}

#[derive(Debug, Args)]
#[clap(about = "Remove profiles")]
pub struct CliRm {
    #[clap(required = true, help = "The names of the profiles to delete.")]
    pub profile_names: Vec<String>,
}
