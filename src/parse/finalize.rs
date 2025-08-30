use address::IPAddress;

use crate::parse::path_plus::PathPlus;
use crate::parse::pre_path::PrePath;
use crate::parse::Error;
use crate::parse::Error::UrlTooLong;
use crate::WebUrl;

/// Finalizes the web-based URL from the pre-path and path-plus parts.
///
/// # Safety
/// The given URL must match the given pre-path and path-plus parts.
pub unsafe fn finalize_web_url(
    url: String,
    pre_path: PrePath,
    path_plus: PathPlus,
) -> Result<WebUrl, (Error, String)> {
    if url.len() > (u32::MAX as usize) {
        return Err((UrlTooLong, url));
    }

    let mut url: String = url;
    pre_path.make_lowercase(url.as_mut_str());

    let scheme_len: u32 = pre_path.scheme_len as u32;
    let host_end: u32 = scheme_len + 3 + (pre_path.host_len as u32);
    let ip: Option<IPAddress> = pre_path.ip;
    let port: Option<u16> = pre_path.port;
    let port_end: u32 = host_end + (pre_path.port_len as u32);
    let path_end: u32 = port_end + (path_plus.path_len as u32);
    let query_end: u32 = path_end + (path_plus.query_len as u32);

    Ok(WebUrl::new(
        url, scheme_len, host_end, ip, port_end, port, path_end, query_end,
    ))
}
