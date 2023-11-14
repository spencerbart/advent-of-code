#!/bin/bash

for i in {1..25}
do
   printf -v filename "d%02d.rs" $i
   touch "$filename"
   printf "use std::io::{Read, stdin};\n\npub fn p01() {\n\tlet mut input = String::new();\n\tstdin().read_to_string(&mut input).unwrap();\n}\n\npub fn p02() {\n\tlet mut input = String::new();\n\tstdin().read_to_string(&mut input).unwrap();\n}" >> ${filename}
done
