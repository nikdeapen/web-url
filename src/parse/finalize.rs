use crate::parse::{PathPlus, PrePath};
use crate::Error;
use crate::Error::UrlTooLong;
use crate::WebUrl;
use address::IPAddress;

/// Finalizes the web-based URL from the pre-path and path-plus parts.
///
/// The `url` must already be normalized apart from the letter case, which is normalized here.
///
/// # Safety
/// The given URL must match the given pre-path and path-plus parts.
pub unsafe fn finalize_web_url(
    mut url: String,
    pre_path: PrePath,
    path_plus: PathPlus,
) -> Result<WebUrl, (Error, String)> {
    if url.len() > WebUrl::MAX_LEN {
        return Err((UrlTooLong, url));
    }

    pre_path.make_lowercase(url.as_mut_str());

    let scheme_len: u32 = pre_path.scheme_len as u32;

    // The canonical host end is used since `url` is the normalized URL, in which an IP address host
    // may be shorter or longer than it was in the parsed URL.
    let host_end: u32 = pre_path.canonical_host_end() as u32;
    let ip: Option<IPAddress> = pre_path.ip;
    let port: Option<u16> = pre_path.port;

    // The canonical length is used since `url` is the normalized URL, in which the port may be
    // shorter than it was in the parsed URL.
    let port_end: u32 = pre_path.canonical_len() as u32;
    let path_end: u32 = port_end + (path_plus.canonical_path_len as u32);
    let query_end: u32 = path_end + (path_plus.query_len as u32);

    Ok(unsafe { WebUrl::new_unchecked(url, scheme_len, host_end, ip, port_end, port, path_end, query_end) })
}
