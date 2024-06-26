#[allow(warnings)]
mod bindings;

use std::io::BufReader;
// TODO make less nested
use bindings::Guest;
use bindings::Pdb;
use pdbtbx::*;

struct Component;

impl Guest for Component {
    fn open_pdb_raw(
        content: String,
    ) -> Result<(bindings::Pdb, Vec<String>), Vec<String>> {
        match open_pdb_raw(
            BufReader::new(content.as_bytes()),
            Context::None,
            StrictnessLevel::Loose,
        ) {
            Ok((pdb, warnings)) => {
                let warnings = warnings.into_iter().map(|w| w.to_string()).collect();
                // TODO convert rust PDB to wit Pdb
                let mypdb = Pdb::new();
                Ok((mypdb, warnings))
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
