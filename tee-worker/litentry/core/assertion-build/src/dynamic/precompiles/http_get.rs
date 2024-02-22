use crate::*;
use itc_rest_client::http_client::SendHttpRequest;

http_get_precompile!(HttpGetBoolPrecompile, Bool, as_bool);
http_get_precompile!(HttpGetI64Precompile, Uint, as_i64);
