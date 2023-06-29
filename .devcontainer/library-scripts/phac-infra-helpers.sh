#!/usr/bin/env bash
get_ulid(){
    curl --silent https://ulid.truestamp.com/ | jq -r '.[] | .ulid' | awk '{print tolower($0)}'
}