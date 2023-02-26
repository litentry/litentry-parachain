FROM golang:1.18

RUN go install github.com/litentry/ChainBridge/cmd/chainbridge@minqi-dev
