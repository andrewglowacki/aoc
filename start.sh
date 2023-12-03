#!/bin/bash

DIR=`dirname $0`

if [[ $# -ne 1 ]]; then
  echo "usage: $0 <day>"
  exit
fi

num=$1
cp -R $DIR/dayX day$num
sed -i "s/dayX/day${num}/g" day${num}/{Cargo.toml,Cargo.lock,pom.xml}
sed -i "s/DayX/Day${num}/g" day${num}/src/main/java/ag/aoc/DayX.java
mv day${num}/src/main/java/ag/aoc/DayX.java day${num}/src/main/java/ag/aoc/Day${num}.java
AOC_COOKIE=`cat ~/aoc-cookie`
curl -b "session=$AOC_COOKIE" https://adventofcode.com/2023/day/${num}/input > day${num}/input.txt
