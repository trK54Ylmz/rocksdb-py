import os
import sys
from github import Github
from pathlib import Path

main = Path(__file__).parent.parent.parent
dest = main.joinpath('target').joinpath('wheels')

token = os.environ.get('GIT_TOKEN')
repository = os.environ.get('GIT_REPO')

g = Github(token)

r = g.get_user().get_repo(repository)

ts = list(r.get_tags())

if len(ts) == 0:
    print('There is no any tag.')

    sys.exit(1)

t = ts[-1]

rs = list(r.get_releases())
s = list(filter(lambda r: r.tag_name == t.name, rs))

if len(s) == 0:
    rl = r.create_git_release(t.name, t.name, t.commit.commit.message)
else:
    rl = s[0]

ss = list(map(lambda s: s.name, rl.get_assets()))

for d in dest.iterdir():
    if not d.name.endswith('.whl'):
        continue

    if d.name in ss:
        continue

    print(f'The file is {d.name}')

    p = str(dest.joinpath(d.name))

    rl.upload_asset(p)

sys.exit(0)
