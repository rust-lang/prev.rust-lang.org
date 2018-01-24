#!/usr/bin/env bash

# ###############################################################
# The purpose of this script is to make sure all the websites
# provided by "friends of Rust" are up and running. Meaning they
# do not respond with 5** status code.
# ###############################################################

readonly FRIENDS_FILE="../_data/users.yml";

# https://stackoverflow.com/questions/5014632/how-can-i-parse-a-yaml-file-from-a-linux-shell-script
function parse_yaml {
   local s fs
   s='[[:space:]]*' w='[a-zA-Z0-9_]*'
   fs=$(echo @|tr @ '\034')
   sed -ne "s|^\($s\):|\1|" \
        -e "s|^\($s\)\($w\)$s:$s[\"']\(.*\)[\"']$s\$|\1$fs\2$fs\3|p" \
        -e "s|^\($s\)\($w\)$s:$s\(.*\)$s\$|\1$fs\2$fs\3|p"  $1 |
   awk -F"$fs" '{
      indent = length($1)/2;
      vname[indent] = $2;
      for (i in vname) {if (i > indent) {delete vname[i]}}
      if (length($3) > 0) {
         vn=""; for (i=0; i<indent; i++) {vn=(vn)(vname[i])("_")}
         if ($2 == "url") {
             printf("%s,", $3);
         }
      }
   }'
}

readonly URLS=$(parse_yaml "$FRIENDS_FILE");

# https://stackoverflow.com/questions/918886/how-do-i-split-a-string-on-a-delimiter-in-bash
IFS=',' read -ra URL_ARRAY <<< "$URLS"

for url in "${URL_ARRAY[@]}"; do
    status_code=$(
        curl "$url" \
            -L \
            --write-out %"{http_code}" \
            --silent \
            --output /dev/null \
    );

    if [ "${status_code:0:1}" == "4" ]; then
        echo "WARNING: URL [" "$url" "] responded with status code [" "$status_code" "], continuing";
    elif [ "${status_code:0:1}" == "5" ]; then
        echo "ERROR: URL [" "$url" "] responded with status code [" "$status_code" "], aborting";
        exit 1;
    else
        echo OK!;
    fi
done

echo "All user websites are up and running";
