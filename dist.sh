#!/bin/bash
#
# Script to populate the dist directory from a build
#

rm -rf dist
mkdir dist

for file in `ls ./build/src/*`; do
    base=`basename $file`
    sha1=`sha1sum $file | awk '{ print $1 }'`
    cp $file dist/$base.$sha1
done
