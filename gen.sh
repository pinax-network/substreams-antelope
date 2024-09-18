#!/bin/bash

VERSION="v2.0.0-rc.7"
SPKG_URL="${ANTELOPE_SPKG:-https://github.com/pinax-network/firehose-antelope/releases/download/$VERSION/antelope-$VERSION.spkg}"

echo "Generating Antelope Protobuf using $ANTELOPE_SPKG"
substreams protogen $SPKG_URL --exclude-paths="sf/substreams,google/" --generate-mod-rs=false --show-generated-buf-gen=false --output-path="core/src/pb"
rm buf.gen.yaml
