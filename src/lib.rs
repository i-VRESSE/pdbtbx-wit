#[allow(warnings)]
mod bindings;

use std::io::BufReader;
use bindings::exports::component::pdbtbx_wit::pdbtbx_api::Guest;
use bindings::exports::component::pdbtbx_wit::pdbtbx_api::Pdbinfo;
use bindings::exports::component::pdbtbx_wit::pdbtbx_api::Residueinfo;
use bindings::exports::component::pdbtbx_wit::pdbtbx_api::Residueinfos;
use pdbtbx::*;

struct Component;

impl Guest for Component {
    fn pdb2pdbinfo(content: String) -> Result<Pdbinfo, Vec<String>> {
        match open_pdb_raw(
            BufReader::new(content.as_bytes()),
            Context::None,
            StrictnessLevel::Loose,
        ) {
            Ok((pdb, warnings)) => {
                let chains: Vec<char> = pdb.chains().map(Chain::id).map(String::from).filter_map(|s| s.chars().next()).collect();
                let mut residue_sequence_numbers: Vec<u64> = pdb
                    .residues()
                    .map(Residue::serial_number)
                    .map(|n| n as u64)
                    .collect::<std::collections::HashSet<_>>()
                    .into_iter()
                    .collect();
                residue_sequence_numbers.sort_unstable();
                let mut residues_per_chain = Vec::new();
                for chain in pdb.chains() {
                    let residues_of_chain = chain
                        .residues()
                        .map(|r| Residueinfo {
                            seqnumber: r.serial_number() as u64,
                            insertioncode: r.insertion_code().unwrap_or("-").to_string(),
                        })
                        .collect();
                    residues_per_chain.push(Residueinfos {
                        chain: chain.id().to_string().chars().next().unwrap(),
                        residues: residues_of_chain,
                    });
                }
                let mut warnings_as_strings = Vec::new();
                for warning in warnings {
                    warnings_as_strings.push(format!("{:?}", warning))
                }
                Ok(Pdbinfo {
                    identifier: pdb.identifier,
                    chains,
                    residuesequencenumbers: residue_sequence_numbers,
                    residuesperchain: residues_per_chain,
                    warnings: warnings_as_strings,
                })
            }
            Err(errors) => {
                let mut errors_as_strings = Vec::new();
                for error in errors {
                    errors_as_strings.push(format!("{:?}", error))
                }
                Err(errors_as_strings)
            }
        }
    }
}

bindings::export!(Component with_types_in bindings);
