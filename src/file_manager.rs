/* File manager will be responsible for compressing, decompressing, snapshots,
and validation of backups.
*/

struct FileManager {
    compression: String,
    decompression: String,
    snapshot: String,
    validate: String
}

