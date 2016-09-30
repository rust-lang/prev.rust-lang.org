git grep -l $'^\xEF\xBB\xBF' .

if [ $? -eq 0 ]; then exit 1; else exit 0; fi
