# Sojourn

A lightning-quick journaling utility

```bash
Journaler.

If no date is specified, today will be used. Input can be piped in or added as
a positional argument in a string. Piped input takes precedence.

Arguments:
-d --date         desired date for entry
-b --base_dir     base directory to create files

Options
-e --edit         open entry in editor - uses \$EDITOR if available
-h --help         show this help and exit
-v --verbose      show verbose output

Usage

journal.sh                              View today's entry
journal.sh -e                           Edit today's entry
journal.sh "TIL something neat"         Add a note to today's entry
journal.sh "TIL something neat" -e      Add a note to today's entry and edit
journal.sh -d 2020-01-01                View the entry from 1/1/20
cat file.txt | journal.sh               Pipe file.txt into today's entry
```

## Recommended aliases

- View your current entry with `jj`
- Create or edit the current day's entry with `jj -e` which uses your `$EDITOR`

```bash
#alias js="sublime $(date '+~/repos/journal/%Y-%m-%d.md')" 
#alias jv="vim $(date '+~/repos/journal/%Y-%m-%d.md')" 
alias jj="~/repos/dotfiles/journal.sh"
alias j="jj -e"
alias jjmd="marked `jj -v | grep FILENAME | awk '{print $2}'`"

function jjw () {
    # A summary of the last 10 files
    ls ~/repos/journal | grep '20.*\.md' | sort -r | head -n 10 | awk '{print "/Users/taylor/repos/journal/"$1}' | xargs bat --plain
}
```
