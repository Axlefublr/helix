set shell := ['fish', '-c']

alias fi := fancy-install
alias b := build

build:
    cargo fmt
    cargo clippy
    cargo test
    cargo xtask docgen
    notify-send 'finished building'

fancy-install:
    pueue add -- 'cargo install --path helix-term --locked'

install:
    cargo install --path helix-term --locked

prs branch='master':
    #!/usr/bin/env fish
    test "$(git status --porcelain)" = '' || return 1

    set -l prs 11234:Swordelf2/reloads \
        9483:scdailey/kakoune_split \
        10576:pantos9000/anchors \
        5749:jakesactualface/scrollbar \
        13053:oxcrow/buffer_search \
        9843:EpocSquadron/indent_textobj \
        13847:nik-rev/execute

    for pr in $prs
        set -l thingy (string split ':' $pr)
        set -l pr_id $thingy[1]
        set -l branch_name $thingy[2]
        git fetch upstream pull/$pr_id/head:$branch_name
        git checkout $branch_name
        git reset --hard FETCH_HEAD
        git switch {{ branch }}
        git commit --allow-empty -m "merge $pr_id: $branch_name"
    end

    git switch {{ branch }}
