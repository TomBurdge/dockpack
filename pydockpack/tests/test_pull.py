from pydockpack import pull_docker_image
import os

def test_pull():
    image = "maxwellflitton/nan-one"
    directory = "./cache/two"
    path = pull_docker_image(image, directory)
    # assert is used in the cargo test, but not needed in python land... - would raise exception
    assert os.path.isdir(path)
