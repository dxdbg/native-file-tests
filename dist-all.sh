#!/bin/bash
#
# A script to package all platform artifacts into a single package
#

if [ $# -ne 1 ]; then
    echo "Usage: $0 version"
    exit 1
fi

VERSION=$1
BASE_URL="https://github.com/udidb/native-file-tests/releases/download/$VERSION/native-file-tests"

URLS=($BASE_URL-macos-$VERSION.zip $BASE_URL-linux-$VERSION.zip)

for url in ${URLS[@]}; do
    curl -sSfL $url || {
        echo "Failed to download $url"
        exit 1
    }

    file_name=$(basename $url)

    unzip -j $file_name -d native-file-tests || {
        echo "Failed to extract $file_name"
        exit 1
    }
done

zip -r native-file-tests-$VERSION.zip native-file-tests/
