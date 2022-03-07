import platform
import subprocess
import sys
from distutils import util
from pathlib import Path

system = platform.system()
main = Path(__file__).parent.parent.parent
dest = main.joinpath('target').joinpath('wheels')
platform = util.get_platform()
expected = sys.argv[1].split('-')[0]

print(f'The platform is {platform}')

for d in dest.iterdir():
    if not d.name.endswith('.whl'):
        continue

    if expected not in d.name:
        print(f'The file {d.name} is SKIPPED.')

        continue

    print(f'The file is {d.name}')

    args = ['pip', 'install', str(dest.joinpath(d.name))]

    try:
        res = subprocess.run(args, check=True, capture_output=True, cwd=str(main))

        print(res.stdout.decode('utf-8'))
        print(res.stderr.decode('utf-8'))
    except subprocess.CalledProcessError as e:
        print(e.output)
        print(e.stderr)
        print(e.stdout)

    sys.exit(0)

sys.exit(1)
