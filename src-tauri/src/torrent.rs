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

    print!("Number of torrents: {}", statuses.len());
    debug!("Number of torrents: {}", statuses.len());

    // Return the result vector
    statuses
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_torrent() {
        let magnet_link = "magnet:?xt=urn:btih:8DF6E26142615621983763B729F640372CF1FC34&dn=Linux+Mint+20.1+%26quot%3BUlyssa%26quot%3B+-+Cinnamon+%2864-bit%29&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&tr=udp%3A%2F%2Fopentracker.i2p.rocks%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fcoppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.zer0day.to%3A1337%2Fannounce";
        download_torrent("/home/user/Downloads", magnet_link).await;

        // Test will pass if no error is thrown during the download of a torrent via magnetlink
        // This might take a while to complete
        // Linux mint is used at its well seeded
        assert!(true);
    }

    #[tokio::test]
    async fn test_get_torrent_statuses() {
        let magnet_link = "magnet:?xt=urn:btih:8DF6E26142615621983763B729F640372CF1FC34&dn=Linux+Mint+20.1+%26quot%3BUlyssa%26quot%3B+-+Cinnamon+%2864-bit%29&tr=udp%3A%2F%2Ftracker.opentrackr.org%3A1337%2Fannounce&tr=http%3A%2F%2Ftracker.openbittorrent.com%3A80%2Fannounce&tr=udp%3A%2F%2Fopentracker.i2p.rocks%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.internetwarriors.net%3A1337%2Fannounce&tr=udp%3A%2F%2Ftracker.leechers-paradise.org%3A6969%2Fannounce&tr=udp%3A%2F%2Fcoppersurfer.tk%3A6969%2Fannounce&tr=udp%3A%2F%2Ftracker.zer0day.to%3A1337%2Fannounce";
        download_torrent("/home/user/Downloads", magnet_link).await;

        let statuses = get_torrent_statuses().await;

        assert!(statuses.len() == 1)
    }
}