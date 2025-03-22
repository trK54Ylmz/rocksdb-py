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

rs = sorted(list(r.get_releases()), key=lambda rls: rls.created_at, reverse=True)
rl = rs[0]

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
