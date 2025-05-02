import os
import shutil
import subprocess
from pathlib import Path
import readline

def complete_path(text, state):
    # expand ~ and environment vars
    text_expanded = os.path.expanduser(os.path.expandvars(text))
    # if it's a directory so far, list inside it
    if os.path.isdir(text_expanded):
        dirpath = text_expanded
        prefix = ''
    else:
        dirpath = os.path.dirname(text_expanded) or '.'
        prefix = os.path.basename(text_expanded)

    try:
        entries = os.listdir(dirpath)
    except OSError:
        entries = []

    matches = []
    for entry in entries:
        if entry.startswith(prefix):
            full = os.path.join(dirpath, entry)
            # append slash for dirs so you can keep tabbing
            display = entry + (os.path.sep if os.path.isdir(full) else '')
            # reconstruct what’s shown to user
            if dirpath in ('.', ''):
                matches.append(display)
            else:
                # preserve user's original form (with ~ or absolute)
                base = text[:len(text) - len(prefix)]
                matches.append(base + display)
    return matches[state] if state < len(matches) else None

# register and bind for both libedit and GNU readline
readline.set_completer(complete_path)
for bind in ("tab: complete", "bind ^I rl_complete"):
    try:
        readline.parse_and_bind(bind)
    except Exception:
        pass

LANG_EXECUTABLES = {
    'python': ['python3', 'python'],
    'node': ['node'],
    'java': ['javac'],
    'ruby': ['ruby'],
    'go': ['go'],
    'php': ['php'],
    'swift': ['swiftc'],
    'rust': ['rustc'],
}


JS_FRAMEWORKS = {'react', 'angular', 'vue', 'ember', 'svelte'}
DB_TYPES = {'sqlite', 'postgres', 'mysql'}
ML_LIBS = {'tensorflow', 'torch', 'scikit-learn'}

# common ignore chunks
_COMMON = [
    ".vscode/",
    ".DS_Store",
    "*.log",
    ".env",
    ".env.*",
]

# per‑lang specifics
_PYTHON = [
    "__pycache__/",
    ".pytest_cache/",
    ".mypy_cache/",
    ".venv/",
    "build/",
    "dist/",
    "*.egg-info/",
]

_NODE = [
    "node_modules/",
    ".npm/",
    ".nvm/",
    "dist/",
    "build/",
    ".cache/",
]

_JAVA = [
    "target/",
    "*.class",
    "*.jar",
    "*.war",
    "*.ear",
    ".settings/",
    ".project",
    ".classpath",
]

_RUBY = [
    ".bundle/",
    "log/",
    "tmp/",
    "vendor/bundle/",
]

_GO = [
    "bin/",
    "pkg/",
    "vendor/",
]

_RUST = [
    "target/",
    "Cargo.lock",
]

# assemble only for LANG_EXECUTABLES keys
GITIGNORE_TEMPLATES = {
    "python": _PYTHON + _COMMON,
    "node": _NODE + _COMMON,
    "java": _JAVA + _COMMON,
    "ruby": _RUBY + _COMMON,
    "go": _GO + _COMMON,
    "php": [] + _COMMON + ["vendor/"],
    "swift": [] + _COMMON + ["DerivedData/","*.xcodeproj/","*.xcworkspace/"],
    "rust": _RUST + _COMMON,
}

# render as sorted, deduped strings
GITIGNORE_TEMPLATES = {
    lang: "\n".join(sorted(set(lines))) + "\n"
    for lang, lines in GITIGNORE_TEMPLATES.items()
}


def is_installed(cmds):
    """
    Return the first installed command from the list, or None if none found.
    """
    for cmd in cmds:
        if shutil.which(cmd):
            return cmd
    return None


def setup_pyenv_env(base: Path, comp: str):
    """
    Create a pyenv virtualenv named <global-version>-<project_component>.
    """
    pyenv = shutil.which('pyenv')
    if not pyenv:
        return None
    try:
        version = subprocess.check_output(['pyenv', 'global'], stderr=subprocess.DEVNULL).decode().strip()
    except subprocess.CalledProcessError:
        return None
    env_name = f"{base.name}-{comp}"
    print(f"→ pyenv virtualenv {version} {env_name}")
    subprocess.run(['pyenv', 'virtualenv', version, env_name])
    comp_path = base / comp
    print(f"→ pyenv local {env_name} in {comp_path}")
    # Ensure directory exists for local setting
    comp_path.mkdir(parents=True, exist_ok=True)
    subprocess.run(['pyenv', 'local', env_name], cwd=str(comp_path))
    env = os.environ.copy()
    env['PYENV_VERSION'] = env_name
    return env


def scaffold_django(base: Path, comp: str):
    """
    Scaffold a Django project:
    - Create project with django-admin startproject <project_name>
    - Rename outer folder to <comp>
    - Inner project folder stays <project_name>
    """
    project_name = base.name
    py = is_installed(LANG_EXECUTABLES['python'])
    if not py:
        print("✖ Python missing, skipping Django.")
        return
    if not shutil.which('django-admin'):
        ans = input("Django not found. Install now? (y/N): ").strip().lower()
        if ans == 'y':
            print(f"→ {py} -m pip install django")
            subprocess.run([py, '-m', 'pip', 'install', 'django'])
        else:
            print("Skipping Django scaffold.")
            return
    # create temp project folder
    print(f"→ django-admin startproject {project_name}")
    subprocess.run(['django-admin', 'startproject', project_name], cwd=str(base))
    # rename temp folder to component name
    src = base / project_name
    dst = base / comp
    if dst.exists():
        print(f"✖ Target folder {dst} exists, cannot rename Django project.")
        return
    print(f"→ rename {src} to {dst}")
    src.rename(dst)
    # setup pyenv/env for comp
    env = setup_pyenv_env(base, comp)
    print(f"Scaffolded Django in {dst}")

def scaffold_flask_fastapi(base: Path, comp: str, tech: str):
    py = is_installed(LANG_EXECUTABLES['python'])
    if not py:
        print(f"✖ Python missing, skip {tech}.")
        return
    comp_path = base / comp
    comp_path.mkdir(exist_ok=True)
    # use pyenv env or fallback to venv
    env = setup_pyenv_env(base, comp)
    if not env:
        venv_dir = comp_path / 'venv'
        print(f"→ {py} -m venv {venv_dir}")
        subprocess.run([py, '-m', 'venv', str(venv_dir)])
        env = os.environ.copy()
        env['VIRTUAL_ENV'] = str(venv_dir)
        env['PATH'] = f"{venv_dir / 'bin'}:{env['PATH']}"
    pkg = ['flask'] if tech == 'flask' else ['fastapi', 'uvicorn']
    for package in pkg:
        result = subprocess.run([py, '-m', 'pip', 'show', package], env=env, stdout=subprocess.DEVNULL)
        if result.returncode != 0:
            ans = input(f"Package '{package}' missing. Install now? (Y/n): ").strip().lower()
            if ans in ('', 'y'):
                print(f"→ {py} -m pip install {package}")
                subprocess.run([py, '-m', 'pip', 'install', package], env=env)
            else:
                print(f"Skipping {package}.")
    stub = comp_path / ('app.py' if tech == 'flask' else 'main.py')
    stub.write_text({
        'flask': """from flask import Flask
app = Flask(__name__)
@app.route('/')
def home(): return 'Hello, Flask!'""",
        'fastapi': """from fastapi import FastAPI
app = FastAPI()
@app.get('/')
async def read_root(): return {'msg':'Hello, FastAPI!'}"""
    }[tech])
    print(f"→ stub {stub.name} written")


def scaffold_ml_lib(base: Path, comp: str, tech: str):
    py = is_installed(LANG_EXECUTABLES['python'])
    if not py:
        print(f"✖ Python missing, skip {tech}.")
        return
    comp_path = base / comp
    comp_path.mkdir(exist_ok=True)
    env = setup_pyenv_env(base, comp)
    if not env:
        venv_dir = comp_path / 'venv'
        print(f"→ {py} -m venv {venv_dir}")
        subprocess.run([py, '-m', 'venv', str(venv_dir)])
        env = os.environ.copy()
        env['VIRTUAL_ENV'] = str(venv_dir)
        env['PATH'] = f"{venv_dir / 'bin'}:{env['PATH']}"
    result = subprocess.run([py, '-m', 'pip', 'show', tech], env=env, stdout=subprocess.DEVNULL)
    if result.returncode != 0:
        ans = input(f"Package '{tech}' missing. Install now? (Y/n): ").strip().lower()
        if ans in ('', 'y'):
            print(f"→ {py} -m pip install {tech}")
            subprocess.run([py, '-m', 'pip', 'install', tech], env=env)
        else:
            print(f"Skipping {tech}.")
    nb = comp_path / 'notebook.ipynb'
    nb.write_text('{"cells":[],"metadata":{},"nbformat":4,"nbformat_minor":2}')
    print(f"→ empty notebook {nb.name} created")


def scaffold_rust(base: Path, comp: str):
    print(f"→ cargo new {comp}")
    subprocess.run(['cargo', 'new', comp], cwd=str(base))


def scaffold_js_framework(base: Path, comp: str, tech: str):
    node = is_installed(LANG_EXECUTABLES['node'])
    if not node:
        print(f"✖ Node missing, skip {tech}.")
        return
    template_map = {
        'react': 'react-ts',
        'vue': 'vue',
        'svelte': 'svelte'
    }
    template = template_map.get(tech)
    print(f"→ npm create vite@latest {comp} -- --template {template}")
    subprocess.run(['npm', 'create', 'vite@latest', comp, '--', '--template', template], cwd=str(base))


def scaffold_node_init(base: Path, comp: str):
    node = is_installed(LANG_EXECUTABLES['node'])
    if not node:
        print("✖ Node missing, skip node init.")
        return
    comp_path = base / comp
    comp_path.mkdir(exist_ok=True)
    print(f"→ npm init -y in {comp}")
    subprocess.run(['npm', 'init', '-y'], cwd=str(comp_path))


def scaffold_plain_folder(base: Path, comp: str, tech: str):
    folder = base / f"{comp}_{tech}"
    folder.mkdir(parents=True, exist_ok=True)
    print(f"→ created folder {folder.name}")


def scaffold_component(base: Path, comp: str, tech: str):
    tech_key = tech.lower().strip()
    if tech_key == 'django':
        scaffold_django(base, comp)
    elif tech_key in {'flask', 'fastapi'}:
        scaffold_flask_fastapi(base, comp, tech_key)
    elif tech_key in ML_LIBS:
        scaffold_ml_lib(base, comp, tech_key)
    elif tech_key == 'rust':
        scaffold_rust(base, comp)
    elif tech_key in {'react', 'vue', 'svelte'}:
        scaffold_js_framework(base, comp, tech_key)
    elif tech_key == 'node':
        scaffold_node_init(base, comp)
    else:
        scaffold_plain_folder(base, comp, tech_key)


def scaffold_db(base: Path, db: str):
    db = db.lower().strip()
    if db == 'sqlite':
        (base / 'db.sqlite3').touch()
        print("→ Created SQLite file db.sqlite3")
    elif db in {'postgres', 'mysql'}:
        dc = base / 'docker-compose.yml'
        image = 'postgres:14' if db == 'postgres' else 'mysql:8'
        dc.write_text(f"""
version: '3.8'
services:
  db:
    image: {image}
    environment:
      - POSTGRES_USER=app
      - POSTGRES_PASSWORD=secret
      - POSTGRES_DB={base.name}
    ports:
      - "5432:5432"
""".strip())
        print(f"→ Generated docker-compose.yml for {db}")

def prompt_clone(proj_dir):
    ans = input("Clone remote git repo into this folder? (y/N): ").strip().lower()
    if ans == 'y':
        repo = input("Repository URL: ").strip()
        print(f"→ git clone {repo} {proj_dir}")
        subprocess.run(['git', 'clone', repo, str(proj_dir)])
        return True
    return False

def gather_components(proj_dir):
    techs = []
    while True:
        comp = input("Component (frontend/backend/etc; blank to finish): ").strip()
        if not comp:
            break
        tech = input(f"Tech for '{comp}': ").strip().lower()
        techs.append(tech)
        # simple installer check
        if tech in JS_FRAMEWORKS:
            if not is_installed(LANG_EXECUTABLES['node']):
                print("✖ Node missing → can't scaffold JS.")
                continue
        elif tech not in ML_LIBS.union({'django','flask','fastapi','pyramid','rust'}):
            if not is_installed(LANG_EXECUTABLES.get(tech, [tech])):
                if not tech.lower().strip() in ['html', 'css','js',""]:
                    print(f"✖ '{tech}' not installed, skip.")

                os.mkdir(proj_dir/comp)
                continue
        scaffold_component(proj_dir, comp, tech)
    return techs

def prompt_db(proj_dir):
    db = input(f"Add DB ({'/'.join(DB_TYPES)}; blank to skip): ").strip().lower()
    if db in DB_TYPES:
        scaffold_db(proj_dir, db)

def write_gitignore(proj_dir, techs):
    ignore_lines = ["\n".join(_COMMON).strip()]
    for t in set(techs):
        tpl = GITIGNORE_TEMPLATES.get(t)
        if not tpl and t in JS_FRAMEWORKS:
            tpl = GITIGNORE_TEMPLATES["node"]
        elif not tpl and t in {'django','flask','fastapi','pyramid'}:
            tpl = GITIGNORE_TEMPLATES["python"]
        if tpl:
            ignore_lines.append(tpl.strip())
    path = proj_dir / '.gitignore'
    with open(path, 'w') as f:
        f.write("\n\n".join(ignore_lines).strip() + "\n")
    sections = set(techs) | {"common"}
    if any(f in techs for f in JS_FRAMEWORKS):
        sections.add("node")
    if any(f in techs for f in {'django','flask','fastapi','pyramid'}):
        sections.add("python")
    print(f"→ Created .gitignore based on: {', '.join(sorted(sections))}")

def write_readme(proj_dir, project, techs):
    path = proj_dir / 'README.md'
    with open(path, 'w') as f:
        f.write(f"# {project}\n\nTechnologies used: {', '.join(techs)}\n")
    print("→ Created README.md")

def init_git(proj_dir, project, techs):
    ans = input("Initialize a new git repository at project root? (Y/n): ").strip().lower()
    if ans in ('', 'y'):
        write_gitignore(proj_dir, techs)
        write_readme(proj_dir, project, techs)
        subprocess.run(['git', 'init'], cwd=proj_dir)
        subprocess.run(['git', 'add', '.'], cwd=proj_dir)
        subprocess.run(['git', 'commit', '-m', 'Initial commit'], cwd=proj_dir)
        print("Initialized git repo at root.")

def main():
    project = input("Project name: ").strip()
    if not project:
        print("Project name can’t be empty.")
        return

    # accept both absolute and relative paths
    base_input = input("Base path (default cwd): ").strip() or os.getcwd()
    base = Path(base_input).expanduser().resolve()
    proj_dir = base / project

    if prompt_clone(proj_dir):
        return

    proj_dir.mkdir(parents=True, exist_ok=True)
    print(f"Created project dir at {proj_dir}")

    techs = gather_components(proj_dir)
    prompt_db(proj_dir)
    init_git(proj_dir, project, techs)

    print("🎉 Done scaffolding.")



if __name__ == "__main__":
    main()
