#!/bin/zsh

function goto {
  goto-rs $1

  if [ $? -eq 0 ]; then
    cd $(cat /tmp/goto.loc)
  fi

}
