import glob
from colorama import init, Fore, Style
import subprocess

regex = "tests/**/*.yl"
tests = glob.glob(regex, recursive=True)

init()

failed = 0
passed = 0

for test in tests:
    process = subprocess.run(["cargo", "run", "-p", "ytest", "--", f"-t={test}"], stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    if process.returncode == 0:
        print(Fore.GREEN + test + Style.RESET_ALL)
        passed += 1
    else:
        failed += 1
        print(Fore.RED + test + Style.RESET_ALL)

print(
    Fore.GREEN + str(passed) + Style.RESET_ALL + 
    " tests exited sucessfully and " + 
    Fore.RED + str(failed) + Style.RESET_ALL + 
    " failed"
)

if failed != 0:
    exit(-1)
else:
    exit(0)