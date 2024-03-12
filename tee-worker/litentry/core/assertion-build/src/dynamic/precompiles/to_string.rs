use crate::dynamic::precompiles::macros::prepare_custom_failure;
use crate::dynamic::precompiles::PrecompileResult;

pub fn bytes_to_string(input: Vec<u8>) -> PrecompileResult {
    let decoded = ethabi::decode(&[ethabi::ParamType::Bytes], &input).map_err(|e| {
        prepare_custom_failure(format!(
            "Could not decode bytes {:?}, reason: {:?}", input, e
        ))
    })?;

    // safe to unwrap
    let bytes = decoded.get(0).unwrap().clone().into_bytes().unwrap();



    let hex_encoded = format!("0x{}", hex::encode(&bytes));
    let encoded = ethabi::encode(&[ethabi::Token::String(hex_encoded)]);
    Ok(evm::executor::stack::PrecompileOutput {
        exit_status: evm::ExitSucceed::Returned,
        output: encoded,
    })
}


#[cfg(test)]
pub mod test {
    use crate::dynamic::precompiles::to_string::bytes_to_string;
    use evm::ExitSucceed;


    #[test]
    pub fn test_bytes_to_string() {
        // given
        let bytes = [1,2,3,4];
        let encoded = ethabi::encode(&[ethabi::Token::Bytes(bytes.to_vec())]);

        // when
        let result = bytes_to_string(encoded).unwrap();

        //then
        assert!(matches!(result.exit_status, ExitSucceed::Returned));
        assert_eq!(
            ethabi::encode(&[ethabi::Token::String("0x01020304".to_string())]),
            result.output
        )

    }


}