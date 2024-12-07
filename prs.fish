#!/usr/bin/env fish

set -l prs 11234:Swordelf2/reloads \
    9483:scdailey/kakoune_split \
    12241:NikitaRevenco/messages \
    12208:NikitaRevenco/hover_buffer \
    9843:EpocSquadron/indent_textobj \
    12308:nik-rev/color_swatches \
    9143:intarga/persistent_state

for pr in $prs
    set -l thingy (string split ':' $pr)
    set -l pr_id $thingy[1]
    set -l branch_name $thingy[2]
    git fetch upstream pull/$pr_id/head:$branch_name
    git checkout $branch_name
    git reset --hard FETCH_HEAD
    if set -q argv[1]
        git switch master
        git commit --allow-empty -m "merge $pr_id: $branch_name"
    end
end

git switch master
