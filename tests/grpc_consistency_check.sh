#!/bin/bash
# Some gRPC files in this repository are generated and based upon other
# sources. When these sources change, the generated files must be generated
# (and committed) again. It is the task of the contributing user to do this
# properly.
#
# This tests makes sure the generated gRPC files are consistent with its
# sources. If this fails, you probably have to generate the gRPC files again.
#
# Read more here: https://github.com/solvio/solvio/blob/master/docs/DEVELOPMENT.md#grpc

set -ex

# Ensure current path is project root
cd "$(dirname "$0")/../"

# Keep current version of file to check
cp ./docs/grpc/{,.diff.}docs.md

# Regenerate gRPC docs
./tools/generate_grpc_docs.sh

# Ensure generated files are the same as files in this repository
if diff -Zwa ./docs/grpc/{,.diff.}docs.md
then
    set +x
    echo "No diff found."
else
    set +x
    echo "ERROR: Generated gRPC file is not consistent with files in this repository, see diff above."
    echo "ERROR: See: https://github.com/solvio/solvio/blob/master/docs/DEVELOPMENT.md#grpc"
    exit 1
fi

# Cleanup
rm -f ./docs/grpc/.diff.docs.md
