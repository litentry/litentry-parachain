FROM golang:1.18

RUN go install github.com/collab-ai-network/ChainBridge/cmd/chainbridge@sol0.8.19
