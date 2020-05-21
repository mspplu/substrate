// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Key related CLI utilities

use sc_cli::{Error, substrate_cli_subcommands};
use structopt::StructOpt;
use frame_utils::{HashFor, SignedExtensionProvider};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
	generate_node_key::GenerateNodeIdCmd,
	generate::GenerateCmd,
	inspect::InspectCmd,
	insert::InsertCmd
};

/// key utilities for the cli.
#[derive(Debug, Clone, StructOpt)]
pub enum KeySubcommand {
	/// Generate a random node libp2p key, save it to file and print its peer ID
	GenerateNodeKey(GenerateNodeIdCmd),

	/// Generate a random account
	Generate(GenerateCmd),

	/// Gets a public key and a SS58 address from the provided Secret URI
	InspectKey(InspectCmd),

	/// Insert a key to the keystore of a node.
	Insert(InsertCmd),
}

impl KeySubcommand {
	/// run the key subcommands
	pub fn run<P>(&self) -> Result<(), Error>
		where
			P: SignedExtensionProvider,
			HashFor<P>: DeserializeOwned + Serialize + Send + Sync,
	{
		match self {
			KeySubcommand::GenerateNodeKey(cmd) => cmd.run(),
			KeySubcommand::Generate(cmd) => cmd.run(),
			KeySubcommand::InspectKey(cmd) => cmd.run(),
			KeySubcommand::Insert(cmd) => cmd.run::<P>(),
		}
	}
}

// CliConfiguration implementation
substrate_cli_subcommands!(
	KeySubcommand =>
	GenerateNodeKey,
	Generate,
	InspectKey,
	Insert
);
