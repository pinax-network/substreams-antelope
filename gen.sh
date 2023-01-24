#!/bin/bash

VERSION="v0.0.4"
ANTELOPE_SPKG="${ANTELOPE_SPKG:-https://github.com/pinax-network/firehose-antelope/releases/download/$VERSION/antelope-$VERSION.spkg}"

echo "Generating Antelope Protobuf using $ANTELOPE_SPKG"
substreams protogen "$ANTELOPE_SPKG" --exclude-paths="sf/substreams/v1,google/" --output-path="./src/pb"
