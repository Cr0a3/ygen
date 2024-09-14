from base64 import decode
import glob
from colorama import init, Fore, Style
import subprocess

regex = "tests/**/*.yl"
tests = glob.glob(regex, recursive=True)

init()

failed = 0
passed = 0

for test in tests:
    process = subprocess.run(["cargo", "run", "-p", "ytest", "--", f"-t={test}"], capture_output=True)
    if process.returncode == 0:
        print(Fore.GREEN + "SUCESS " + test + Style.RESET_ALL)
        passed += 1
    else:
        failed += 1
        print(Fore.RED + "==== FAILED " + test + " ====" + Style.RESET_ALL)
        print(process.stdout.decode())
        print(process.stdout.decode())

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