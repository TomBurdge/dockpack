from pydockpack import pull_docker_image
import os
import pytest
import pathlib
import shutil


@pytest.fixture
def cache_path():
    file_path = pathlib.Path("./cache/")
    yield file_path
    shutil.rmtree(file_path)


def test_pull(cache_path):
    image = "maxwellflitton/nan-one"
    directory = str(cache_path)
    path = pull_docker_image(image, directory)
    # assert is used in the cargo test, but not needed in python land... - would raise exception
    assert os.path.isdir(path)
