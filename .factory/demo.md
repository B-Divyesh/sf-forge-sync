# forge-sync demo sandbox

## Browser entry

Open `https://forge-sync.sociobot.in/?demo=1` or `/demo/`. The page presents
fictional Harbor Cooperative output and stores only a `demo:forge-sync:session`
marker in browser local storage. The persistent banner says that it is sample
data. **Reset demo** removes every `demo:forge-sync:` key and starts a new
marker. **Start for real** removes every demo key and returns to `/`. Keys
outside the demo prefix are never read, changed, or removed.

## CLI entry

Run `forge-sync demo` from any directory. The command never reads the current
directory, configuration, or token environment variables. It creates a newly
named directory below the operating system temporary directory and prints that
path. Delete the printed directory to remove the sample.

The shipped input records live in `examples/sample-mirror/`. The command uses
the same model rendering, SQLite state, audit, JSON archive, and Git commit
code as a real run. Output includes a target-style pull-request issue,
branches, tags, one source-to-target mapping, three audit events, and a local
archive commit.
