#!/bin/bash

DIR=`dirname $0`

if [[ $# -eq 0 && $# -gt 2 ]]; then
  echo "usage: $0 <day> [input-only]"
  echo "input-only: should be 'true' if desired"
  exit
fi

num=$1
inputOnly=$2
year=$(date +%Y)

dayPath=$DIR/$year/day$num

if [[ $inputOnly != 'true' ]]; then
  cp -R $DIR/dayX $dayPath
  sed -i "s/dayX/day${num}/g" $dayPath/{Cargo.toml,Cargo.lock,pom.xml}
  sed -i "s/DayX/Day${num}/g" $dayPath/src/main/java/ag/aoc/DayX.java
  mv $dayPath/src/main/java/ag/aoc/DayX.java $dayPath/src/main/java/ag/aoc/Day${num}.java
  touch $dayPath/sample.txt
fi
AOC_COOKIE=`cat ~/aoc-cookie`
curl -b "session=$AOC_COOKIE" https://adventofcode.com/$year/day/${num}/input > $dayPath/input.txt
echo "Input head:"
head $dayPath/input.txt
