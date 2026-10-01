use crate::WebUrl;
use crate::parse::{CanonicalHost, Parts, PrePath};
use address::IPAddress;

/// Finalizes the web-based URL from the `parts`.
///
/// The `url` must already be normalized apart from the letter case, which is normalized here.
///
/// # Safety
/// The given URL must match the given `parts`, which must be from `parse_parts`.
pub(crate) unsafe fn finalize_web_url(mut url: String, parts: Parts) -> WebUrl {
    let pre_path: PrePath = parts.pre_path;
    pre_path.make_lowercase(url.as_mut_str());

    let ip: Option<IPAddress> = pre_path.ip.as_ref().map(CanonicalHost::ip);
    unsafe { WebUrl::new_unchecked(url, parts.offsets(), ip, pre_path.port) }
}
