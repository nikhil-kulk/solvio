#!/usr/bin/env sh
# Script to run solvio in docker container and handle contingencies, like OOM.
# The functioning logic is as follows:
# - If recovery mode is allowed, we check if solvio was killed during initialization or not.
#   - If it was killed during initialization, we remove run solvio in recovery mode
#   - If it was killed after initialization, do nothing and restart container
# - If recovery mode is not allowed, we just restart container

./solvio $@

EXIT_CODE=$?

solvio_ALLOW_RECOVERY_MODE=${solvio_ALLOW_RECOVERY_MODE:-false}

# Check that recovery mode is allowed
if [ "$solvio_ALLOW_RECOVERY_MODE" != true ]; then
    exit $EXIT_CODE
fi

# Check that solvio was killed (exit code 137)
# Ideally, we want to catch only OOM, but it's not possible to distinguish it from random kill signal
if [ $EXIT_CODE != 137 ]; then
    exit $EXIT_CODE
fi

IS_INITIALIZED_FILE='.solvio-initialized'
RECOVERY_MESSAGE="Solvio was killed during initialization. Most likely it's Out-of-Memory.
Please check memory consumption, increase memory limit or remove some collections and restart"

# Check that solvio was initialized
# Solvio creates IS_INITIALIZED_FILE file after initialization
# So if it doesn't exist, solvio was killed during initialization
if [ ! -f "$IS_INITIALIZED_FILE" ]; then
    # Run solvio in recovery mode.
    # No collection operations are allowed in recovery mode except for removing collections
    solvio__STORAGE__RECOVERY_MODE="$RECOVERY_MESSAGE" ./solvio $@
    exit $?
fi

exit $EXIT_CODE
