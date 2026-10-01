use crate::ParseError;
use crate::ParseError::UserInfoNotSupported;

/// Checks that the `authority` has no user info.
///
/// The '@' char is invalid in both a domain name & an IPv6 literal, so an '@' char in the authority
/// always indicates user info.
///
/// # RFC 3986
/// The authority is `[ userinfo "@" ] host [ ":" port ]`. This library supports only the host & the
/// optional port. User info is rejected rather than silently discarded, since discarding it would
/// drop credentials & leave the caller with an unauthenticated URL & no indication why.
/// <https://www.rfc-editor.org/rfc/rfc3986#section-3.2.1>
pub(crate) fn check_no_user_info(authority: &str) -> Result<(), ParseError> {
    if authority.as_bytes().contains(&b'@') {
        Err(UserInfoNotSupported)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::ParseError;
    use crate::ParseError::UserInfoNotSupported;
    use crate::parse::check_no_user_info;

    #[test]
    fn no_user_info() {
        let test_cases: &[(&str, Result<(), ParseError>)] = &[
            // No user info.
            ("", Ok(())),
            ("host", Ok(())),
            ("host:80", Ok(())),
            ("[::1]:80", Ok(())),
            // User info in every form.
            ("user@host", Err(UserInfoNotSupported)),
            ("user:pass@host", Err(UserInfoNotSupported)),
            ("user:@host", Err(UserInfoNotSupported)),
            (":pass@host", Err(UserInfoNotSupported)),
            ("@host", Err(UserInfoNotSupported)),
            ("user:pass@host:8080", Err(UserInfoNotSupported)),
            ("a@b@host", Err(UserInfoNotSupported)),
            ("user@[::1]:80", Err(UserInfoNotSupported)),
        ];
        for (authority, expected) in test_cases {
            let result: Result<(), ParseError> = check_no_user_info(authority);
            assert_eq!(result, *expected, "authority={}", authority);
        }
    }
}
