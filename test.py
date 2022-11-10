import os

project_dir = os.path.dirname(os.path.realpath(__file__))
result = os.system("cargo r --bin gen -- " + project_dir +
                   "/crates/tests/src/integration")
if result is 0:
    os.system("cargo t")
else:
    print("Error: Test generation exit status = ", result)
