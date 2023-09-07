use crate::cli::args::{CLArgs, Operation};

use anyhow::Result;

// for processing a file and spitting out words within it
pub fn main(args: CLArgs) -> Result<()> {
    info!("{:?} operation is being performed. The file found at {:?} will be transcribed.", 
            &args.operation, 
            match &args.operation { 
                Operation::File { file_path } => file_path, 
                _ => {
                    error!("Non-file operation inside file operation branch! Args: {:?}", args);
                    panic!()
                },
            }
        );
        
    todo!();
}