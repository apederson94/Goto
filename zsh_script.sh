#!/bin/zsh

function goto {
  goto $1

  if [ $? -eq 0 ]; then
    cd $(cat /tmp/goto.loc)
  fi

}
