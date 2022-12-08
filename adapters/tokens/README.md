# Node.js `Substream` Tokens Adapter

## Environment Variables

**.env**

```env
# Required
PACKAGE="../../substreams/tokens/tokens-v0.0.1.spkg"
MODULES=store_tokens
START_BLOCK_NUM=2
STOP_BLOCK_NUM=1001

# (Optional)
FIREHOSE_HOST=eos.firehose.eosnation.io:9001
API_TOKEN=""
```

## Quickstart

```bash
$ npm ci
$ npm start
```