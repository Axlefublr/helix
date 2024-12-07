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

    set -l prs 11234:reloads/Swordelf2 \
                9483:kakoune_split/scdailey \
               10576:anchors/pantos9000 \
                5749:scrollbar/jakesactualface \
               13053:buffer_search/oxcrow \
                9843:indent_textobj/EpocSquadron \
               13847:execute/nik-rev \
               14121:move_lines/omentic \
               14120:overtype/413x1nkp

    for pr in $prs
        set -l thingy (string split ':' $pr)
        set -l pr_id $thingy[1]
        set -l branch_name $thingy[2]
        git fetch upstream pull/$pr_id/head:$branch_name
        git checkout $branch_name
        git reset --hard FETCH_HEAD
        git switch {{ branch }}
        git commit --allow-empty -m "merge $branch_name ($pr_id)"
    end

    git switch {{ branch }}
