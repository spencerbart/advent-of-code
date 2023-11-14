#!/bin/bash

for i in {1..25}
do
   printf -v filename "d%02d.rs" $i
   touch "$filename"
done
