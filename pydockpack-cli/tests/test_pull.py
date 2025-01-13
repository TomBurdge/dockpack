import os
import pytest
import pathlib
import subprocess


@pytest.fixture
def cache_path():
    file_path = pathlib.Path("./cache/")
    yield file_path
    # shutil.rmtree(file_path)


def test_pull(cache_path):
    image = "maxwellflitton/nan-one"
    command = "pull"
    directory = os.path.join(str(cache_path), command)
    # subprocess, to not replace the current process
    _completed_process = subprocess.run(["dockpack", command,"-i", image, "-d", directory])
    # assert is used in the cargo test, but not needed in python land... - would raise exception
    assert os.path.isdir(directory)
