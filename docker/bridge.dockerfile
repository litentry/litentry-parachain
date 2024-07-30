FROM golang:1.18

RUN go install github.com/litentry/ChainBridge/cmd/chainbridge@sol0.8.19
