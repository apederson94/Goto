#!/bin/zsh

function goto {
  __goto $1
  output_file=$(__goto -p)
  if [ $? -eq 0 ]; then
    cd $(cat $output_file)
  fi
}
