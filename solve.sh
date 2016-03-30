#!/bin/bash

# the directory where the templates are
template_dir="$0"
if [ -L "$template_dir" ]; then
  template_dir=`readlink -f "$template_dir"`
fi
template_dir=`dirname "$template_dir"`

# the solution name argument
name=$1

# the solution output directory
solution_dir=`pwd`

# the solution output path
output_file=$solution_dir/$1.rs

# check if the name is empty
if [ -z $name ]; then
  echo "usage: solve solution_name"
  echo "  solution_name     The name of the output solution file."
  exit 1
fi

# check if a solution file already exists
if [ -e "$output_file" ]; then
  echo "Solution already exists, you can resume working!"
else
  cp -n $template_dir/rust_template.rs $output_file
  echo "Solution '$1.rs' created. Happy coding!"
fi
