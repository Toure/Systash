/* File manager will be responsible for compressing, decompressing, snapshots,
and validation of backups.
*/
struct FileManager {
    compression: String,
    decompression: String,
    path: String,
    snapshot: String,
    validate: String
}

mod graphene {

    pub mod compress {

    }

    pub mod decompress {

    }

    pub mod snapshot {

    }

    pub mod delete {

    }

    pub mod undelete {
        
    }

}
