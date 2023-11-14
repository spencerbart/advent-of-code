#!/bin/bash

for i in {1..25}
do
   printf -v filename "input_%02d.txt" $i
   touch "$filename"
done
