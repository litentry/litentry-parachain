#![allow(dead_code, unused_imports, const_item_mutation)]

use super::*;
use codec::Decode;
use frame_support::assert_err;
use hex_literal::hex;
// reproduce with "integritee_service dump_ra"
const TEST1_CERT: &[u8] = include_bytes!("../test/test_ra_cert_MRSIGNER1_MRENCLAVE1.der");
const TEST2_CERT: &[u8] = include_bytes!("../test/test_ra_cert_MRSIGNER2_MRENCLAVE2.der");
const TEST3_CERT: &[u8] = include_bytes!("../test/test_ra_cert_MRSIGNER3_MRENCLAVE2.der");
const TEST4_CERT: &[u8] = include_bytes!("../test/ra_dump_cert_TEST4.der");
const TEST5_CERT: &[u8] = include_bytes!("../test/ra_dump_cert_TEST5.der");
const TEST6_CERT: &[u8] = include_bytes!("../test/ra_dump_cert_TEST6.der");
const TEST7_CERT: &[u8] = include_bytes!("../test/ra_dump_cert_TEST7.der");
const TEST8_CERT: &[u8] = include_bytes!("../test/ra_dump_cert_TEST8_PRODUCTION.der");
const TEST9_CERT: &[u8] = include_bytes!("../test/ra_dump_cert_TEST9_enclave_add.der");

const TEST1_SIGNER_ATTN: &[u8] =
	include_bytes!("../test/test_ra_signer_attn_MRSIGNER1_MRENCLAVE1.bin");
const TEST2_SIGNER_ATTN: &[u8] =
	include_bytes!("../test/test_ra_signer_attn_MRSIGNER2_MRENCLAVE2.bin");
const TEST3_SIGNER_ATTN: &[u8] =
	include_bytes!("../test/test_ra_signer_attn_MRSIGNER3_MRENCLAVE2.bin");

// reproduce with "integritee_service signing-key"
const TEST1_SIGNER_PUB: &[u8] =
	include_bytes!("../test/test_ra_signer_pubkey_MRSIGNER1_MRENCLAVE1.bin");
const TEST2_SIGNER_PUB: &[u8] =
	include_bytes!("../test/test_ra_signer_pubkey_MRSIGNER2_MRENCLAVE2.bin");
const TEST3_SIGNER_PUB: &[u8] =
	include_bytes!("../test/test_ra_signer_pubkey_MRSIGNER3_MRENCLAVE2.bin");
const TEST4_SIGNER_PUB: &[u8] = include_bytes!("../test/enclave-signing-pubkey-TEST4.bin");
// equal to TEST4!
const TEST5_SIGNER_PUB: &[u8] = include_bytes!("../test/enclave-signing-pubkey-TEST5.bin");
const TEST6_SIGNER_PUB: &[u8] = include_bytes!("../test/enclave-signing-pubkey-TEST6.bin");
const TEST7_SIGNER_PUB: &[u8] = include_bytes!("../test/enclave-signing-pubkey-TEST7.bin");

// reproduce with "make mrenclave" in worker repo root
const TEST1_MRENCLAVE: &[u8] = &[
	62, 252, 187, 232, 60, 135, 108, 204, 87, 78, 35, 169, 241, 237, 106, 217, 251, 241, 99, 189,
	138, 157, 86, 136, 77, 91, 93, 23, 192, 104, 140, 167,
];
const TEST2_MRENCLAVE: &[u8] = &[
	4, 190, 230, 132, 211, 129, 59, 237, 101, 78, 55, 174, 144, 177, 91, 134, 1, 240, 27, 174, 81,
	139, 8, 22, 32, 241, 228, 103, 189, 43, 44, 102,
];
const TEST3_MRENCLAVE: &[u8] = &[
	4, 190, 230, 132, 211, 129, 59, 237, 101, 78, 55, 174, 144, 177, 91, 134, 1, 240, 27, 174, 81,
	139, 8, 22, 32, 241, 228, 103, 189, 43, 44, 102,
];

// MRSIGNER is 83d719e77deaca1470f6baf62a4d774303c899db69020f9c70ee1dfc08c7ce9e
const TEST4_MRENCLAVE: [u8; 32] =
	hex!("7a3454ec8f42e265cb5be7dfd111e1d95ac6076ed82a0948b2e2a45cf17b62a0");
const TEST5_MRENCLAVE: [u8; 32] =
	hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d1");
// equal to TEST5!
const TEST6_MRENCLAVE: [u8; 32] =
	hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d1");
// equal to TEST6!
const TEST7_MRENCLAVE: [u8; 32] =
	hex!("f4dedfc9e5fcc48443332bc9b23161c34a3c3f5a692eaffdb228db27b704d9d1");

// production mode
// MRSIGNER is 117f95f65f06afb5764b572156b8b525c6230db7d6b1c94e8ebdb7fba068f4e8
const TEST8_MRENCLAVE: [u8; 32] =
	hex!("bcf66abfc6b3ef259e9ecfe4cf8df667a7f5a546525dee16822741b38f6e6050");

// unix epoch. must be later than this
const TEST1_TIMESTAMP: i64 = 1580587262i64;
const TEST2_TIMESTAMP: i64 = 1581259412i64;
const TEST3_TIMESTAMP: i64 = 1581259975i64;

//const CERT: &[u8] =
// b"0\x82\x0c\x8c0\x82\x0c2\xa0\x03\x02\x01\x02\x02\x01\x010\n\x06\x08*\x86H\xce=\x04\x03\x020\
// x121\x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0\x1e\x17\r190617124609Z\x17\r190915124609Z0\x121\
// x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\
// x03\x01\x07\x03B\0\x04RT\x16\x16
// \xef_\xd8\xe7\xc3\xb7\x03\x1d\xd6:\x1fF\xe3\xf2b!\xa9/\x8b\xd4\x82\x8f\xd1\xff[\x9c\x97\xbc\xf27\
// xb8,L\x8a\x01\xb0r;;\xa9\x83\xdc\x86\x9f\x1d%y\xf4;I\xe4Y\xc80'$K[\xd6\xa3\x82\x0bw0\x82\x0bs0\
// x82\x0bo\x06\t`\x86H\x01\x86\xf8B\x01\r\x04\x82\x0b`{\"id\":\"
// 117077750682263877593646412006783680848\",\"timestamp\":\"2019-06-17T12:46:04.002066\",\"version\
// ":3,\"isvEnclaveQuoteStatus\":\"GROUP_OUT_OF_DATE\",\"platformInfoBlob\":\"
// 1502006504000900000909020401800000000000000000000008000009000000020000000000000B401A355B313FC939B4F48A54349C914A32A3AE2C4871BFABF22E960C55635869FC66293A3D9B2D58ED96CA620B65D669A444C80291314EF691E896F664317CF80C\
// ",\"isvEnclaveQuoteBody\":\"AgAAAEALAAAIAAcAAAAAAOE6wgoHKsZsnVWSrsWX9kky0kWt9K4xcan0fQ996Ct+CAj//
// wGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABwAAAAAAAAAHAAAAAAAAAFJJYIbPVot9NzRCjW2z9+k+9K8BsHQKzVMEHOR14hNbAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACD1xnnferKFHD2uvYqTXdDA8iZ22kCD5xw7h38CMfOngAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSVBYWIO9f2OfDtwMd1jofRuPyYiGpL4vUgo/
// R/1ucl7zyN7gsTIoBsHI7O6mD3IafHSV59DtJ5FnIMCckS1vW\"}|EbPFH/ThUaS/
// dMZoDKC5EgmdUXUORFtQzF49Umi1P55oeESreJaUvmA0sg/
// ATSTn5t2e+e6ZoBQIUbLHjcWLMLzK4pJJUeHhok7EfVgoQ378i+eGR9v7ICNDGX7a1rroOe0s1OKxwo/
// 0hid2KWvtAUBvf1BDkqlHy025IOiXWhXFLkb/
// qQwUZDWzrV4dooMfX5hfqJPi1q9s18SsdLPmhrGBheh9keazeCR9hiLhRO9TbnVgR9zJk43SPXW+pHkbNigW+2STpVAi5ugWaSwBOdK11ZjaEU1paVIpxQnlW1D6dj1Zc3LibMH+ly9ZGrbYtuJks4eRnjPhroPXxlJWpQ==|MIIEoTCCAwmgAwIBAgIJANEHdl0yo7CWMA0GCSqGSIb3DQEBCwUAMH4xCzAJBgNVBAYTAlVTMQswCQYDVQQIDAJDQTEUMBIGA1UEBwwLU2FudGEgQ2xhcmExGjAYBgNVBAoMEUludGVsIENvcnBvcmF0aW9uMTAwLgYDVQQDDCdJbnRlbCBTR1ggQXR0ZXN0YXRpb24gUmVwb3J0IFNpZ25pbmcgQ0EwHhcNMTYxMTIyMDkzNjU4WhcNMjYxMTIwMDkzNjU4WjB7MQswCQYDVQQGEwJVUzELMAkGA1UECAwCQ0ExFDASBgNVBAcMC1NhbnRhIENsYXJhMRowGAYDVQQKDBFJbnRlbCBDb3Jwb3JhdGlvbjEtMCsGA1UEAwwkSW50ZWwgU0dYIEF0dGVzdGF0aW9uIFJlcG9ydCBTaWduaW5nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAqXot4OZuphR8nudFrAFiaGxxkgma/
// Es/BA+tbeCTUR106AL1ENcWA4FX3K+E9BBL0/7X5rj5nIgX/R/1ubhkKWw9gfqPG3KeAtIdcv/
// uTO1yXv50vqaPvE1CRChvzdS/ZEBqQ5oVvLTPZ3VEicQjlytKgN9cLnxbwtuvLUK7eyRPfJW/
// ksddOzP8VBBniolYnRCD2jrMRZ8nBM2ZWYwnXnwYeOAHV+W9tOhAImwRwKF/95yAsVwd21ryHMJBcGH70qLagZ7Ttyt++qO/
// 6+KAXJuKwZqjRlEtSEz8gZQeFfVYgcwSfo96oSMAzVr7V0L6HSDLRnpb6xxmbPdqNol4tQIDAQABo4GkMIGhMB8GA1UdIwQYMBaAFHhDe3amfrzQr35CN+s1fDuHAVE8MA4GA1UdDwEB/
// wQEAwIGwDAMBgNVHRMBAf8EAjAAMGAGA1UdHwRZMFcwVaBToFGGT2h0dHA6Ly90cnVzdGVkc2VydmljZXMuaW50ZWwuY29tL2NvbnRlbnQvQ1JML1NHWC9BdHRlc3RhdGlvblJlcG9ydFNpZ25pbmdDQS5jcmwwDQYJKoZIhvcNAQELBQADggGBAGcIthtcK9IVRz4rRq+ZKE+7k50/
// OxUsmW8aavOzKb0iCx07YQ9rzi5nU73tME2yGRLzhSViFs/
// LpFa9lpQL6JL1aQwmDR74TxYGBAIi5f4I5TJoCCEqRHz91kpG6Uvyn2tLmnIdJbPE4vYvWLrtXXfFBSSPD4Afn7+3/
// XUggAlc7oCTizOfbbtOFlYA4g5KcYgS1J2ZAeMQqbUdZseZCcaZZZn65tdqee8UXZlDvx0+NdO0LR+5pFy+juM0wWbu59MvzcmTXbjsi7HY6zd53Yq5K244fwFHRQ8eOB0IWB+4PfM7FeAApZvlfqlKOlLcZL2uyVmzRkyR5yW72uo9mehX44CiPJ2fse9Y6eQtcfEhMPkmHXI01sN+KwPbpA39+xOsStjhP9N1Y1a2tQAVo+yVgLgV2Hws73Fc0o3wC78qPEA+v2aRs/
// Be3ZFDgDyghc/1fgU+7C+P6kbqd4poyb6IW8KCJbxfMJvkordNOgOUUxndPHEi/tb/U7uLjLOgPA==0\n\x06\x08*\x86H\
// xce=\x04\x03\x02\x03H\00E\x02!\0\xae6\x06\t@Sy\x8f\x8ec\x9d\xdci^Ex*\x92}\xdcG\x15A\x97\xd7\xd7\
// xd1\xccx\xe0\x1e\x08\x02
// \x15Q\xa0BT\xde'~\xec\xbd\x027\xd3\xd8\x83\xf7\xe6Z\xc5H\xb4D\xf7\xe2\r\xa7\xe4^f\x10\x85p";
const CERT_FAKE_QUOTE_STATUS: &[u8] = b"0\x82\x0c\x8c0\x82\x0c2\xa0\x03\x02\x01\x02\x02\x01\x010\n\x06\x08*\x86H\xce=\x04\x03\x020\x121\x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0\x1e\x17\r190617124609Z\x17\r190915124609Z0\x121\x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\0\x04RT\x16\x16 \xef_\xd8\xe7\xc3\xb7\x03\x1d\xd6:\x1fF\xe3\xf2b!\xa9/\x8b\xd4\x82\x8f\xd1\xff[\x9c\x97\xbc\xf27\xb8,L\x8a\x01\xb0r;;\xa9\x83\xdc\x86\x9f\x1d%y\xf4;I\xe4Y\xc80'$K[\xd6\xa3\x82\x0bw0\x82\x0bs0\x82\x0bo\x06\t`\x86H\x01\x86\xf8B\x01\r\x04\x82\x0b`{\"id\":\"117077750682263877593646412006783680848\",\"timestamp\":\"2019-06-17T12:46:04.002066\",\"version\":3,\"isvEnclaveQuoteStatus\":\"OK\",\"platformInfoBlob\":\"1602006504000900000909020401800000000000000000000008000009000000020000000000000B401A355B313FC939B4F48A54349C914A32A3AE2C4871BFABF22E960C55635869FC66293A3D9B2D58ED96CA620B65D669A444C80291314EF691E896F664317CF80C\",\"isvEnclaveQuoteBody\":\"AgAAAEALAAAIAAcAAAAAAOE6wgoHKsZsnVWSrsWX9kky0kWt9K4xcan0fQ996Ct+CAj//wGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABwAAAAAAAAAHAAAAAAAAAFJJYIbPVot9NzRCjW2z9+k+9K8BsHQKzVMEHOR14hNbAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACD1xnnferKFHD2uvYqTXdDA8iZ22kCD5xw7h38CMfOngAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSVBYWIO9f2OfDtwMd1jofRuPyYiGpL4vUgo/R/1ucl7zyN7gsTIoBsHI7O6mD3IafHSV59DtJ5FnIMCckS1vW\"}|EbPFH/ThUaS/dMZoDKC5EgmdUXUORFtQzF49Umi1P55oeESreJaUvmA0sg/ATSTn5t2e+e6ZoBQIUbLHjcWLMLzK4pJJUeHhok7EfVgoQ378i+eGR9v7ICNDGX7a1rroOe0s1OKxwo/0hid2KWvtAUBvf1BDkqlHy025IOiXWhXFLkb/qQwUZDWzrV4dooMfX5hfqJPi1q9s18SsdLPmhrGBheh9keazeCR9hiLhRO9TbnVgR9zJk43SPXW+pHkbNigW+2STpVAi5ugWaSwBOdK11ZjaEU1paVIpxQnlW1D6dj1Zc3LibMH+ly9ZGrbYtuJks4eRnjPhroPXxlJWpQ==|MIIEoTCCAwmgAwIBAgIJANEHdl0yo7CWMA0GCSqGSIb3DQEBCwUAMH4xCzAJBgNVBAYTAlVTMQswCQYDVQQIDAJDQTEUMBIGA1UEBwwLU2FudGEgQ2xhcmExGjAYBgNVBAoMEUludGVsIENvcnBvcmF0aW9uMTAwLgYDVQQDDCdJbnRlbCBTR1ggQXR0ZXN0YXRpb24gUmVwb3J0IFNpZ25pbmcgQ0EwHhcNMTYxMTIyMDkzNjU4WhcNMjYxMTIwMDkzNjU4WjB7MQswCQYDVQQGEwJVUzELMAkGA1UECAwCQ0ExFDASBgNVBAcMC1NhbnRhIENsYXJhMRowGAYDVQQKDBFJbnRlbCBDb3Jwb3JhdGlvbjEtMCsGA1UEAwwkSW50ZWwgU0dYIEF0dGVzdGF0aW9uIFJlcG9ydCBTaWduaW5nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAqXot4OZuphR8nudFrAFiaGxxkgma/Es/BA+tbeCTUR106AL1ENcWA4FX3K+E9BBL0/7X5rj5nIgX/R/1ubhkKWw9gfqPG3KeAtIdcv/uTO1yXv50vqaPvE1CRChvzdS/ZEBqQ5oVvLTPZ3VEicQjlytKgN9cLnxbwtuvLUK7eyRPfJW/ksddOzP8VBBniolYnRCD2jrMRZ8nBM2ZWYwnXnwYeOAHV+W9tOhAImwRwKF/95yAsVwd21ryHMJBcGH70qLagZ7Ttyt++qO/6+KAXJuKwZqjRlEtSEz8gZQeFfVYgcwSfo96oSMAzVr7V0L6HSDLRnpb6xxmbPdqNol4tQIDAQABo4GkMIGhMB8GA1UdIwQYMBaAFHhDe3amfrzQr35CN+s1fDuHAVE8MA4GA1UdDwEB/wQEAwIGwDAMBgNVHRMBAf8EAjAAMGAGA1UdHwRZMFcwVaBToFGGT2h0dHA6Ly90cnVzdGVkc2VydmljZXMuaW50ZWwuY29tL2NvbnRlbnQvQ1JML1NHWC9BdHRlc3RhdGlvblJlcG9ydFNpZ25pbmdDQS5jcmwwDQYJKoZIhvcNAQELBQADggGBAGcIthtcK9IVRz4rRq+ZKE+7k50/OxUsmW8aavOzKb0iCx07YQ9rzi5nU73tME2yGRLzhSViFs/LpFa9lpQL6JL1aQwmDR74TxYGBAIi5f4I5TJoCCEqRHz91kpG6Uvyn2tLmnIdJbPE4vYvWLrtXXfFBSSPD4Afn7+3/XUggAlc7oCTizOfbbtOFlYA4g5KcYgS1J2ZAeMQqbUdZseZCcaZZZn65tdqee8UXZlDvx0+NdO0LR+5pFy+juM0wWbu59MvzcmTXbjsi7HY6zd53Yq5K244fwFHRQ8eOB0IWB+4PfM7FeAApZvlfqlKOlLcZL2uyVmzRkyR5yW72uo9mehX44CiPJ2fse9Y6eQtcfEhMPkmHXI01sN+KwPbpA39+xOsStjhP9N1Y1a2tQAVo+yVgLgV2Hws73Fc0o3wC78qPEA+v2aRs/Be3ZFDgDyghc/1fgU+7C+P6kbqd4poyb6IW8KCJbxfMJvkordNOgOUUxndPHEi/tb/U7uLjLOgPA==0\n\x06\x08*\x86H\xce=\x04\x03\x02\x03H\x000E\x02!\0\xae6\x06\t@Sy\x8f\x8ec\x9d\xdci^Ex*\x92}\xdcG\x15A\x97\xd7\xd7\xd1\xccx\xe0\x1e\x08\x02 \x15Q\xa0BT\xde'~\xec\xbd\x027\xd3\xd8\x83\xf7\xe6Z\xc5H\xb4D\xf7\xe2\r\xa7\xe4^f\x10\x85p";
const CERT_WRONG_PLATFORM_BLOB: &[u8] = b"0\x82\x0c\x8c0\x82\x0c2\xa0\x03\x02\x01\x02\x02\x01\x010\n\x06\x08*\x86H\xce=\x04\x03\x020\x121\x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0\x1e\x17\r190617124609Z\x17\r190915124609Z0\x121\x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\0\x04RT\x16\x16 \xef_\xd8\xe7\xc3\xb7\x03\x1d\xd6:\x1fF\xe3\xf2b!\xa9/\x8b\xd4\x82\x8f\xd1\xff[\x9c\x97\xbc\xf27\xb8,L\x8a\x01\xb0r;;\xa9\x83\xdc\x86\x9f\x1d%y\xf4;I\xe4Y\xc80'$K[\xd6\xa3\x82\x0bw0\x82\x0bs0\x82\x0bo\x06\t`\x86H\x01\x86\xf8B\x01\r\x04\x82\x0b`{\"id\":\"117077750682263877593646412006783680848\",\"timestamp\":\"2019-06-17T12:46:04.002066\",\"version\":3,\"isvEnclaveQuoteStatus\":\"GROUP_OUT_OF_DATE\",\"platformInfoBlob\":\"1602006504000900000909020401800000000000000000000008000009000000020000000000000B401A355B313FC939B4F48A54349C914A32A3AE2C4871BFABF22E960C55635869FC66293A3D9B2D58ED96CA620B65D669A444C80291314EF691E896F664317CF80C\",\"isvEnclaveQuoteBody\":\"AgAAAEALAAAIAAcAAAAAAOE6wgoHKsZsnVWSrsWX9kky0kWt9K4xcan0fQ996Ct+CAj//wGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABwAAAAAAAAAHAAAAAAAAAFJJYIbPVot9NzRCjW2z9+k+9K8BsHQKzVMEHOR14hNbAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACD1xnnferKFHD2uvYqTXdDA8iZ22kCD5xw7h38CMfOngAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSVBYWIO9f2OfDtwMd1jofRuPyYiGpL4vUgo/R/1ucl7zyN7gsTIoBsHI7O6mD3IafHSV59DtJ5FnIMCckS1vW\"}|EbPFH/ThUaS/dMZoDKC5EgmdUXUORFtQzF49Umi1P55oeESreJaUvmA0sg/ATSTn5t2e+e6ZoBQIUbLHjcWLMLzK4pJJUeHhok7EfVgoQ378i+eGR9v7ICNDGX7a1rroOe0s1OKxwo/0hid2KWvtAUBvf1BDkqlHy025IOiXWhXFLkb/qQwUZDWzrV4dooMfX5hfqJPi1q9s18SsdLPmhrGBheh9keazeCR9hiLhRO9TbnVgR9zJk43SPXW+pHkbNigW+2STpVAi5ugWaSwBOdK11ZjaEU1paVIpxQnlW1D6dj1Zc3LibMH+ly9ZGrbYtuJks4eRnjPhroPXxlJWpQ==|MIIEoTCCAwmgAwIBAgIJANEHdl0yo7CWMA0GCSqGSIb3DQEBCwUAMH4xCzAJBgNVBAYTAlVTMQswCQYDVQQIDAJDQTEUMBIGA1UEBwwLU2FudGEgQ2xhcmExGjAYBgNVBAoMEUludGVsIENvcnBvcmF0aW9uMTAwLgYDVQQDDCdJbnRlbCBTR1ggQXR0ZXN0YXRpb24gUmVwb3J0IFNpZ25pbmcgQ0EwHhcNMTYxMTIyMDkzNjU4WhcNMjYxMTIwMDkzNjU4WjB7MQswCQYDVQQGEwJVUzELMAkGA1UECAwCQ0ExFDASBgNVBAcMC1NhbnRhIENsYXJhMRowGAYDVQQKDBFJbnRlbCBDb3Jwb3JhdGlvbjEtMCsGA1UEAwwkSW50ZWwgU0dYIEF0dGVzdGF0aW9uIFJlcG9ydCBTaWduaW5nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAqXot4OZuphR8nudFrAFiaGxxkgma/Es/BA+tbeCTUR106AL1ENcWA4FX3K+E9BBL0/7X5rj5nIgX/R/1ubhkKWw9gfqPG3KeAtIdcv/uTO1yXv50vqaPvE1CRChvzdS/ZEBqQ5oVvLTPZ3VEicQjlytKgN9cLnxbwtuvLUK7eyRPfJW/ksddOzP8VBBniolYnRCD2jrMRZ8nBM2ZWYwnXnwYeOAHV+W9tOhAImwRwKF/95yAsVwd21ryHMJBcGH70qLagZ7Ttyt++qO/6+KAXJuKwZqjRlEtSEz8gZQeFfVYgcwSfo96oSMAzVr7V0L6HSDLRnpb6xxmbPdqNol4tQIDAQABo4GkMIGhMB8GA1UdIwQYMBaAFHhDe3amfrzQr35CN+s1fDuHAVE8MA4GA1UdDwEB/wQEAwIGwDAMBgNVHRMBAf8EAjAAMGAGA1UdHwRZMFcwVaBToFGGT2h0dHA6Ly90cnVzdGVkc2VydmljZXMuaW50ZWwuY29tL2NvbnRlbnQvQ1JML1NHWC9BdHRlc3RhdGlvblJlcG9ydFNpZ25pbmdDQS5jcmwwDQYJKoZIhvcNAQELBQADggGBAGcIthtcK9IVRz4rRq+ZKE+7k50/OxUsmW8aavOzKb0iCx07YQ9rzi5nU73tME2yGRLzhSViFs/LpFa9lpQL6JL1aQwmDR74TxYGBAIi5f4I5TJoCCEqRHz91kpG6Uvyn2tLmnIdJbPE4vYvWLrtXXfFBSSPD4Afn7+3/XUggAlc7oCTizOfbbtOFlYA4g5KcYgS1J2ZAeMQqbUdZseZCcaZZZn65tdqee8UXZlDvx0+NdO0LR+5pFy+juM0wWbu59MvzcmTXbjsi7HY6zd53Yq5K244fwFHRQ8eOB0IWB+4PfM7FeAApZvlfqlKOlLcZL2uyVmzRkyR5yW72uo9mehX44CiPJ2fse9Y6eQtcfEhMPkmHXI01sN+KwPbpA39+xOsStjhP9N1Y1a2tQAVo+yVgLgV2Hws73Fc0o3wC78qPEA+v2aRs/Be3ZFDgDyghc/1fgU+7C+P6kbqd4poyb6IW8KCJbxfMJvkordNOgOUUxndPHEi/tb/U7uLjLOgPA==0\n\x06\x08*\x86H\xce=\x04\x03\x02\x03H\x000E\x02!\0\xae6\x06\t@Sy\x8f\x8ec\x9d\xdci^Ex*\x92}\xdcG\x15A\x97\xd7\xd7\xd1\xccx\xe0\x1e\x08\x02 \x15Q\xa0BT\xde'~\xec\xbd\x027\xd3\xd8\x83\xf7\xe6Z\xc5H\xb4D\xf7\xe2\r\xa7\xe4^f\x10\x85p";
const CERT_WRONG_SIG: &[u8] = b"0\x82\x0c\x8c0\x82\x0c2\xa0\x03\x02\x01\x02\x02\x01\x010\n\x06\x08*\x86H\xce=\x04\x03\x020\x121\x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0\x1e\x17\r190617124609Z\x17\r190915124609Z0\x121\x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\0\x04RT\x16\x16 \xef_\xd8\xe7\xc3\xb7\x03\x1d\xd6:\x1fF\xe3\xf2b!\xa9/\x8b\xd4\x82\x8f\xd1\xff[\x9c\x97\xbc\xf27\xb8,L\x8a\x01\xb0r;;\xa9\x83\xdc\x86\x9f\x1d%y\xf4;I\xe4Y\xc80'$K[\xd6\xa3\x82\x0bw0\x82\x0bs0\x82\x0bo\x06\t`\x86H\x01\x86\xf8B\x01\r\x04\x82\x0b`{\"id\":\"117077750682263877593646412006783680848\",\"timestamp\":\"2019-06-17T12:46:04.002066\",\"version\":3,\"isvEnclaveQuoteStatus\":\"GROUP_OUT_OF_DATE\",\"platformInfoBlob\":\"1602006504000900000909020401800000000000000000000008000009000000020000000000000B401A355B313FC939B4F48A54349C914A32A3AE2C4871BFABF22E960C55635869FC66293A3D9B2D58ED96CA620B65D669A444C80291314EF691E896F664317CF80C\",\"isvEnclaveQuoteBody\":\"AgAAAEALAAAIAAcAAAAAAOE6wgoHKsZsnVWSrsWX9kky0kWt9K4xcan0fQ996Ct+CAj//wGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABwAAAAAAAAAHAAAAAAAAAFJJYIbPVot9NzRCjW2z9+k+9K8BsHQKzVMEHOR14hNbAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAACD1xnnferKFHD2uvYqTXdDA8iZ22kCD5xw7h38CMfOngAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABSVBYWIO9f2OfDtwMd1jofRuPyYiGpL4vUgo/R/1ucl7zyN7gsTIoBsHI7O6mD3IafHSV59DtJ5FnIMCckS1vW\"}|EbPFH/ThUaS/dMZoDKC5EgmdUXUORFtQzF49Umi1P55oeESreJaUvmA0sg/ATSTn5t2e+e6ZoBQIUbLHjcWLMLzK4pJJUeHhok7EfVgoQ378i+eGR9v7ICNDGX7a1rroOe0s1OKxwo/0hid2KWvtAUBvf1BDkqlHy025IOiXWhXFLkb/qQwUZDWzrV4dooMfX5hfqJPi1q9s18SsdLPmhrGBheh9keazeCR9hiLhRO9TbnVgR9zJk43SPXW+pHkbNigW+2STpVAi5ugWaSwBOdK11ZjaEU1paVIpxQnlW1D6dj1Zc3LibMH+ly9ZGrbYtuJks4eRnjPhroPXxlJWpQ==|MIIEoTCCAwmgAwIBAgIJANEHdl0yo7CWMA0GCSqGSIb3DQEBCwUAMH4xCzAJBgNVBAYTAlVTMQswCQYDVQQIDAJDQTEUMBIGA1UEBwwLU2FudGEgQ2xhcmExGjAYBgNVBAoMEUludGVsIENvcnBvcmF0aW9uMTAwLgYDVQQDDCdJbnRlbCBTR1ggQXR0ZXN0YXRpb24gUmVwb3J0IFNpZ25pbmcgQ0EwHhcNMTYxMTIyMDkzNjU4WhcNMjYxMTIwMDkzNjU4WjB7MQswCQYDVQQGEwJVUzELMAkGA1UECAwCQ0ExFDASBgNVBAcMC1NhbnRhIENsYXJhMRowGAYDVQQKDBFJbnRlbCBDb3Jwb3JhdGlvbjEtMCsGA1UEAwwkSW50ZWwgU0dYIEF0dGVzdGF0aW9uIFJlcG9ydCBTaWduaW5nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAqXot4OZuphR8nudFrAFiaGxxkgma/Es/BA+tbeCTUR106AL1ENcWA4FX3K+E9BBL0/7X5rj5nIgX/R/1ubhkKWw9gfqPG3KeAtIdcv/uTO1yXv50vqaPvE1CRChvzdS/ZEBqQ5oVvLTPZ3VEicQjlytKgN9cLnxbwtuvLUK7eyRPfJW/ksddOzP8VBBniolYnRCD2jrMRZ8nBM2ZWYwnXnwYeOAHV+W9tOhAImwRwKF/95yAsVwd21ryHMJBcGH70qLagZ7Ttyt++qO/6+KAXJuKwZqjRlEtSEz8gZQeFfVYgcwSfo96oSMAzVr7V0L6HSDLRnpb6xxmbPdqNol4tQIDAQABo4GkMIGhMB8GA1UdIwQYMBaAFHhDe3amfrzQr35CN+s1fDuHAVE8MA4GA1UdDwEB/wQEAwIGwDAMBgNVHRMBAf8EAjAAMGAGA1UdHwRZMFcwVaBToFGGT2h0dHA6Ly90cnVzdGVkc2VydmljZXMuaW50ZWwuY29tL2NvbnRlbnQvQ1JML1NHWC9BdHRlc3RhdGlvblJlcG9ydFNpZ25pbmdDQS5jcmwwDQYJKoZIhvcNAQELBQADggGBAGcIthtcK9IVRz4rRq+ZKE+7k50/OxUsmW8aavOzKb0iCx07YQ9rzi5nU73tME2yGRLzhSViFs/LpFa9lpQL6JL1aQwmDR74TxYGBAIi5f4I5TJoCCEaRHz91kpG6Uvyn2tLmnIdJbPE4vYvWLrtXXfFBSSPD4Afn7+3/XUggAlc7oCTizOfbbtOFlYA4g5KcYgS1J2ZAeMQqbUdZseZCcaZZZn65tdqee8UXZlDvx0+NdO0LR+5pFy+juM0wWbu59MvzcmTXbjsi7HY6zd53Yq5K244fwFHRQ8eOB0IWB+4PfM7FeAApZvlfqlKOlLcZL2uyVmzRkyR5yW72uo9mehX44CiPJ2fse9Y6eQtcfEhMPkmHXI01sN+KwPbpA39+xOsStjhP9N1Y1a2tQAVo+yVgLgV2Hws73Fc0o3wC78qPEA+v2aRs/Be3ZFDgDyghc/1fgU+7C+P6kbqd4poyb6IW8KCJbxfMJvkordNOgOUUxndPHEi/tb/U7uLjLOgPA==0\n\x06\x08*\x86H\xce=\x04\x03\x02\x03H\x000E\x02!\0\xae6\x06\t@Sy\x8f\x8ec\x9d\xdci^Ex*\x92}\xdcG\x15A\x97\xd7\xd7\xd1\xccx\xe0\x1e\x08\x02 \x15Q\xa0BT\xde'~\xec\xbd\x027\xd3\xd8\x83\xf7\xe6Z\xc5H\xb4D\xf7\xe2\r\xa7\xe4^f\x10\x85p";
const CERT_TOO_SHORT1: &[u8] = b"0\x82\x0c\x8c0\x82\x0c2\xa0\x03\x02\x01\x02\x02\x01\x010\n\x06\x08*\x86H\xce=\x04\x03\x020\x121\x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0\x1e\x17\r190617124609Z\x17\r190915124609Z0\x121\x100\x0e\x06\x03U\x04\x03\x0c\x07MesaTEE0Y0\x13\x06\x07*\x86H\xce=\x02\x01\x06\x08*\x86H\xce=\x03\x01\x07\x03B\0\x04RT\x16\x16 \xef_\xd8\xe7\xc3\xb7\x03\x1d\xd6:\x1fF\xe3\xf2b!\xa9/\x8b\xd4\x82\x8f\xd1\xff[\x9c\x97\xbc\xf27\xb8,L\x8a\x01\xb0r;;\xa9\x83\xdc\x86\x9f\x1d%y\xf4;I\xe4Y\xc80'$K[\xd6\xa3\x82\x0bw0\x82\x0bs0\x82\x0bo\x06\t`\x86H\x01\x86\xf8B\x01\r\x04\x82\x0b`{\"id\":\"117077750682263877593646412006783680848\",\"timestamp\":\"2019-06-17T12:46:04.002066\",\"version\":3,\"isvEnclaveQuoteStatus\":\"GROUP_OUT_OF_DATE\",\"platformInfoBlob\":\"1602006504000900000909020401800000000000000000000008000009000000020000000000000B401A355B313FC939B4F48A54349C91\x03\x02\x03H\x000E\x02!\0\xae6\x06\t@Sy\x8f\x8ec\x9d\xdci^Ex*\x92}\xdcG\x15A\x97\xd7\xd7\xd1\xccx\xe0\x1e\x08\x02 \x15Q\xa0BT\xde'~\xec\xbd\x027\xd3\xd8\x83\xf7\xe6Z\xc5H\xb4D\xf7\xe2\r\xa7\xe4^f\x10\x85p";
const CERT_TOO_SHORT2: &[u8] = b"0\x82\x0c\x8c0";

#[test]
fn verify_ias_report_should_work() {
	let _signer_attn: [u32; 16] = Decode::decode(&mut TEST1_SIGNER_ATTN).unwrap();
	let report = verify_ias_report(TEST4_CERT);
	let report = report.unwrap();
	assert_eq!(report.mr_enclave, TEST4_MRENCLAVE);
	assert!(report.timestamp >= TEST1_TIMESTAMP.try_into().unwrap());
	assert_eq!(report.pubkey, TEST4_SIGNER_PUB);
	//assert_eq!(report.status, SgxStatus::GroupOutOfDate);
	assert_eq!(report.status, SgxStatus::ConfigurationNeeded);
	assert_eq!(report.build_mode, SgxBuildMode::Debug);
}

#[test]
fn verify_zero_length_cert_returns_err() {
	// CERT empty, argument 2 and 3 are wrong too!
	let _signer_attn: [u32; 16] = Decode::decode(&mut TEST1_SIGNER_ATTN).unwrap();
	assert!(verify_ias_report(&Vec::new()[..]).is_err())
}

#[test]
fn verify_wrong_cert_is_err() {
	// CERT wrong, argument 2 and 3 are wrong too!
	let _signer_attn: [u32; 16] = Decode::decode(&mut TEST1_SIGNER_ATTN).unwrap();
	assert!(verify_ias_report(CERT_WRONG_PLATFORM_BLOB).is_err())
}

#[test]
fn verify_wrong_fake_enclave_quote_is_err() {
	// quote wrong, argument 2 and 3 are wrong too!
	let _signer_attn: [u32; 16] = Decode::decode(&mut TEST1_SIGNER_ATTN).unwrap();
	assert!(verify_ias_report(CERT_FAKE_QUOTE_STATUS).is_err())
}

#[test]
fn verify_wrong_sig_is_err() {
	// sig wrong, argument 2 and 3 are wrong too!
	let _signer_attn: [u32; 16] = Decode::decode(&mut TEST1_SIGNER_ATTN).unwrap();
	assert!(verify_ias_report(CERT_WRONG_SIG).is_err())
}

#[test]
fn verify_short_cert_is_err() {
	let _signer_attn: [u32; 16] = Decode::decode(&mut TEST1_SIGNER_ATTN).unwrap();
	assert!(verify_ias_report(CERT_TOO_SHORT1).is_err());
	assert!(verify_ias_report(CERT_TOO_SHORT2).is_err());
}

#[test]
fn fix_incorrect_handling_of_iterator() {
	// In `verify_ias_report` we called `iter.next()` with unwrap three times, which could fail
	// for certain invalid reports as the one in this test. This test verifies that the issue
	// has been fixed.
	//
	// For context, see: https://github.com/integritee-network/pallet-teerex/issues/35

	let report: [u8; 56] = [
		224, 224, 224, 224, 224, 224, 224, 224, 235, 2, 0, 1, 5, 40, 0, 8, 255, 6, 8, 42, 134, 72,
		206, 61, 3, 1, 7, 0, 2, 183, 64, 48, 48, 0, 1, 10, 23, 3, 6, 9, 96, 134, 72, 1, 134, 248,
		66, 1, 13, 0, 0, 0, 13, 1, 14, 177,
	];

	assert_err!(verify_ias_report(&report), "Invalid netscape payload");
}

#[test]
fn verify_sgx_build_mode_works() {
	//verify report from enclave in debug mode
	let report = verify_ias_report(TEST4_CERT);
	let report = report.unwrap();
	assert_eq!(report.build_mode, SgxBuildMode::Debug);
	//verify report from enclave in production mode
	let report = verify_ias_report(TEST8_CERT);
	let report = report.unwrap();
	assert_eq!(report.build_mode, SgxBuildMode::Production);
}

#[test]
fn parse_enclave_metadata_works() {
	let report = verify_ias_report(TEST9_CERT).unwrap();
	assert_eq!(report.status, SgxStatus::ConfigurationAndSwHardeningNeeded);

	let spid = [220_u8, 102, 69, 18, 100, 146, 156, 242, 117, 174, 42, 236, 164, 33, 3, 41];
	let nonce = [200_u8, 253, 140, 194, 153, 34, 106, 241, 252, 37, 235, 176, 218, 195, 5, 87];
	let sig_rl = Vec::<u8>::new();
	let isv_enclave_quote = [
		2_u8, 0, 0, 0, 88, 12, 0, 0, 13, 0, 13, 0, 0, 0, 0, 0, 220, 102, 69, 18, 100, 146, 156,
		242, 117, 174, 42, 236, 164, 33, 3, 41, 92, 42, 81, 78, 112, 106, 91, 25, 241, 254, 78,
		190, 78, 66, 55, 204, 19, 19, 255, 255, 1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0,
		0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0, 0, 49, 141, 114, 177, 254, 227, 122, 120, 68, 218, 24,
		161, 8, 190, 114, 5, 97, 183, 231, 92, 66, 118, 224, 33, 106, 10, 7, 118, 15, 196, 33, 190,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 131, 215, 25, 231, 125, 234, 202, 20, 112, 246, 186, 246, 42, 77, 119, 67, 3, 200,
		153, 219, 105, 2, 15, 156, 112, 238, 29, 252, 8, 199, 206, 158, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 32, 235, 32, 225, 150, 102, 191, 132, 149, 65, 24, 111, 28, 102, 0, 165, 54, 94, 226,
		57, 146, 159, 195, 82, 253, 207, 229, 255, 122, 155, 54, 168, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
		0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 168, 2, 0, 0, 75, 24,
		193, 203, 163, 117, 77, 87, 240, 240, 126, 60, 112, 163, 75, 253, 210, 79, 29, 228, 136,
		150, 64, 248, 40, 33, 82, 188, 117, 221, 250, 202, 111, 23, 115, 238, 243, 39, 33, 184,
		211, 240, 72, 25, 65, 90, 203, 43, 237, 46, 159, 182, 120, 2, 249, 76, 95, 116, 56, 48,
		135, 208, 204, 50, 237, 243, 61, 30, 25, 206, 125, 146, 16, 43, 176, 135, 203, 55, 20, 193,
		205, 223, 14, 123, 194, 120, 143, 219, 235, 148, 16, 7, 248, 139, 216, 194, 110, 86, 211,
		223, 250, 34, 85, 157, 150, 193, 141, 125, 30, 186, 159, 74, 125, 11, 217, 209, 59, 208,
		68, 218, 171, 227, 26, 78, 141, 52, 233, 188, 28, 194, 2, 154, 212, 57, 180, 188, 209, 142,
		146, 189, 105, 63, 83, 91, 107, 38, 82, 24, 143, 115, 90, 43, 187, 172, 183, 197, 245, 204,
		93, 82, 130, 134, 89, 101, 249, 195, 60, 224, 249, 214, 101, 176, 98, 16, 159, 49, 28, 106,
		161, 191, 108, 183, 180, 16, 158, 171, 6, 140, 106, 30, 58, 186, 185, 120, 64, 227, 148,
		159, 214, 67, 227, 9, 26, 239, 23, 202, 105, 121, 172, 221, 47, 168, 243, 252, 218, 78,
		201, 241, 182, 134, 141, 193, 111, 196, 39, 230, 193, 245, 6, 133, 168, 174, 240, 227, 68,
		133, 218, 132, 93, 82, 93, 12, 146, 29, 230, 234, 203, 58, 76, 27, 216, 165, 159, 161, 161,
		45, 59, 49, 133, 19, 241, 196, 63, 104, 88, 251, 12, 104, 215, 195, 83, 33, 169, 203, 234,
		127, 64, 105, 141, 65, 108, 30, 40, 55, 183, 90, 39, 73, 140, 35, 11, 238, 91, 253, 25,
		165, 125, 189, 23, 243, 104, 1, 0, 0, 217, 4, 245, 212, 142, 218, 189, 11, 131, 186, 13,
		22, 247, 94, 25, 4, 86, 148, 6, 43, 180, 156, 175, 138, 115, 69, 132, 247, 74, 172, 183,
		175, 243, 50, 231, 72, 231, 249, 167, 162, 61, 87, 226, 110, 43, 63, 222, 131, 53, 126,
		124, 235, 40, 52, 186, 107, 195, 164, 17, 143, 60, 127, 80, 203, 162, 55, 69, 221, 82, 112,
		106, 27, 61, 34, 68, 18, 108, 176, 31, 88, 253, 90, 215, 39, 50, 207, 214, 112, 54, 235,
		57, 168, 26, 111, 34, 219, 205, 2, 136, 52, 230, 220, 49, 108, 236, 252, 156, 44, 21, 6,
		24, 39, 131, 106, 68, 115, 160, 47, 202, 95, 93, 52, 150, 233, 250, 186, 117, 183, 161, 89,
		218, 241, 25, 40, 224, 99, 132, 24, 166, 54, 167, 129, 161, 152, 77, 155, 7, 18, 55, 85,
		12, 177, 45, 170, 59, 95, 177, 255, 188, 19, 198, 202, 80, 85, 50, 123, 46, 212, 169, 120,
		25, 74, 100, 128, 159, 113, 103, 201, 218, 148, 87, 0, 104, 28, 42, 23, 138, 88, 220, 194,
		172, 84, 116, 242, 162, 176, 101, 176, 195, 249, 49, 224, 22, 55, 134, 185, 76, 220, 73,
		32, 197, 57, 74, 173, 159, 143, 44, 109, 100, 171, 47, 140, 130, 161, 121, 53, 101, 72,
		228, 192, 232, 150, 194, 213, 228, 128, 121, 119, 240, 23, 204, 1, 198, 53, 173, 140, 105,
		75, 83, 250, 59, 64, 186, 34, 79, 60, 85, 116, 227, 243, 75, 97, 230, 207, 147, 91, 8, 71,
		20, 172, 93, 30, 147, 223, 246, 220, 123, 77, 166, 1, 147, 119, 245, 217, 140, 61, 252,
		183, 56, 18, 52, 107, 252, 238, 55, 221, 202, 182, 171, 77, 93, 134, 77, 192, 184, 219, 29,
		61, 134, 137, 210, 113, 218, 211, 101, 39, 8, 176, 111, 108, 230, 204, 63, 30, 81, 150,
		189, 85, 190, 236, 47, 229, 119, 4, 177, 87, 105, 123, 141, 82, 94, 225, 231, 14, 120, 204,
		64, 104, 103, 60, 28, 70, 238, 20, 176, 60, 226, 187, 217, 98, 218, 248, 175, 245, 250, 65,
		137, 163, 128, 134, 204, 230, 46, 210, 63, 187,
	];

	let metadata = report.metadata;
	assert_eq!(metadata.quote_inputs.spid, spid);
	assert_eq!(metadata.quote_inputs.nonce, nonce);
	assert_eq!(metadata.quote_inputs.sig_rl, sig_rl);
	assert_eq!(metadata.isv_enclave_quote, isv_enclave_quote);
}
