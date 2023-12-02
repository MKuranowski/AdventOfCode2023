#!/bin/sh
DAY="${1:?No day provided}"
DAY_NUMBER_ONLY=$(echo "$DAY" | sed 's/[a-z]//')

if [ "x$2" = "xtest" ]
then
    SUFFIX=".test.txt"
else
    SUFFIX=".txt"
fi

if [ -e "input/${DAY}${SUFFIX}" ]
then
    INPUT="input/${DAY}${SUFFIX}"
else
    INPUT="input/${DAY_NUMBER_ONLY}${SUFFIX}"
fi

cargo run --release --bin "$DAY" <"$INPUT"
