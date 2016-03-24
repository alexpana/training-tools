#!/bin/bash

template_dir=~/src/training/tools
name=$1
solution_dir=`pwd`
output_template=$solution_dir/$1.rs

cp -u $template_dir/rust_template.rs $solution_dir/$1.rs

echo "Template '$1.rs' copied. Good luck!"
