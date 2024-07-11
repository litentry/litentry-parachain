#!/bin/bash

# Copyright 2020-2024 Trust Computing GmbH.

while getopts ":p:A:B:u:W:V:C:" opt; do
    case $opt in
        p)
            NPORT=$OPTARG
            ;;
        A)
            WORKER1PORT=$OPTARG
            ;;
        u)
            NODEURL=$OPTARG
            ;;
        V)
            WORKER1URL=$OPTARG
            ;;
        C)
            CLIENT_BIN=$OPTARG
            ;;
    esac
done

# Using default port if none given as arguments.
NPORT=${NPORT:-9944}
NODEURL=${NODEURL:-"ws://127.0.0.1"}

WORKER1PORT=${WORKER1PORT:-2000}
WORKER1URL=${WORKER1URL:-"ws://127.0.0.1"}

CLIENT_BIN=${CLIENT_BIN:-"./../bin/litentry-cli"}

echo "Using client binary $CLIENT_BIN"
echo "Using node uri $NODEURL:$NPORT"
echo "Using trusted-worker uri $WORKER1URL:$WORKER1PORT"
echo ""

CLIENT="$CLIENT_BIN -p $NPORT -P $WORKER1PORT -u $NODEURL -U $WORKER1URL"
echo "CLIENT is: $CLIENT"

evm_vec=(
    "0x4e9Cc05F7F944D618DE85396e669922c2CF6813E"
    "0x390127D12612391313077c6B8016FE2BefE0Eb20"
    "0x73648d08A36a595267161453d76576b64AcF42D3"
    "0x4675C7e5BaAFBFFbca748158bEcBA61ef3b0a263"
    "0xCe91228789B57DEb45e66Ca10Ff648385fE7093b"
    "0x388C818CA8B9251b393131C08a736A67ccB19297"
    "0x34cb778eA0B3e386a16616EC643f06E900BdEa26"
    "0x88AA83547A5A647EfA4c41991BEfC87705Bd71D2"
    "0x80A45f3eCEDEe79E8EF7560E4A3d8823A7866971"
    "0x515fE77C5FFB9E763BB0f6d15BC0058a50316361"
    "0xE94f1fa4F27D9d288FFeA234bB62E1fBC086CA0c"
    "0x6d2e03b7EfFEae98BD302A9F836D0d6Ab0002766"
    "0x11bCE4536296C81d1A291B1FFbe292FDd55a3A89"
    "0x8E4D28D5890ce63f835fFbFCC11f5daD3a326F88"
    "0x52749C7bcf4c5a6B8DDa943EB61186A21A80aA16"
    "0xa41ab7e565eD20C9666193db510a9C8ad08Ad080"
    "0x8fcaec55743ccDFad794c28D6BFf2B218919DaDF"
    "0xFf5e8F55d236B45Ef38b734767b25f213B2D0824"
    "0xeAB4dc477B95C343D1AD00034F43eAEE99379F13"
    "0x75e89d5979E4f6Fba9F97c104c2F0AFB3F1dcB88"
    "0xa7Ca2C8673bcFA5a26d8ceeC2887f2CC2b0Db22A"
    "0x28C6c06298d514Db089934071355E5743bf21d60"
    "0x4e35EB1462deF3Eb9347390A1506596Be4F1A360"
    "0x0e076eE390eC4a1c75B2AdcaB6CC39211F5947Ac"
    "0x343C19897186a6F114c1De5637e776B33fCB60dF"
    "0x2a785850bf6455e00359fBae0dA83E9A3a3aD44A"
    "0x366e70960DbAC148b778224683FFf77f02D7FCfe"
    "0x95e63F78Bd2Fd5ea8a0D58182f3d998558881FDA"
    "0xF31a49D00164c425D80434F110aafB803E30217F"
    "0x4a88301e9AA5DA7bA5302CeF5652C2f46116a5a3"
    "0x688b3F0E2b6e50d081f7D2f09248053E81E330d6"
    "0xb0162310bb380acA7D2b1CaF9F19694D3612c223"
    "0x26F697cFE9BaB4B0De8b9e13A6cd0f6DCE8FaFb8"
    "0x0363dBbc047815AE3e32543245d28546Fa4eF6D6"
    "0xaB782bc7D4a2b306825de5a7730034F8F63ee1bC"
    "0x0F7cB31C9adE61455cb5A4c053B576544a9141Cd"
    "0x9507850543C3F0661c4D89996B4FBd346c0F32b4"
    "0xCe0de738F636e911051583412f924A77D6aDDB7b"
    "0x4f82c3519961C1BaF9d6688c95302562fbc755f2"
    "0xbfAC0847E6EB3425CE75b9CA295438b39988Add5"
    "0x0d3714DFb4CAD3133709eDf5474E7f3b37102665"
    "0x5Ba11Fa7e47399e21101Ef5cdCbdf281f476e91b"
    "0x58edF78281334335EfFa23101bBe3371b6a36A51"
    "0xb68b989B298A470C96d2D3E3C2f7Fd8353775dF8"
    "0x4c4cba9baCC4926a902828a2eE4CD1CB53FB5F40"
    "0x125f660239707C9De3462D3fa633F2723aD0b884"
    "0xF0F42DE747f797A06AD77F1c1DcAb862Ac260715"
    "0x95222290DD7278Aa3Ddd389Cc1E1d165CC4BAfe5"
    "0x4838B106FCe9647Bdf1E7877BF73cE8B0BAD5f97"
    "0x3Bee5122E2a2FbE11287aAfb0cB918e22aBB5436"
    "0x1f9090aaE28b8a3dCeaDf281B0F12828e676c326"
    "0x199D5ED7F45F4eE35960cF22EAde2076e95B253F"
)
sub_vec=(
    "0xd077e0ae84e39eabadc60ffcaf6c721d7e2f27d5cc3e49d6ed4f7aa5d8ed8f1b"
    "0x1ae96e2bb1efefb74634741dc023331aaefed0459bae846aa0c929e9062cba3c"
    "0xdeb8f64e46c9dc7235515ef2acc0a27f0e8681a23e3ec159a0a0895aec7fbb46"
    "0x646314264d2d3801d72121e3fda840c86b0ef1d6c294b1acba554fbc876fbf3e"
    "0x9f7f960da4af057a9f8190e76b322b5de7a06bd37defff9f668eaec432f27425"
    "0xb3a5306e430e8a634864a7828d833d3fd17180611056e83732e88dd701190b3a"
    "0x97ab033c2d9cdc2f4a038cbfdf5652837a86692d058b2fc1a5e28c238ea3307b"
    "0x1e83d56498d4431df9182aea22c7ecb183565a4a6fd1617aef2712b6cf13b9cd"
    "0x78e324908e5bb66ce3b087f1d0f4b4f2812857aa978a133261cf92db71d487cf"
    "0x58099c071fd9b147372d04e0e84718a88c3e674a70e92ca22fc9b10f19a7db3e"
    "0xe8e2262e16583379847ad70b7f77ca559d5c17aa69062230f8a0dbd1bf5da5d4"
    "0x6c1b752375304917c15af9c2e7a4426b3af513054d89f6c7bb26cd7e30e4413e"
    "0x28c5c756b5da7978e45b746ab840e8ac27df108750d5e2cbaf310232d3489421"
    "0x9c3e6fda7870bcc2ffa7ffe63aa71522fc18be9e96517541b5af492c7910f0f0"
    "0x0bd1177d190c955fac5de6a176769fb1b3237c47c3a22a8bff2451a39979634d"
    "0x44b5c1a27d85b0c3a2f8fe5f6721e893204032fd7f63d62ad7f20ddbb9f76e10"
    "0x5acfa87d804864af77a4878649813d602073eec57bcf26e23598201c464f166c"
    "0x40fa5d5923e5becbfa4a8c23acb24327e08913fca5a2faa13ed12919b185de33"
    "0xd614bc9eef81a44e8b629877b51b447cf9fb2343d4527e08ac75d7ea5eae4221"
    "0x950e74ffcaadedcc5b8bc89d11e1d5630636110a17985242e646c3a2d1adc795"
    "0x0658111801107f1c3774182a6ed3ec5535a77107590a799af7a0adb1d92593c2"
    "0x429b067ff314c1fed75e57fcf00a6a4ff8611268e75917b5744ac8c4e1810d17"
    "0x84054b7ba3f01b81bab709d303c4d46a78de37377a2f5d78fbba89e1d4743801"
    "0x3b35bbee99b3486b060e46a55564344f8f9056e90b8db48318ef8158da393c45"
    "0x44e8937bee393b6a3f2d26bb21b6a6f7978f14a486c81fb70f0fd442167ddd06"
    "0xdc9b8a32839c812e9eeb323c8e6e3f884b1e559851c9e2382b2be2deb947245a"
)

if [ "${#evm_vec[@]}" -ne "$((2 * ${#sub_vec[@]}))" ]; then
    echo "Error: Length of evm_vec should be twice the length of sub_vec"
    exit 1
fi
timestamp1=$(date +%s%N)
for ((i = 0; i < ${#sub_vec[@]}; i++)); do
    sub_item=${sub_vec[i]}
    evm1=${evm_vec[$((i*2))]}
    evm2=${evm_vec[$((i*2 + 1))]}

    echo "link '$evm1' and '$evm2' to main account '$sub_item'"
    OUTPUT=$(${CLIENT} trusted -d link-identity did:litentry:substrate:$sub_item did:litentry:evm:$evm1 bsc,ethereum) || { echo "Link identity command failed"; exit 1; }
    OUTPUT=$(${CLIENT} trusted -d link-identity did:litentry:substrate:$sub_item did:litentry:evm:$evm2 bsc,ethereum) || { echo "Link identity command failed"; exit 1; }
done
timestamp2=$(date +%s%N)
echo "Link identities succeed! Elapsed time: $(( ($timestamp2 - $timestamp1) )) ns"

assertion_vec=(
    "a1"
    "a4 0"
    "a6"
    "a7 0"
    "a8 litentry,litmus"
    "a10 0"
    "a11 0"
    "a14"
    "a20"
    "bnb-domain-holding"
    "one-block completion"
    "one-block outstanding"
    "one-block participation"
    "generic-discord-role contest legend"
    "generic-discord-role contest popularity"
    "generic-discord-role contest participant"
    "generic-discord-role sora-quiz attendee"
    "generic-discord-role sora-quiz master"
    "bnb-digital-domain-club bnb999-club-member"
    "bnb-digital-domain-club bnb10k-club-member"
    "vip3-membership-card gold"
    "vip3-membership-card silver"
    "weirdo-ghost-gang-holder"
    "evm-amount-holding ton"
    "evm-amount-holding trx"
    "crypto-summary"
    "brc20-amount-holder"
    "lit-staking"
    "token-holding-amount bnb"
    "token-holding-amount eth"
    "token-holding-amount space-id"
    "token-holding-amount lit"
    "token-holding-amount wbtc"
    "token-holding-amount usdc"
    "token-holding-amount usdt"
    "token-holding-amount crv"
    "token-holding-amount matic"
    "token-holding-amount dydx"
    "token-holding-amount amp"
    "token-holding-amount cvx"
    "token-holding-amount tusd"
    "token-holding-amount usdd"
    "token-holding-amount gusd"
    "token-holding-amount link"
    "token-holding-amount grt"
    "token-holding-amount comp"
    "token-holding-amount people"
    "token-holding-amount gtc"
    "token-holding-amount ton"
    "token-holding-amount trx"
    "token-holding-amount nfp"
    "token-holding-amount sol"
    "platform-user karat-dao-user"
    "nft-holder weirdo-ghost-gang"
    "nft-holder club3-sbt"
    "nft-holder mfan"
)

assertions=()
for assertion in "${assertion_vec[@]}"; do
    assertions+=("-a"  "$assertion")
done

echo "Start request VC"
timestamp3=$(date +%s%N)
for ((i = 0; i < ${#sub_vec[@]}; i++)); do
    {
        sub_item=${sub_vec[i]}
        REQUEST_OUTPUT=$(${CLIENT} trusted -d request-vc did:litentry:substrate:$sub_item "${assertions[@]}") || { echo "Request batch vc command failed"; exit 1; }
        echo -e "************************************************* Request VC for main account '$sub_item': $REQUEST_OUTPUT ================================================="
    } &
done
wait

timestamp4=$(date +%s%N)
echo ""
echo "In total ${#sub_vec[@]} substrate main accounts, linked in total ${#evm_vec[@]} evm accounts. Each of main accounts requested ${#assertion_vec[@]} assertions."
echo "Request VC finished. Elapsed time: $(( ($timestamp4 - $timestamp3) )) ns"

