#!/bin/bash

sudo uv pip install --system --break-system-packages pre-commit

pre-commit run
# pre-commit install
