package main

import (
	"crypto/aes"
	"crypto/cipher"
	"crypto/rand"
	"crypto/rsa"
	"crypto/sha256"
	"encoding/json"
	"flag"
	"fmt"
	"github.com/ethereum/go-ethereum/common/hexutil"
	"github.com/ethereum/go-ethereum/common/math"
	"github.com/ethereum/go-ethereum/crypto"
	"github.com/ethereum/go-ethereum/crypto/blake2b"
	"github.com/ethereum/go-ethereum/crypto/secp256k1"
	"github.com/itering/scale.go/source"
	"github.com/itering/scale.go/types/scaleBytes"
	"io"
	"io/ioutil"
	"math/big"
	"slices"
)
import "github.com/gorilla/websocket"
import "crypto/tls"
import "github.com/itering/scale.go/types"
import "github.com/itering/scale.go/utiles"

type response struct {
	Jsonrpc string `json:"jsonrpc"`
	Result  string `json:"result"`
	Id      int    `json:"id"`
}

type request struct {
	Jsonrpc string   `json:"jsonrpc"`
	Method  string   `json:"method"`
	Params  []string `json:"params"`
	Id      int      `json:"id"`
}

type rpcResult struct {
	Value    string                 `json:"value"`
	Do_watch bool                   `json:"do_watch"`
	Status   map[string]interface{} `json:"status"`
}

type aesOutput struct {
	Ciphertext string `json:"ciphertext"`
	Aad        string `json:"aad"`
	Nonce      string `json:"nonce"`
}

type Rsa3072PubKey struct {
	N [384]byte `json:"n"`
	E [4]byte   `json:"e"`
}

func main() {
	portPtr := flag.String("port", "2000", "worker's port number")
	flag.Parse()

	fmt.Println("port:", *portPtr)

	registerCustomTypes()
	c := create_conn(*portPtr)

	//** request shielding key
	requestAuthorGetShieldingKey(*c)
	res := read_response(*c)

	// ** decode response and parse shielding key
	getShieldingKeyResult, _ := decodeRpcReturnValue(res.Result)
	pubKey := parseShieldingKey(getShieldingKeyResult)

	//** request aggregated public key
	requestAggregatedPublicKey(*c)
	res = read_response(*c)

	aggregatedPubKeyResult, _ := decodeRpcReturnValue(res.Result)
	fmt.Println("Aggregated public key:")
	fmt.Println(utiles.HexToBytes(aggregatedPubKeyResult))

	//** request mrenclave
	requestStateGetMrenclave(*c)
	res = read_response(*c)
	// shard is also mrenclave
	getStateMrEnclaveResult, _ := decodeRpcReturnValue(res.Result)
	//at this point we got all stuff from worker - shielding key, mrenclave and shard (shard == mrenclave)

	//** WARNING: use this key only for environment without real value
	//public 0xffefbfc831e25a4dc6ece5c3600db669132a06ff8db152e3d7a1bbc0a3d425e596e708015b72266e0c6b7975662c794db43846c312ab58a678d9440a42cceba9
	//address 0x144Fa896B5FAbcA9D352483f0741776d1F836094
	key, _ := crypto.HexToECDSA("453134b1fda19819772d2fe7de3c2a8670f930e3187f2a81a509a52500e3a281")
	ethAddress := crypto.PubkeyToAddress(key.PublicKey).Bytes()

	fmt.Println("Eth address")
	fmt.Println(crypto.PubkeyToAddress(key.PublicKey))

	//** prepare identity (signer)
	identity := map[string]interface{}{
		"Evm": hexutil.Encode(ethAddress),
	}

	//** prepare SignEthereum direct call
	//generate random aes key :)
	aesKey := []byte("AES256Key-1234123412341234123412")
	prehashedEthereumMessage := []byte{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 64}

	//** prepare signed direct call
	directCall := prepareSignEthereumDirectCall(identity, aesKey, prehashedEthereumMessage)
	encodedDirectCall := types.Encode("DirectCall", directCall)

	encodedMrEnclave := types.Encode("[u8; 32]", getStateMrEnclaveResult)

	//prepare payload to sign
	payloadToSign := blake2b.Sum256(prepareDirectCallSignaturePayload(encodedDirectCall, encodedMrEnclave))

	payloadHash := crypto.Keccak256(payloadToSign[:])
	sig, _ := secp256k1.Sign(payloadHash, math.PaddedBigBytes(key.D, key.Params().BitSize/8))

	directCallSigned := prepareSignedDirectCall(directCall, sig)
	encodedDirectCallSigned := types.Encode("DirectCallSigned", directCallSigned)

	//** aes encrypt signed direct call
	ciphertext, aad, nonce := aesEncrypt(aesKey, encodedDirectCallSigned)
	aesOutput := prepareAesOutputObject(ciphertext, aad, nonce)
	encryptedAesKey := encryptAesKeyWithShieldingKey(aesKey, pubKey)

	//** create AesRequest
	aesRequest := map[string]interface{}{
		"shard":   getStateMrEnclaveResult,
		"key":     hexutil.Encode(encryptedAesKey),
		"payload": aesOutput,
	}
	encodedAesRequest := types.Encode("AesRequest", aesRequest)

	// ** create rpc request with hex encoded scale encoded aes request
	signRequest := request{
		Jsonrpc: "2.0",
		Method:  "bitacross_submitRequest",
		Params:  []string{encodedAesRequest},
		Id:      1,
	}
	serializedRequest, srErr := json.Marshal(signRequest)

	if srErr != nil {
		fmt.Println("Problem while serializing the request")
		fmt.Println(srErr)
	}

	sendRequest(*c, serializedRequest)

	// ** decode response and parse shielding key, status 0 means success

	signResp := read_response(*c)

	signResult, signStatus := decodeRpcReturnValue(signResp.Result)
	resultAesOutput := decodeAesOutput(signResult)
	decryptedResult := aesDecrypt(aesKey, utiles.HexToBytes(resultAesOutput.Ciphertext), utiles.HexToBytes(resultAesOutput.Nonce), utiles.HexToBytes(resultAesOutput.Aad))

	fmt.Println("Decrypted result")
	fmt.Println(decryptedResult)

	if _, ok := signStatus["Error"]; ok {
		signBitcoinError := decodeSignEthereumError(decryptedResult)
		fmt.Println("Got SignEthereumError")
		fmt.Println(signBitcoinError)
	} else {
		signature := decryptedResult
		fmt.Println("Got signature:")
		fmt.Println(signature)
	}
}

func prepareAesOutputObject(cipher []byte, aad []byte, nonce []byte) map[string]interface{} {
	return map[string]interface{}{
		"ciphertext": hexutil.Encode(cipher),
		"aad":        hexutil.Encode(aad),
		"nonce":      hexutil.Encode(nonce),
	}
}

func encryptAesKeyWithShieldingKey(aesKey []byte, shieldingKey Rsa3072PubKey) []byte {
	nBytes := shieldingKey.N[:]
	eBytes := shieldingKey.E[:]
	slices.Reverse(nBytes)
	slices.Reverse(eBytes)
	rsaPubKey := rsa.PublicKey{
		N: big.NewInt(1).SetBytes(nBytes),
		E: int(big.NewInt(0).SetBytes(eBytes).Uint64()),
	}

	encryptedAesKey, encryptAesError := rsa.EncryptOAEP(sha256.New(), rand.Reader, &rsaPubKey, aesKey, nil)
	if encryptAesError != nil {
		fmt.Println("encrypt aes key error")
		fmt.Println(encryptAesError)
	}
	return encryptedAesKey
}

func aesEncrypt(aesKey []byte, encodedCall string) ([]byte, []byte, []byte) {
	block, err := aes.NewCipher(aesKey)
	if err != nil {
		panic(err.Error())
	}

	nonce := make([]byte, 12)
	if _, err := io.ReadFull(rand.Reader, nonce); err != nil {
		panic(err.Error())
	}

	aesgcm, err := cipher.NewGCM(block)
	if err != nil {
		panic(err.Error())
	}

	additionalData := []byte{}
	return aesgcm.Seal(nil, nonce, utiles.HexToBytes(encodedCall), additionalData), additionalData, nonce
}

func aesDecrypt(aesKey []byte, ciphertext []byte, nonce []byte, aad []byte) []byte {
	block, err := aes.NewCipher(aesKey)
	if err != nil {
		panic(err.Error())
	}

	aesgcm, err := cipher.NewGCM(block)

	decrypted, _ := aesgcm.Open(nil, nonce, ciphertext, aad)
	return decrypted
}

func prepareDirectCallSignaturePayload(directCallScaleEncoded string, mrEnclaveScaleEncoded string) []byte {
	enclaveAppended := append(utiles.HexToBytes(directCallScaleEncoded), utiles.HexToBytes(mrEnclaveScaleEncoded)...)
	shardAppended := append(enclaveAppended, utiles.HexToBytes(mrEnclaveScaleEncoded)...)
	return shardAppended
}

func prepareSignedDirectCall(directCall map[string]interface{}, signature []byte) map[string]interface{} {
	return map[string]interface{}{
		"call": directCall,
		"signature": map[string]interface{}{
			"Ethereum": map[string]interface{}{
				"col1": hexutil.Encode(signature),
			},
		},
	}
}

func prepareSignEthereumDirectCall(identity map[string]interface{}, aesKey []byte, prehashedEthereumMessage []byte) map[string]interface{} {
	signEthereumDirectCall := map[string]interface{}{
		"col1": identity,
		"col2": utiles.BytesToHex(aesKey),
		"col3": utiles.BytesToHex(prehashedEthereumMessage),
	}

	return map[string]interface{}{
		"SignEthereum": signEthereumDirectCall,
	}

}

func prepareSignBitcoinTaprootSpendableDirectCall(identity map[string]interface{}, aesKey []byte, bitcoinPayload []byte, merkleRootHash [32]byte) map[string]interface{} {
	payload := map[string]interface{}{
		"TaprootSpendable": map[string]interface{}{
			"col1": string(bitcoinPayload),
			"col2": utiles.BytesToHex(merkleRootHash[:]),
		},
	}

	signBitcoinDirectCall := map[string]interface{}{
		"col1": identity,
		"col2": utiles.BytesToHex(aesKey),
		"col3": payload,
	}

	return map[string]interface{}{
		"SignBitcoin": signBitcoinDirectCall,
	}
}

func prepareSignBitcoinWithTweakDirectCall(identity map[string]interface{}, aesKey []byte, bitcoinPayload []byte, tweakBytes [32]byte, tweakIsXOnly bool) map[string]interface{} {
	tweaks := []map[string]interface{}{
		map[string]interface{}{
			"col1": utiles.BytesToHex(tweakBytes[:]),
			"col2": tweakIsXOnly,
		},
	}

	payload := map[string]interface{}{
		"WithTweaks": map[string]interface{}{
			"col1": string(bitcoinPayload),
			"col2": tweaks,
		},
	}

	directCall := map[string]interface{}{
		"col1": identity,
		"col2": utiles.BytesToHex(aesKey),
		"col3": payload,
	}

	return map[string]interface{}{
		"SignBitcoin": directCall,
	}
}

func prepareSignBitcoinTaprootUnspendableDirectCall(identity map[string]interface{}, aesKey []byte, bitcoinPayload []byte) map[string]interface{} {
	payload := map[string]interface{}{
		"TaprootUnspendable": string(bitcoinPayload),
	}

	signBitcoinDirectCall := map[string]interface{}{
		"col1": identity,
		"col2": utiles.BytesToHex(aesKey),
		"col3": payload,
	}

	return map[string]interface{}{
		"SignBitcoin": signBitcoinDirectCall,
	}

}

func prepareSignBitcoinDerivedDirectCall(identity map[string]interface{}, aesKey []byte, bitcoinPayload []byte) map[string]interface{} {
	payload := map[string]interface{}{
		"Derived": string(bitcoinPayload),
	}

	signBitcoinDirectCall := map[string]interface{}{
		"col1": identity,
		"col2": utiles.BytesToHex(aesKey),
		"col3": payload,
	}

	return map[string]interface{}{
		"SignBitcoin": signBitcoinDirectCall,
	}

}

func parseShieldingKey(hexEncodedShieldingKey string) Rsa3072PubKey {
	var pubKey Rsa3072PubKey
	keyBytes := utiles.HexToBytes(hexEncodedShieldingKey)
	//we need to strip first two bytes - I don't know why
	err := json.Unmarshal(keyBytes[2:len(keyBytes)], &pubKey)
	if err != nil {
		fmt.Println("error unmarshaling")
		fmt.Println(err)
	}
	return pubKey
}

func requestAuthorGetShieldingKey(c websocket.Conn) {
	err := c.WriteMessage(websocket.TextMessage, []byte("{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"author_getShieldingKey\",\"params\":[]}"))
	if err != nil {
		fmt.Println("Error sending message")
		fmt.Println(err)
	}
}

func requestAggregatedPublicKey(c websocket.Conn) {
	err := c.WriteMessage(websocket.TextMessage, []byte("{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"bitacross_aggregatedPublicKey\",\"params\":[]}"))
	if err != nil {
		fmt.Println("Error sending message")
		fmt.Println(err)
	}
}

func sendRequest(c websocket.Conn, request []byte) {
	err := c.WriteMessage(websocket.TextMessage, request)
	if err != nil {
		fmt.Println("Error sending message")
		fmt.Println(err)
	}
}

func requestStateGetMrenclave(c websocket.Conn) {
	err := c.WriteMessage(websocket.TextMessage, []byte("{\"id\":1,\"jsonrpc\":\"2.0\",\"method\":\"state_getMrenclave\",\"params\":[]}"))
	if err != nil {
		fmt.Println("Error sending message")
		fmt.Println(err)
	}
}

func decodeRpcReturnValue(hexEncoded string) (string, map[string]interface{}) {
	bytes := scaleBytes.ScaleBytes{Data: utiles.HexToBytes(hexEncoded)}
	m := types.ScaleDecoder{}
	m.Init(bytes, nil)
	var rpcResult rpcResult
	err := utiles.UnmarshalAny(m.ProcessAndUpdateData("RpcReturnValue").(interface{}), &rpcResult)

	if err != nil {
		fmt.Println("Unmarshall error!")
		fmt.Println(err)
	}
	return rpcResult.Value, rpcResult.Status
}

func decodeAesOutput(hexEncoded string) aesOutput {
	bytes := scaleBytes.ScaleBytes{Data: utiles.HexToBytes(hexEncoded)}
	m := types.ScaleDecoder{}
	m.Init(bytes, nil)
	var output aesOutput
	err := utiles.UnmarshalAny(m.ProcessAndUpdateData("AesOutput").(interface{}), &output)

	if err != nil {
		fmt.Println("Unmarshall error!")
		fmt.Println(err)
	}
	return output
}

func decodeSignBitcoinError(encoded []byte) map[string]interface{} {
	bytes := scaleBytes.ScaleBytes{Data: encoded}
	m := types.ScaleDecoder{}
	m.Init(bytes, &types.ScaleDecoderOption{
		SubType: "string,string",
	})
	var output map[string]interface{}
	err := utiles.UnmarshalAny(m.ProcessAndUpdateData("SignBitcoinError").(interface{}), &output)

	if err != nil {
		fmt.Println("Unmarshall error!")
		fmt.Println(err)
	}
	return output
}

func decodeSignEthereumError(encoded []byte) map[string]interface{} {
	bytes := scaleBytes.ScaleBytes{Data: encoded}
	m := types.ScaleDecoder{}
	m.Init(bytes, &types.ScaleDecoderOption{
		SubType: "string,string",
	})
	var output map[string]interface{}
	err := utiles.UnmarshalAny(m.ProcessAndUpdateData("SignEthereumError").(interface{}), &output)

	if err != nil {
		fmt.Println("Unmarshall error!")
		fmt.Println(err)
	}
	return output
}

func read_response(c websocket.Conn) response {
	_, message, r_err := c.ReadMessage()
	if r_err != nil {
		fmt.Println("Error reading message")
		fmt.Println(r_err)
	}

	res := response{}
	if err := json.Unmarshal(message, &res); err != nil {
		panic(err)
	}
	return res
}

func create_conn(port string) *websocket.Conn {

	dialer := *websocket.DefaultDialer
	url := "ws://localhost:" + port
	fmt.Println("Connecting to worker:")
	fmt.Println(url)

	// this is not secure, use with caution
	dialer.TLSClientConfig = &tls.Config{InsecureSkipVerify: true}
	c, _, err := dialer.Dial(url, nil)
	if err != nil {
		fmt.Println("Could not connect to worker")
		fmt.Println(err)
	}
	fmt.Println("connected to worker")
	return c
}

func registerCustomTypes() {
	def, read_err := ioutil.ReadFile("definitions.json")
	if read_err != nil {
		fmt.Println("Error while reading definitions file")
		fmt.Println(read_err)
	}
	types.RegCustomTypes(source.LoadTypeRegistry(def))
	types.TypeRegistry["[u8; 4]"] = &types.FixedU8{FixedLength: 4}
	types.TypeRegistry["[u8; 12]"] = &types.FixedU8{FixedLength: 12}
	types.TypeRegistry["[u8; 32]"] = &types.FixedU8{FixedLength: 32}
	types.TypeRegistry["[u8; 20]"] = &types.FixedU8{FixedLength: 20}
	types.TypeRegistry["[u8; 65]"] = &types.FixedU8{FixedLength: 65}
}
