#!/bin/sh
set -euo pipefail

DIR=`dirname $0`
cd ${DIR}

cargo run --package tuple-regen