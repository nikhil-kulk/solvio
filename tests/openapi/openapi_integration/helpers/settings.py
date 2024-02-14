import os

import schemathesis

ROOT_DIR = os.path.dirname(__file__)
OPENAPI_FILE = os.environ.get("OPENAPI_FILE", os.path.join(os.path.dirname(ROOT_DIR), '../../../openapi', 'openapi-merged.yaml'))

SCHEMA = schemathesis.from_file(open(OPENAPI_FILE))
solvio_HOST = os.environ.get("solvio_HOST", "http://localhost:6333")
