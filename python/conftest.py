"""
conftest.py — Adds the python/ directory to sys.path so pytest and IDEs
can resolve ``input_validation_engine`` without a pip install.
"""

import sys
from pathlib import Path

# Insert the python/ directory (this file's parent) at the front of sys.path
# so `import input_validation_engine` always resolves correctly.
sys.path.insert(0, str(Path(__file__).parent))
