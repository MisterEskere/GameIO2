/*
This file will be used to manage the torrents.
It will allow to add torrents, to get the status of all torrents and to start the torrents at the beginning of the program.
*/

use librqbit::ManagedTorrent;
use librqbit::{AddTorrent, AddTorrentOptions, AddTorrentResponse, Session};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use log::{trace, debug, error};

// Define the global list of handles, this will be used to store all the active torrents
static HANDLES: Lazy<Arc<Mutex<Vec<Arc<ManagedTorrent>>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::new())));

/// Function to download a torrent from a magnet link.
/// It will be recalled when the user clicks on the torrent to download from the TODO page.
/// Flow:
/// * `Create a new session
/// * `Add the torrent to the session\
/// * `Add the handle to the HANDLES list
/// * `Wait for the handle to finish
/// * `Delete thje session
/// 
/// # Arguments
/// * `directory` - A string slice that holds the directory where the torrent will be downloaded.
/// * `magnet_link` - A string slice that holds the magnet link of the torrent.
///
pub async fn download_torrent(directory: &str, magnet_link: &str) {

    print!("Downloading torrent... {}", magnet_link);
    // Create the session
    trace!("Creating session");
    debug!("Directory: {}", directory);
    let session = match Session::new(directory.into()).await {
        Ok(s) => s,
        Err(e) => {
            error!("error creating session: {:?}", e);
            return;
        }
    };
    trace!("Session created");

    // Add the torrent to the session
    trace!("Adding torrent");
    trace!("Magnet link: {}", magnet_link);
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
            _ => unreachable!(),
        },
        Err(e) => {
            error!("error adding torrent: {:?}", e);
            return;
        }
    };
    trace!("Torrent added");

    // Add the handle to the global list
    trace!("Adding handle to global list");
    let handle_clone: Arc<ManagedTorrent> = handle.clone();
    let mut handles = match HANDLES.lock() {
        Ok(h) => h,
        Err(e) => {
            error!("error locking global handles: {:?}", e);
            return;
        }
    };
    handles.push(handle_clone);
    trace!("Handle added to global list");

    // Wait for the handle to finish
    trace!("Waiting for torrent to complete");
    if let Err(e) = handle.wait_until_completed().await {
        error!("error waiting for torrent to complete: {:?}", e);
    }
    trace!("Torrent completed");

    // Delete the session implicitly by dropping it
    drop(session);
}

/// Function to get the status of all the torrents.
/// It will be recalled when the user clicks on the TODO page.
/// 
/// Flow:
///* `Get the global list of handles
///* `Create the result vector
///* `Iterate over all the handles
///* `Get the status of the handle
///* `Push the status to the result vector
/// 
/// # Returns
/// A vector of strings that holds the status of all the torrents.
/// 
pub async fn get_torrent_statuses() -> Vec<std::string::String> {

    // Get the global list of handles
    trace!("Getting list of handles");
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

    debug!("Number of torrents: {}", statuses.len());

    // Return the result vector
    statuses
}

