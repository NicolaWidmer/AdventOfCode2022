#!/bin/bash

cargo new day"$1"
touch ./day"$1"/src/in.txt
touch ./day"$1"/src/test.txt
rm ./day"$1"/src/main.rs
cp ./template.rs ./day"$1"/src/main.rs