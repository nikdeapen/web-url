use crate::Error;
use crate::Error::UserInfoNotSupported;
use crate::parse::is_authority_end;

/// Checks that the authority at the prefix of `s` has no user info.
///
/// The '@' char is invalid in both a domain name & an IPv6 literal, so an '@' char in the authority
/// always indicates user info.
///
/// # RFC 3986
/// The authority is `[ userinfo "@" ] host [ ":" port ]`. This library supports only the host & the
/// optional port. User info is rejected rather than silently discarded, since discarding it would
/// drop credentials & leave the caller with an unauthenticated URL & no indication why.
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.2.1>
pub fn check_no_user_info(s: &str) -> Result<(), Error> {
    let end: usize = s
        .as_bytes()
        .iter()
        .position(|c| is_authority_end(*c))
        .unwrap_or(s.len());

    if s.as_bytes()[..end].contains(&b'@') {
        Err(UserInfoNotSupported)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::Error;
    use crate::Error::UserInfoNotSupported;
    use crate::parse::check_no_user_info;

    #[test]
    fn fn_check_no_user_info() {
        let test_cases: &[(&str, Result<(), Error>)] = &[
            // No user info.
            ("", Ok(())),
            ("host", Ok(())),
            ("host/", Ok(())),
            ("host:80/path", Ok(())),
            ("[::1]:80/path", Ok(())),
            // The '@' char is only user info inside the authority.
            ("host/a@b", Ok(())),
            ("host/p?a@b", Ok(())),
            ("host/p#a@b", Ok(())),
            ("host?a@b", Ok(())),
            ("host#a@b", Ok(())),
            // User info in every form.
            ("user@host", Err(UserInfoNotSupported)),
            ("user:pass@host", Err(UserInfoNotSupported)),
            ("user:@host", Err(UserInfoNotSupported)),
            (":pass@host", Err(UserInfoNotSupported)),
            ("@host", Err(UserInfoNotSupported)),
            ("user@host/path", Err(UserInfoNotSupported)),
            ("user:pass@host:8080/path", Err(UserInfoNotSupported)),
            ("user@host?query", Err(UserInfoNotSupported)),
            ("user@host#frag", Err(UserInfoNotSupported)),
            ("a@b@host", Err(UserInfoNotSupported)),
            ("user@[::1]:80/p", Err(UserInfoNotSupported)),
        ];
        for (s, expected) in test_cases {
            let result: Result<(), Error> = check_no_user_info(s);
            assert_eq!(result, *expected, "s={}", s);
        }
    }
}
