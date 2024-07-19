/*
This file will be used to manage the torrents.
It will allow to add torrents, to get the status of all torrents and to start the torrents at the beginning of the program.
*/

use librqbit::ManagedTorrent;
use librqbit::{AddTorrent, AddTorrentOptions, AddTorrentResponse, Session};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

// Define the global state for torrent handles
static HANDLES: Lazy<Arc<Mutex<Vec<Arc<ManagedTorrent>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

/// Function to download a torrent from a magnet link.
/// It will take the directory where the torrent will be downloaded and the magnet link.
/// It will return a Result<(), anyhow::Error> to handle potential errors.
/// 
/// # Arguments
/// - directory: &str (directory where the torrent will be downloaded)
/// - magnet_link: &str (magnet link of the torrent)
/// 
/// # Example
/// ```rust
/// 
/// download_torrent("downloads", "magnet:?xt=urn:btih:...");
///    
/// ```
/// 
pub async fn download_torrent(directory: &str, magnet_link: &str) {
    // Create the session
    let session = match Session::new(directory.into()).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error creating session: {:?}", e);
            return;
        }
    };

    // Add the torrent to the session
    let handle = match session
        .add_torrent(
            AddTorrent::from_url(magnet_link),
            Some(AddTorrentOptions {
                // Allow writing on top of existing files.
                overwrite: true,
                ..Default::default()
            }),
        )
        .await
    {
        Ok(response) => match response {
            AddTorrentResponse::Added(_, handle) => handle,
            // For a brand new session other variants won't happen.
            _ => unreachable!(),
        },
        Err(e) => {
            eprintln!("error adding torrent: {:?}", e);
            return;
        }
    };

    // Clone the handle
    let handle_clone: Arc<ManagedTorrent> = handle.clone();

    // Safely add the handle to the global list
    let mut handles = match HANDLES.lock() {
        Ok(h) => h,
        Err(e) => {
            eprintln!("error locking handles: {:?}", e);
            return;
        }
    };
    handles.push(handle_clone);

    // Wait for the handle to finish
    if let Err(e) = handle.wait_until_completed().await {
        eprintln!("error waiting for torrent to complete: {:?}", e);
    }

    // Delete the session implicitly by dropping it
    drop(session);
}

/// Function to get the status of all the torrents.
/// It will return a vector with the status of all the torrents as strings.
/// The status will be a JSON object with the following structure:
pub async fn get_torrent_statuses() -> Vec<std::string::String> {
    // Get the global list of handles
    let handles = HANDLES.lock().unwrap();

    // Create the result vector
    let mut statuses = Vec::new();

    // Iterate over all the handles
    for handle in handles.iter() {
        // Get the status of the handle
        let status = handle.stats();

        // Push the status to the result vector
        statuses.push(status.to_string());
    }

    // Return the result vector
    statuses

}

