#!/bin/bash

VERSION="v0.0.5"
ANTELOPE_SPKG="${ANTELOPE_SPKG:-https://github.com/pinax-network/firehose-antelope/releases/download/$VERSION/antelope-$VERSION.spkg}"

echo "Generating Antelope Protobuf using $ANTELOPE_SPKG"
wget $ANTELOPE_SPKG
substreams protogen "antelope-$VERSION.spkg" --exclude-paths="sf/substreams/v1,google/"
