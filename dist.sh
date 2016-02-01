#!/bin/bash
#
# Script to populate the dist directory from a build
#

rm -rf dist
mkdir dist

for file in `ls ./build/src/*`; do
    basefilename=`basename $file`
    base=`echo $basefilename | awk -F- '{ print $1 }'`
    if [ "$basefilename" != "${basefilename##*.}" ]; then
        base="$base.${basefilename##*.}"
    fi

    sha1=`sha1sum $file | awk '{ print $1 }'`
    cp $file dist/$base.$sha1
done
