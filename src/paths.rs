use crate::{ZKError, ZKResult};

pub(crate) fn validate_path(client_path: &str) -> ZKResult<()> {
    if client_path.is_empty() {
        return Err(ZKError::PathError(client_path.into(), "Path can't be empty".into()));
    }
    if !client_path.starts_with('/') {
        return Err(ZKError::PathError(client_path.into(), "Path must start with '/'".into()));
    }
    if client_path == "/" {
        return Ok(());
    }
    if client_path.ends_with('/') {
        return Err(ZKError::PathError(client_path.into(), "Path must not end with '/'".into()));
    }

    // TODO 具体的非法字符
    // if (c > 'U+0000' && c <= '\u001f'
    //     || c >= '\u007f' && c <= '\u009F'
    //     || c >= '\ud800' && c <= '\uf8ff'
    //     || c >= '\ufff0' && c <= '\uffff') {
    //     reason = "invalid character @" + i;
    //     break;

    Ok(())
}


#[cfg(test)]
mod test {
    use crate::ZKError;
    use crate::paths::validate_path;

    #[test]
    fn test_validate_path() {
        if let Err(e) = validate_path("") {
            println!("{}", e.to_string());
        }
        if let Err(e) = validate_path("123") {
            println!("{}", e.to_string());
        }
        if let Err(e) = validate_path("/123/") {
            println!("{}", e.to_string());
        }
    }
}