use crate::constants::Error;
use crate::{ZKError, ZKResult};

pub(crate) fn validate_path(client_path: &str) -> ZKResult<()> {
    if client_path.len() == 0 {
        return Err(ZKError(Error::BadArguments, "Path can't be empty"));
    }
    if !client_path.starts_with("/") {
        return Err(ZKError(Error::BadArguments, "Path must start with '/'"));
    }
    if client_path == "/" {
        return Ok(());
    }
    if client_path.ends_with("/") {
        return Err(ZKError(Error::BadArguments, "Path must not end with '/'"));
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
