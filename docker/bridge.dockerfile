FROM golang:1.18

RUN git clone --depth 1  --branch dev  https://github.com/litentry/ChainBridge.git \
    && cd ChainBridge \
    && make build
