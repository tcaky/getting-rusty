#!/usr/bin/env bash

# insert to bashrc
rcFile="~/.bashrc"

source ~/phac-infra-helpers.sh

if grep -q "^source ~/phac-infra-helpers.sh" "$rcFile"; then
  # sed -i "s/^export $prop=.*$/export $prop=$val/" "$rcFile" &&
  # echo "[updated] export $prop=$val"
else
  echo -e "source ~/phac-infra-helpers.sh" >> "$rcFile"
  echo "[inserted] source ~/phac-infra-helpers.sh"
fi

# delete from bashrc
# rcFile="~/.bashrc"

# prop="POSTGRE_PORT"    # export property to delete

# if grep -q "^export $prop=" "$rcFile"; then
#   sed -i "/^export $prop=.*$/d" "$rcFile" &&
#   echo "[deleted] export $prop"
# else
#   echo "[not found] export $prop"
# fi
