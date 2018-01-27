#!/bin/bash
#
# Script to package a .zip containing the artifacts
#

if [ $# -ne 2 ]; then
    echo "Usage: $0 <platform name> <version>"
    exit 1
fi

PLATFORM_NAME=$1
VERSION=$2

mkdir native-file-tests-$PLATFORM_NAME-$VERSION

for file in `ls ./build/*/*`; do
    base=`basename $file`
    sha1=`sha1sum $file | awk '{ print $1 }'`
    cp $file native-file-tests-$PLATFORM_NAME-$VERSION/$base.$sha1
done

zip -r native-file-tests-$PLATFORM_NAME-$VERSION.zip native-file-tests-$PLATFORM_NAME-$VERSION/ 2>&1
